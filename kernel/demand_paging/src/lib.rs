#![no_std]
#![feature(abi_x86_interrupt)]

extern crate alloc;

use alloc::boxed::Box;
use core::arch::asm;
use hashbrown::HashMap;
use irq_safety::MutexIrqSafe;
use memory::VirtualAddress;
use x86_64::{
    registers::control::Cr2,
    structures::idt::{InterruptStackFrame, PageFaultErrorCode},
};

lazy_static::lazy_static! {
    static ref REQUESTED_PAGES: MutexIrqSafe<
        // TODO: Use page size
        HashMap<VirtualAddress, Box<dyn FnOnce() -> [u8; 4096] + Send>>,
    > = MutexIrqSafe::new(HashMap::new());
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    if error_code.contains(PageFaultErrorCode::PROTECTION_VIOLATION) {
        todo!("program violated page protections e.g. writing to immutable page");
    }

    if error_code.contains(PageFaultErrorCode::CAUSED_BY_WRITE) {
        todo!("do we demand page writable pages?");
    }

    if error_code.contains(PageFaultErrorCode::INSTRUCTION_FETCH) {
        todo!("could be useful");
    }

    let ip: u64 = stack_frame.instruction_pointer.as_u64();
    let virtual_address = VirtualAddress::new_canonical(Cr2::read_raw() as usize);

    let mut pages = REQUESTED_PAGES.lock();
    let bytes = match pages.remove(&virtual_address) {
        Some(f) => {
            f()
        }
        None => {
            todo!();
        }
    };

    let slice =
        unsafe { core::slice::from_raw_parts_mut(virtual_address.value() as *mut u8, 4096) };
    slice.copy_from_slice(&bytes);

    // FIXME
    unsafe { asm!("jmp {}", in(reg) ip) };
}
