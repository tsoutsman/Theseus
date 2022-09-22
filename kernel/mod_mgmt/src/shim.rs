#![deny(unsafe_op_in_unsafe_fn)]

use crate::*;
use core::{arch::global_asm, ffi::CStr};
use memory::get_kernel_mmi_ref;
use xmas_elf::{
    sections::Rela,
    symbol_table::{Entry, Entry64},
};

impl CrateNamespace {
    pub(crate) fn relocate_text_to_shim(
        &self,
        new_crate_ref: &StrongCrateRef,
        relocations: &[Rela<u64>],
        symbol_table: &[Entry64],
        elf_file: &ElfFile,
    ) -> Result<(), &'static str> {
        let mut new_crate = new_crate_ref.lock_as_mut().unwrap();

        let mapped_pages = Arc::new(Mutex::new(MappedPages::empty()));

        let mut text_shim = Vec::new();
        // Map of shim function names to indexes in relocations vec. We store the index
        // rather than cloning the Arc because that would prevent us from using
        // Arc::get_mut.
        let mut shim_sections = HashMap::new();

        let relocations = relocations
            .iter()
            .map(|relocation| {
                let dependency_entry = &symbol_table[relocation.get_symbol_table_index() as usize];

                let dependency_index = dependency_entry.shndx() as usize;
                let dependency_name = dependency_entry.get_name(&elf_file).unwrap();
                let dependency_value = dependency_entry.value() as usize;

                let section = match new_crate.sections.get(&dependency_index) {
                    // This is a relocation within the same crate.
                    Some(ss) => Arc::clone(ss),
                    // This is a relocation to some other crate.
                    None => {
                        let dependency_name = demangle(dependency_name).to_string();

                        // If the symbol is already loaded there's no need to create shims.
                        if let Some(sec) = self.get_symbol_internal(&dependency_name) {
                            sec.upgrade().unwrap()
                        } else {
                            // If a shim has already been generated for this dependency
                            if let Some(section) = shim_sections.get(&dependency_name) {
                                Arc::clone(&section)
                            } else {
                                let sec_shim = generate_function_shim(&dependency_name);

                                let section = Arc::new(LoadedSection {
                                    name: StrRef::from(dependency_name.as_ref()),
                                    typ: SectionType::Text,
                                    global: true,
                                    mapped_pages: Arc::clone(&mapped_pages),
                                    mapped_pages_offset: text_shim.len(),
                                    address_range: VirtualAddress::new_canonical(text_shim.len())
                                        ..VirtualAddress::new_canonical(
                                            text_shim.len() + sec_shim.len(),
                                        ),
                                    parent_crate: CowArc::downgrade(new_crate_ref),
                                    inner: Default::default(),
                                });

                                text_shim.extend(sec_shim);
                                shim_sections.insert(dependency_name, Arc::clone(&section));

                                section
                            }
                        }
                    }
                };

                (relocation, section)
            })
            .collect::<Vec<_>>();

        if text_shim.len() == 0 {
            log::info!("{} did not require shims", new_crate.crate_name);
            assert_eq!(shim_sections.len(), 0);
            return Ok(());
        }

        let mut mapped_pages =
            memory::create_mapping(text_shim.len(), TEXT_SECTION_FLAGS | EntryFlags::WRITABLE)?;
        mapped_pages.as_slice_mut(0, text_shim.len())?.clone_from_slice(&text_shim);

        let start = mapped_pages.start_address();

        let mapped_pages = Arc::new(Mutex::new(mapped_pages));

        // FIXME
        let mut shndx = 10000;

        // Now that we have allocated mapped pages for the shim, we can fill out the
        // shim section address ranges and mapped pages.
        for (_, mut sec) in shim_sections.iter_mut() {
            let address_range = (start + sec.address_range.start)..(start + sec.address_range.end);

            // SAFETY: The other Arcs are located in the relocations vec which is not being
            // accessed.
            let sec_mut = unsafe { Arc::get_mut_unchecked(&mut sec) };
            sec_mut.address_range = address_range;
            sec_mut.mapped_pages = Arc::clone(&mapped_pages);
            drop(sec_mut);

            new_crate.sections.insert(shndx, Arc::clone(sec));
            new_crate.global_sections.insert(shndx);

            shndx += 1;
        }

        for (relocation, dependency_section) in relocations {
            let dependency_entry = &symbol_table[relocation.get_symbol_table_index() as usize];
            let dependency_sec_value = dependency_entry.value() as usize;

            write_relocation(
                RelocationEntry::from_elf_relocation(relocation),
                todo!(),
                todo!(),
                dependency_section.start_address() + dependency_sec_value,
                false,
            )?;
        }

