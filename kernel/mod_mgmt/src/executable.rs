use alloc::sync::Arc;
use core::cmp::{max, min};
use xmas_elf::ElfFile;
use xmas_elf::program::Type;
use xmas_elf::sections::{SectionData, ShType};
use xmas_elf::symbol_table::Entry;
use crate_metadata::StrRef;
use crate_name_utils::crate_name_from_path;
use fs_node::{File, FileOrDir, FileRef};
use memory::{MappedPages, MmiRef, PteFlags, PteFlagsArch};
use path::{Path, PathBuf};
use crate::{AppCrateRef, CrateNamespace};

fn load_file<'a>(namespace: &Arc<CrateNamespace>, file: &'a dyn File) -> Result<&'a [u8], &'static str> {
    let mapped_pages = file.as_mapping()?;
    let size_in_bytes = file.len();
    let abs_path = PathBuf::from(file.get_absolute_path());
    let crate_name = StrRef::from(
        crate_name_from_path(&abs_path)
            .ok_or("failed to get crate name from path")?
    );

    // First, check to make sure this crate hasn't already been loaded.
    // Application crates are now added to the CrateNamespace just like kernel crates,
    // so to load an application crate multiple times and run multiple instances of it,
    // you can create a top-level new namespace to hold that application crate.
    if namespace.get_crate(&crate_name).is_some() {
        return Err("the crate has already been loaded, cannot load it again in the same namespace");
    }

    // It's probably better to pass in the actual crate file reference so we can use it here,
    // but since we don't currently do that, we just get another reference to the crate object file via its Path.
    let crate_object_file = match Path::get_absolute(&abs_path) {
        Some(FileOrDir::File(f)) => f,
        _ => return Err("BUG: load_crate_sections(): couldn't get crate object file path"),
    };

    // Parse the crate file as an ELF file
    let buf: &[u8] = mapped_pages.as_slice(0, size_in_bytes)?;

    Ok(buf)
}

fn load(namespace: &Arc<CrateNamespace>, crate_object_file: &FileRef, kernel_mmi: &MmiRef) -> Result<AppCrateRef, &'static str> {
    let file = crate_object_file.lock();
    let buf = load_file(namespace, &*file)?;
    let elf_file = ElfFile::new(buf)?;

    let (mut start, mut end) = (usize::MAX, usize::MAX);
    let mut num_programs = 0;

    for header in elf_file.program_iter() {
        log::info!("{header:?}");
        match header.get_type()? {
            Type::Load => {
                if header.mem_size() == 0 {
                    continue;
                }

                let virtual_address = header.virtual_addr() as usize;
                start = min(start, virtual_address);
                end = max(end, virtual_address + header.mem_size() as usize);
                num_programs += 1;
            }
            Type::Dynamic => {}
            Type::Interp => todo!(),
            Type::ShLib => todo!(),
            Type::Tls => todo!(),
            _ => {}
        }
    }

    // TODO: Fine grained permissions.
    let mut mapped_pages = memory::create_mapping(end - start, PteFlags::new().executable(true).writable(true))?;
    let slice = mapped_pages.as_slice_mut(0, mapped_pages.size_in_bytes())?;

    for header in elf_file.program_iter() {
        match header.get_type()? {
            Type::Load => {
                let memory_size = header.mem_size() as usize;

                if memory_size == 0 {
                    continue;
                }

                let file_size = header.file_size() as usize;
                let offset = header.offset() as usize;

                let start = header.virtual_addr() as usize;
                let end = start + file_size;

                slice[start..end].copy_from_slice(&buf[offset..(offset + file_size)]);

                if memory_size > file_size {
                    slice[end..(start + memory_size)].fill(0);
                }
            }
            _ => {}
        }
    }

    let mut start_offset = None;

    'outer: for section in elf_file.section_iter() {
        if let Ok(SectionData::SymbolTable64(symbols)) = section.get_data(&elf_file) {
            for symbol in symbols {
                if let Ok("_start") = symbol.get_name(&elf_file) {
                    start_offset = Some(symbol.value());
                    break 'outer;
                }
            }
        }
    }

    let start_offset = start_offset.ok_or("no _start symbol found for executable")? as usize;
    let start = start_offset + mapped_pages.start_address().value();

    log::info!("executable has entry point {start_offset} adjusted to {start}");

    todo!();
}