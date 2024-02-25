//! An application that loads C language ELF executables atop Theseus.
//!
//! This will be integrated into the Theseus kernel in the future,
//! likely as a separate crate that integrates well with the `mod_mgmt` crate.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeSet,
    format,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
use core::{
    cmp::{max, min},
    ops::Range,
};

use app_io::println;
use getopts::{Matches, Options};
use log::{debug, error, warn};
use memory::{MappedPages, Page, PteFlags, PteFlagsArch, VirtualAddress};
use mod_mgmt::{
    find_symbol_table, write_relocation, CrateNamespace, RelocationEntry, StrongDependency,
};
use path::Path;
use rustc_demangle::demangle;
use xmas_elf::{program::SegmentData, sections::ShType, ElfFile, P64};

pub fn main(args: Vec<String>) -> isize {
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(args) {
        Ok(m) => m,
        Err(_f) => {
            println!("{}", _f);
            print_usage(opts);
            return -1;
        }
    };

    if matches.opt_present("h") {
        print_usage(opts);
        return 0;
    }

    match rmain(matches) {
        Ok(retval) => retval as isize,
        Err(e) => {
            println!("Error:\n{}", e);
            -1
        }
    }
}

fn rmain(matches: Matches) -> Result<c_int, String> {
    let (curr_wd, namespace, mmi) = task::with_current_task(|curr_task| {
        (
            curr_task.get_env().lock().working_dir.clone(),
            curr_task.get_namespace().clone(),
            curr_task.mmi.clone(),
        )
    })
    .map_err(|_| String::from("failed to get current task"))?;

    let path = matches
        .free
        .first()
        .ok_or_else(|| "Missing path to ELF executable".to_string())?;
    let file_ref = Path::new(path)
        .get_file(&curr_wd)
        .ok_or_else(|| format!("Failed to access file at {path:?}"))?;
    let file = file_ref.lock();

    // Parse the file as an ELF executable
    let file_mp = file.as_mapping().map_err(String::from)?;
    let byte_slice: &[u8] = file_mp.as_slice(0, file.len())?;

    let (mut segments, entry_point, elf_file) = parse_and_load_elf_executable(byte_slice)?;
    debug!("Parsed ELF executable, moving on to overwriting relocations.");

    // Now, overwrite (recalculate) the relocations that refer to symbols that
    // already exist in Theseus, most important of which are static data
    // sections, as it is logically incorrect to have duplicates of data that
    // are supposed to be global system-wide singletons. We should throw a
    // warning here if there are no relocations in the file, as it was probably
    // built/linked with the wrong arguments. overwrite_relocations(&namespace,
    // &mut segments, &elf_file, &mmi, false)?;

    // Remap each segment's mapped pages using the correct flags; they were
    // previously mapped as always writable.
    {
        let page_table = &mut mmi.lock().page_table;
        for segment in segments.iter_mut() {
            if segment.mp.flags() != segment.flags {
                segment.mp.remap(page_table, segment.flags)?;
            }
        }
    }

    segments.iter().enumerate().for_each(|(i, seg)| {
        debug!(
            "Segment {} needed {} relocations to be rewritten.",
            i,
            seg.sections_i_depend_on.len()
        )
    });

    let _executable = LoadedExecutable {
        segments,
        entry_point,
    }; // must persist through the entire executable's runtime.

    debug!("Jumping to entry point {:#X}", entry_point);

    let dummy_args = ["hello", "world"];
    let dummy_env = ["USER=root", "PWD=/"];

    // TODO: FIXME: use `MappedPages::as_func()` instead of `transmute()`.
    let start_fn: StartFunction = unsafe { core::mem::transmute(entry_point.value()) };
    let c_retval = start_fn(&dummy_args, &dummy_env);

    debug!(
        "C _start entry point returned value {}({:#X})",
        c_retval, c_retval
    );

    Ok(c_retval)
}