        // for (relocation, section, address) in relocations {
        //     let entry = &symbol_table[relocation.get_symbol_table_index() as usize];
        //     let dependency_address = if address == VirtualAddress::zero() {
        //         let name = entry.get_name(&elf_file).unwrap();
        //         let name = demangle(name).to_string();
        //         shim_sections.get(&name).unwrap().start_address()
        //     } else {
        //         address
        //     };

        //     write_relocation(
        //         RelocationEntry::from_elf_relocation(relocation),
        //         &mut section.mapped_pages.lock(),
        //         section.mapped_pages_offset,
        //         dependency_address,
        //         false,
        //     )?;

        //     // let source_index = source_entry.shndx() as usize;

        //     // if new_crate.sections.get(&source_index).is_none() {
        //     //     // TODO
        //     // }
        // }

        Ok(())
    }

    pub(crate) fn post_shim_load() {}
}

fn generate_function_shim(function_name: &str) -> Vec<u8> {
    // XXX: These "functions" are not functions, they are references to the labels
    // defined at the bottom of this file. There should be a way to do this using
    // the asm! macro, but I can't figure it out and this ain't too bad.
    extern "C" {
        fn __shim_func_start();
        fn __shim_func_end();
    }

    let shim_func_start = __shim_func_start as usize;
    let shim_func_end = __shim_func_end as usize;
    let shim_func_template_len = shim_func_end - shim_func_start;

    let shim_func_template = unsafe {
        core::slice::from_raw_parts(shim_func_start as *const u8, shim_func_template_len)
    };

    let mut shim = Vec::with_capacity(shim_func_template_len + function_name.as_bytes().len() + 1);

    shim.extend(shim_func_template);
    shim.extend(function_name.as_bytes());
    shim.push(0);

    shim
}

#[no_mangle]
#[doc(hidden)]
pub unsafe extern "C" fn __rewrite_shim_relocations_rust(
    call_instruction_in_shim: usize,
    shim_caller: usize,
) -> usize {
    log::error!(
        "---------------------------------------------- __rewrite_shim_relocations_rust ----------------------------------------------"
    );
    log::error!("call instruction in shim: {call_instruction_in_shim}");
    log::error!("shim caller: {shim_caller}");

    panic!();

    // XXX: Same as above.
    extern "C" {
        fn __shim_call_rewrite();
        fn __shim_func_end();
    }

    let call = __shim_call_rewrite as usize;
    let end = __shim_func_end as usize;

    let offset = end - call;
    let string_ptr = (call_instruction_in_shim + offset) as *const i8;

    let name = unsafe { CStr::from_ptr(string_ptr) }
        .to_str()
        .expect("function name contained invalid utf-8");

    let kernel_namespace = get_initial_kernel_namespace().unwrap();
    let kernel_mmi_ref = get_kernel_mmi_ref().unwrap();
    let symbol = kernel_namespace
        .get_symbol_or_load(name, None, kernel_mmi_ref, false)
        .upgrade()
        .expect("couldn't upgrade loaded section");

    let implementation_address =
        symbol.mapped_pages.lock().start_address().value() + symbol.mapped_pages_offset;

    // shim_caller actually points to the next instruction (the value pushed onto
    // the stack when the shim caller calls the shim function). The eight bytes
    // before the next instruction are the operands for the previous instruction
    // which is the call address.
    let shim_caller_ptr = (shim_caller - 8) as *mut usize;
    // This is an address to the operand of the call instruction to the shim i.e.
    // where we will write the address of the found symbol to.
    let reloc_target: &mut usize = unsafe { &mut *shim_caller_ptr };
    *reloc_target = implementation_address;

    implementation_address
}

// FIXME: Never inline __rewrite_shim_relocations
// FIXME: Unload the shim.
#[cfg(target_arch = "x86_64")]
global_asm!(
    "
.global __rewrite_shim_relocations
__rewrite_shim_relocations:
    # Save arguments passed to the function.
    push rdi
    push rsi
    push rdx
    push rcx
    push r8
    push r9
    mov rdi, rax # call_instruction_in_shim
    mov rsi, rbx # shim_caller
    call __rewrite_shim_relocations_rust
    pop r9
    pop r9
    pop r8
    pop rcx
    pop rdx
    pop rsi
    pop rdi
    # Call the address of the actual implementation, returned by
    # __rewrite_shim_relocations_rust.
    call rax
    # Return to the original caller of the shim.
    ret
__shim_func_start:
    # We write these into scratch registers so they don't overwrite any
    # function arguments. __rewrite_shim_relocations will move them into the
    # correct registers after saving the necessary registers.
    lea rax, [rip] # call_instruction_in_shim
    lea rbx, [rsp] # shim_caller
__shim_call_rewrite:
    jmp __rewrite_shim_relocations
__shim_func_end:
"
);
