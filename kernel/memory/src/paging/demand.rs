use core::arch::asm;

use x86_64::{
    registers::control::Cr2,
    structures::idt::{InterruptStackFrame, PageFaultErrorCode},
};

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
    let _virtual_address = Cr2::read_raw() as usize;
    
    // FIXME: Check if virtual address is valid and if so load it in.

    // FIXME
    unsafe { asm!("jmp {}", in(reg) ip) };
}