/// Corresponds to C function:  `int foo()`
use libc::c_int;
use xmas_elf::{dynamic::Tag, sections::Rela, symbol_table::Entry};

type StartFunction = fn(args: &[&str], env: &[&str]) -> c_int;

#[allow(unused)]
struct LoadedExecutable {
    segments: Vec<LoadedSegment>,
    entry_point: VirtualAddress,
}

/// Represents an ELF program segment that has been loaded into memory.
#[derive(Debug)]
#[allow(dead_code)]
pub struct LoadedSegment {
    /// The memory region allocated to hold this program segment.
    mp: MappedPages,
    /// The specific range of virtual addresses occupied by this
    /// (may be a subset)
    bounds: Range<VirtualAddress>,
    /// The proper flags for this segment specified by the ELF file.
    flags: PteFlagsArch,
    /// The indices of the sections in the ELF file
    /// that were grouped ("mapped") into this segment by the linker.
    section_ndxs: BTreeSet<usize>,
    /// The list of sections in existing Theseus crates that this segment's
    /// sections depends on, i.e., the required dependencies that must exist
    /// as long as this segment.
    sections_i_depend_on: Vec<StrongDependency>,
}

/// Parses an elf executable file from the given slice of bytes and load it into
/// memory.
///
/// # Important note about memory mappings
/// This function will allocate new memory regions to store each program segment
/// and copy each segment's data into them.
/// When this function returns, those segments will be mapped as writable in
/// order to allow them to be modified as needed.
/// Before running this executable, each segment's `MappedPages` should be
/// remapped to the proper `flags` specified in its `LoadedSegment.flags` field.
///
/// # Return
/// Returns a tuple of:
/// 1. A list of program segments mapped into memory.
/// 2. The virtual address of the executable's entry point, e.g., the `_start`
///    function. This is the function that we should call to start running the
///    executable.
/// 3. The `Offset` by which all virtual addresses in the loaded executable
///    should be shifted by. This is the difference between where the program is
///    *actually* loaded in memory and where the program *expected* to be loaded
///    into memory.
/// 4. A reference to the parsed `ElfFile`, whose lifetime is tied to the given
///    `file_contents` parameter.
fn parse_and_load_elf_executable(
    file_contents: &[u8],
) -> Result<(Vec<LoadedSegment>, VirtualAddress, ElfFile), String> {
    debug!("Parsing Elf executable of size {}", file_contents.len());

    let elf_file = ElfFile::new(file_contents).map_err(String::from)?;

    // check that elf_file is an executable type
    let typ = elf_file.header.pt2.type_().as_type();
    if typ != xmas_elf::header::Type::SharedObject {
        error!(
            "parse_elf_executable(): ELF file has wrong type {:?}, must be an Executable Elf File!",
            typ
        );
        return Err("not an executable".into());
    }

    let (mut start_vaddr, mut end_vaddr) = (usize::MAX, usize::MIN);
    let mut num_segments = 0;
    for prog_hdr in elf_file.program_iter() {
        if prog_hdr.get_type() == Ok(xmas_elf::program::Type::Load) {
            num_segments += 1;
            start_vaddr = min(start_vaddr, prog_hdr.virtual_addr() as usize);
            end_vaddr = max(
                end_vaddr,
                prog_hdr.virtual_addr() as usize + prog_hdr.mem_size() as usize,
            );
        }
    }

    let mut mapped_segments = Vec::with_capacity(num_segments);

    // Allocate enough virtually-contiguous space for all the segments together.
    let total_size_in_bytes = end_vaddr - start_vaddr;
    let mut all_pages = memory::allocate_pages_by_bytes(total_size_in_bytes)
        .ok_or_else(|| format!("Failed to allocate {total_size_in_bytes}"))?;
    let file_start = all_pages.start_address();

    for section in elf_file.section_iter() {
        let name = section.get_name(&elf_file);
        log::info!("section: {name:#?} {:?}", section.get_type());
    }
    log::info!("done");

    // Iterate through each segment again and map them into pages we just allocated
    // above, copying their segment data to the proper location.
    for (segment_ndx, prog_hdr) in elf_file.program_iter().enumerate() {
        log::info!("looking at segment {segment_ndx} {prog_hdr:#?}");
        // if prog_hdr.get_type() == Ok(xmas_elf::program::Type::Load) ||
        // prog_hdr.get_type() == Ok(xmas_elf::program::Type::Phdr) {
        if prog_hdr.get_type() != Ok(xmas_elf::program::Type::Load) {
            continue;
        }

        // A segment (program header) has two sizes:
        // 1) memory size: the size in memory that the segment, when loaded, will
        //    actually consume. This is how much virtual memory space we have to
        //    allocate for it.
        // 2) file size: the size of the segment's actual data from the ELF file itself.
        //    This is how much data we will actually copy from the file's segment into
        //    our allocated memory.
        // The difference is primarily due to .bss sections, in which the file size will
        // be less than the memory size. If memory size > file size, the
        // difference should be filled with zeros.
        let memory_size_in_bytes = prog_hdr.mem_size() as usize;
        let file_size_in_bytes = prog_hdr.file_size() as usize;
        if memory_size_in_bytes == 0 {
            // warn!("Skipping zero-sized LOAD segment {:?}", prog_hdr);
            continue;
        }

        let offset = VirtualAddress::new(prog_hdr.virtual_addr() as usize).ok_or_else(|| {
            error!("Program header virtual address was invalid: {:?}", prog_hdr);
            "Program header had an invalid virtual address"
        })?;
        let start_vaddr = file_start + offset;
        let end_page = Page::containing_address(start_vaddr + (memory_size_in_bytes - 1));

        debug!("Splitting {:?} after end page {:?}", all_pages, end_page);

        let (this_ap, remaining_pages) = all_pages.split(end_page + 1).map_err(|_ap| {
            format!("Failed to split allocated pages {_ap:?} at page {start_vaddr:#X}")
        })?;
        all_pages = remaining_pages;
        debug!(
            "Successfully split pages into {:?} and {:?}",
            this_ap, all_pages
        );
        debug!(
            "Adjusted segment vaddr: {:#X}, size: {:#X}, {:?}",
            start_vaddr,
            memory_size_in_bytes,
            this_ap.start_address()
        );

        let initial_flags = convert_to_pte_flags(prog_hdr.flags());
        let mmi = task::with_current_task(|t| t.mmi.clone()).unwrap();
        // Must initially map the memory as writable so we can copy the segment data to
        // it later.
        let mut mp = mmi
            .lock()
            .page_table
            .map_allocated_pages(this_ap, initial_flags.writable(true))
            .map_err(String::from)?;

        // Copy data from this section into the correct offset into our newly-mapped
        // pages
        let offset_into_mp = mp.offset_of_address(start_vaddr).ok_or_else(|| {
            format!("BUG: destination address {start_vaddr:#X} wasn't within segment's {mp:?}")
        })?;
        match prog_hdr.get_data(&elf_file).map_err(String::from)? {
            SegmentData::Undefined(segment_data) => {
                // debug!("Segment had undefined data of {} ({:#X}) bytes, file size {}
                // ({:#X})",     segment_data.len(), segment_data.len(),
                // file_size_in_bytes, file_size_in_bytes);
                let dest_slice: &mut [u8] = mp
                    .as_slice_mut(offset_into_mp, memory_size_in_bytes)
                    .map_err(String::from)?;
                dest_slice[..file_size_in_bytes]
                    .copy_from_slice(&segment_data[..file_size_in_bytes]);
                if memory_size_in_bytes > file_size_in_bytes {
                    // debug!("    Zero-filling extra bytes for segment from range [{}:{}).",
                    // file_size_in_bytes, dest_slice.len());
                    dest_slice[file_size_in_bytes..].fill(0);
                }
            }
            other => {
                warn!("Segment had data of unhandled type: {:?}", other);
            }
        };

        let segment_bounds = start_vaddr..(start_vaddr + memory_size_in_bytes);

        // Populate the set of sections that comprise this segment.
        let mut section_ndxs = BTreeSet::new();
        for (shndx, sec) in elf_file.section_iter().enumerate() {
            if segment_bounds.contains(&VirtualAddress::new_canonical(sec.address() as usize)) {
                section_ndxs.insert(shndx);
            }
        }

        debug!(
            "Loaded segment {} at {:X?} contains sections: {:?}",
            segment_ndx, segment_bounds, section_ndxs
        );

        mapped_segments.push(LoadedSegment {
            mp,
            bounds: segment_bounds,
            flags: initial_flags.into(),
            section_ndxs,
            sections_i_depend_on: Vec::new(), /* this is populated later in
                                               * `overwrite_relocations()` */
        });
    }

    let mut relocation_table_offset = None;
    let mut relocation_table_size = None;
    let mut relocation_entry_size = None;

    for (segment_ndx, prog_hdr) in elf_file.program_iter().enumerate() {
        log::info!("looking at segment {segment_ndx} {prog_hdr:#?}");
        if let Ok(SegmentData::Dynamic64(list)) = prog_hdr.get_data(&elf_file) {
            for entry in list {
                if let Ok(tag) = entry.get_tag() {
                    match tag {
                        Tag::Rela => {
                            relocation_table_offset = Some(entry.get_ptr().unwrap());
                        }
                        Tag::RelaSize => relocation_table_size = Some(entry.get_val().unwrap()),
                        Tag::RelaEnt => relocation_entry_size = Some(entry.get_val().unwrap()),
                        _ => log::warn!("unhandled dyn tag: {tag:?}"),
                    }
                } else {
                    log::error!("Error decoding tag");
                }
            }
        }
    }

    // The offset of the relocation table.
    let relocation_table_offset = relocation_table_offset.unwrap() as usize;
    let relocation_table_size = relocation_table_size.unwrap();
    let relocation_entry_size = relocation_entry_size.unwrap();

    let ptr = (file_start + relocation_table_offset).value() as *const Rela<P64>;
    let len = (relocation_table_size / relocation_entry_size) as usize;
    assert_eq!(
        relocation_table_size % relocation_entry_size,
        0,
        "relocation table size wasn't a multiple of entry size"
    );
    let rela_table = unsafe { core::slice::from_raw_parts(ptr, len) };

    for rela in rela_table {
        let x = rela.get_symbol_table_index();
        log::info!("X: {x:0x?}");
        let entry = RelocationEntry::from_elf_relocation(rela);
        let slice = unsafe {
            core::slice::from_raw_parts_mut(file_start.value() as *mut u8, total_size_in_bytes)
        };
        write_relocation(entry, slice, 0, file_start, false).unwrap();
    }


    let entry_offset = VirtualAddress::new(elf_file.header.pt2.entry_point() as usize)
        .ok_or("invalid entry point address")?;
    let entry_vaddr = entry_offset + file_start;

    debug!(
        "ELF had entry point {:#X}, adjusted to {:#X}",
        entry_offset, entry_vaddr
    );

    Ok((mapped_segments, entry_vaddr, elf_file))
}

/// Converts the given ELF program flags into `PteFlags`.
fn convert_to_pte_flags(prog_flags: xmas_elf::program::Flags) -> PteFlags {
    PteFlags::new()
        .valid(prog_flags.is_read())
        .writable(prog_flags.is_write())
        .executable(prog_flags.is_execute())
}

fn print_usage(opts: Options) {
    println!("{}", opts.usage(USAGE));
}

const USAGE: &str = "Usage: loadc [ARGS] PATH
Loads C language ELF executables on Theseus.";
