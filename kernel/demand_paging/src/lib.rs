#![no_std]
#![feature(abi_x86_interrupt)]

extern crate alloc;

use locked_idt::LockedIdt;
use memory::{Mapper, Page, VirtualAddress};
use x86_64::{
    registers::control::Cr2,
    structures::idt::{InterruptStackFrame, PageFaultErrorCode},
};

pub const PAGE_FAULT_HANDLER_IST_INDEX: usize = 1;

pub fn init(idt: &'static LockedIdt, mapper: &mut Mapper) -> Result<(), &'static str> {
    let stack = stack::alloc_stack_eagerly(8, mapper)
        .ok_or("couldn't allocate stack for demand pager page fault handler")?;
    log::info!("stack for thingy: {stack:#?}");
    log::info!(
        "using top: {:0x?}",
        x86_64::VirtAddr::new(stack.top_unusable().value() as u64)
    );
    tss::TSS
        .get(&cpu::current_cpu())
        .ok_or("no TSS exists for current cpu")?
        .lock()
        .interrupt_stack_table[PAGE_FAULT_HANDLER_IST_INDEX] =
        x86_64::VirtAddr::new(stack.top_unusable().value() as u64);
    log::warn!("translated top usable: {:0x?}", mapper.translate(stack.top_usable()));
    core::mem::forget(stack);
    Ok(())
}

pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    let virtual_address = VirtualAddress::new_canonical(Cr2::read_raw() as usize);
    log::error!("{virtual_address:0x?}");

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
    log::error!("SUCCESFUL");
}

fn inner_page_fault_handler(
    virtual_address: VirtualAddress,
    error_code: PageFaultErrorCode,
) -> Result<(), &'static str> {
    log::error!("VIRTUAL ADDRESS: {virtual_address:0x?}");
    if error_code.contains(PageFaultErrorCode::PROTECTION_VIOLATION) {
        return Err("protection violation");
    }

    // TODO: Is this ok?
    log::error!("1");
    let mut mapper = Mapper::from_current();
    log::error!("2");
    let entry: &mut _ = mapper
        .get_entry(Page::containing_address(virtual_address))
        .ok_or("failed to get page table entry")?;
    log::error!("3");

    let flags = entry.flags();
    log::error!("4");
    if flags.is_requested() {
        log::error!("5");
        let frame = frame_allocator::allocate_frames(1).ok_or("failed to allocate frame")?;
        log::error!("6");
        let new_flags = flags.valid(true).requested(false);
        log::error!("7");
        entry.set_entry(frame.as_allocated_frame(), new_flags);
        log::error!("8");
        Ok(())
    } else {
        Err("page not requested")
    }
}
