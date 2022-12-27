#![no_std]
#![feature(abi_x86_interrupt)]

extern crate alloc;

use memory::{Mapper, Page, VirtualAddress};
use x86_64::{
    registers::control::Cr2,
    structures::idt::{InterruptStackFrame, PageFaultErrorCode},
};

pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    let virtual_address = VirtualAddress::new_canonical(Cr2::read_raw() as usize);
    log::debug!("{virtual_address:0x?}");

    if let Err(error) = inner_page_fault_handler(virtual_address, error_code) {
        log::error!("{error}");
        halt::kill_and_halt(
            0xe,
            &stack_frame,
            Some(::halt::ErrorCode::PageFaultError {
                accessed_address: virtual_address.value(),
                pf_error: error_code,
            }),
            true,
        );
    }
}

fn inner_page_fault_handler(
    virtual_address: VirtualAddress,
    error_code: PageFaultErrorCode,
) -> Result<(), &'static str> {
    // log::error!("VIRTUAL ADDRESS: {virtual_address:0x?}");
    if error_code.contains(PageFaultErrorCode::PROTECTION_VIOLATION) {
        return Err("protection violation");
    }

    // TODO: Is this ok?
    let mut mapper = Mapper::from_current();
    let entry: &mut _ = mapper
        .get_entry(Page::containing_address(virtual_address))
        .ok_or("failed to get page table entry")?;

    let flags = entry.flags();
    if flags.is_requested() {
        let frame = frame_allocator::allocate_frames(1).ok_or("failed to allocate frame")?;
        let new_flags = flags.valid(true).requested(false);
        entry.set_entry(frame.as_allocated_frame(), new_flags);
        Ok(())
    } else {
        Err("page not requested")
    }
}
