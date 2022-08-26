#![deny(unsafe_op_in_unsafe_fn)]

use crate::*;
use core::{arch::global_asm, ffi::CStr};
use crate_metadata::R_X86_64_64;
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
        log::error!("-------------------------------------------------------------------------------------------");
        let mut new_crate = new_crate_ref.lock_as_mut().unwrap();

        let mapped_pages = Arc::new(Mutex::new(MappedPages::empty()));

        let mut shim = Vec::new();
        let mut shim_sections = HashMap::new();

        let relocations = relocations
            .iter()
            .map(|relocation| {
                let source_entry = &symbol_table[relocation.get_symbol_table_index() as usize];

                let source_index = source_entry.shndx() as usize;
                let source_name = source_entry.get_name(&elf_file).unwrap();

                let start_address = match new_crate.sections.get(&source_index) {
                    // This is a relocation within the same crate.
                    Some(ss) => {
                        log::info!("internal reloc: {}", ss.name);
                        ss.start_address()
                    }
                    // This is a relocation to some other crate.
                    None => {
                        let source_name = demangle(source_name).to_string();

                        // If the symbol is already loaded there's no need to create shims.
                        if let Some(sec) = self.get_symbol_internal(&source_name) {
                            log::trace!("external reloc: {}", source_name);
                            sec.upgrade().unwrap().start_address()
                        } else {
                            log::error!("shim reloc: {}", source_name);
                            if !shim_sections.contains_key(&source_name) {
                                let sec_shim = generate_function_shim(&source_name);
                                let sec = Arc::new(LoadedSection {
                                    name: StrRef::from(source_name.as_ref()),
                                    typ: SectionType::Text,
                                    global: true,
                                    mapped_pages: Arc::clone(&mapped_pages),
                                    mapped_pages_offset: shim.len(),
                                    address_range: VirtualAddress::new_canonical(shim.len())
                                        ..VirtualAddress::new_canonical(
                                            shim.len() + sec_shim.len(),
                                        ),
                                    parent_crate: CowArc::downgrade(new_crate_ref),
                                    inner: Default::default(),
                                });
                                shim.extend(sec_shim);
                                shim_sections.insert(source_name, sec);
                            }
                            VirtualAddress::zero()
                        }
                    }
                };
                (relocation, start_address)
            })
            .collect::<Vec<_>>();

        if shim.len() == 0 {
            log::info!("{} did not require shims", new_crate.crate_name);
            assert_eq!(shim_sections.len(), 0);
            return Ok(());
        }

        log::error!("NUM SHIMS: {}", shim_sections.len());
        log::error!("SHIM: {shim_sections:#?}");

        let mut mapped_pages =
            memory::create_mapping(shim.len(), TEXT_SECTION_FLAGS | EntryFlags::WRITABLE)?;
        // TODO: Create pages with shim rather than cloning into.
        mapped_pages
            .as_slice_mut(0, shim.len())?
            .clone_from_slice(&shim);

        let start = mapped_pages.start_address();

        // TODO
        let mut shndx = 10000;

        for (_, mut sec) in shim_sections.iter_mut() {
            let address_range = (start + sec.address_range.start)..(start + sec.address_range.end);

            let sec_mut = Arc::get_mut(&mut sec).unwrap();
            sec_mut.address_range = address_range;
            drop(sec_mut);

            new_crate.sections.insert(shndx, Arc::clone(sec));
            new_crate.global_sections.insert(shndx);

            shndx += 1;
        }

        for (relocation, start_address) in relocations {
            let entry = &symbol_table[relocation.get_symbol_table_index() as usize];
            let start_address = if start_address == VirtualAddress::zero() {
                let name = entry.get_name(&elf_file).unwrap();
                let name = demangle(name).to_string();
                shim_sections.get(&name).unwrap().start_address()
            } else {
                start_address
            };

            write_relocation(
                RelocationEntry {
                    typ: R_X86_64_64,
                    addend: 0,
                    offset: 0,
                },
                todo!(),
                0,
                start_address + entry.value() as usize,
                false,
            )?;

            // let source_index = source_entry.shndx() as usize;

            // if new_crate.sections.get(&source_index).is_none() {
            //     // TODO
            // }
        }

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
    log::error!("---------------------------------------------- __rewrite_shim_relocations_rust ----------------------------------------------");
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
