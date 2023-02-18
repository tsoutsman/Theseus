//! Exception handlers that are task-aware, and will kill a task on an exception.

// TODO: Add direct explanation to why this empty loop is necessary and criteria for replacing it with something else
#![allow(clippy::empty_loop)]
#![no_std]
#![feature(abi_x86_interrupt)]

use memory::{VirtualAddress, Page};
use signal_handler::ErrorCode;
use x86_64::{
    registers::control::Cr2,
    structures::idt::{
        InterruptStackFrame,
        PageFaultErrorCode
    },
};
use locked_idt::LockedIdt;
use fault_log::log_exception;
use halt::kill_and_halt;


/// Initialize the given `idt` with fully-featured exception handlers.
/// 
/// This only sets the exception `Entry`s in the `IDT`, i.e.,
/// entries from `0` to `31` (inclusive).
/// Entries from `32` to `255` (inclusive) are not modified, 
/// as those are for custom OS-specfici interrupt handlers.
pub fn init(idt_ref: &'static LockedIdt) {
    { 
        let mut idt = idt_ref.lock(); // withholds interrupts

        // SET UP FIXED EXCEPTION HANDLERS
        idt.divide_error.set_handler_fn(divide_error_handler);
        idt.debug.set_handler_fn(debug_handler);
        idt.non_maskable_interrupt.set_handler_fn(nmi_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.overflow.set_handler_fn(overflow_handler);
        idt.bound_range_exceeded.set_handler_fn(bound_range_exceeded_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt.device_not_available.set_handler_fn(device_not_available_handler);
        let options = idt.double_fault.set_handler_fn(double_fault_handler);
        unsafe { 
            options.set_stack_index(tss::DOUBLE_FAULT_IST_INDEX as u16);
        }
        // reserved: 0x09 coprocessor segment overrun exception
        idt.invalid_tss.set_handler_fn(invalid_tss_handler);
        idt.segment_not_present.set_handler_fn(segment_not_present_handler);
        idt.stack_segment_fault.set_handler_fn(stack_segment_fault_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        let options = idt.page_fault.set_handler_fn(demand_paging::page_fault_handler);
        unsafe { 
            options.set_stack_index(demand_paging::PAGE_FAULT_HANDLER_IST_INDEX as u16);
        }
        // reserved: 0x0F
        idt.x87_floating_point.set_handler_fn(x87_floating_point_handler);
        idt.alignment_check.set_handler_fn(alignment_check_handler);
        idt.machine_check.set_handler_fn(machine_check_handler);
        idt.simd_floating_point.set_handler_fn(simd_floating_point_handler);
        idt.virtualization.set_handler_fn(virtualization_handler);
        // reserved: 0x15 - 0x1C
        idt.vmm_communication_exception.set_handler_fn(vmm_communication_exception_handler);
        idt.security_exception.set_handler_fn(security_exception_handler);
        // reserved: 0x1F
    }

    idt_ref.load();
}


/// calls print!() and then print_raw!()
macro_rules! println_both {
    ($fmt:expr) => {
        vga_buffer::print_raw!(concat!($fmt, "\n"));
        app_io::print!(concat!($fmt, "\n"));
    };
    ($fmt:expr, $($arg:tt)*) => {
        vga_buffer::print_raw!(concat!($fmt, "\n"), $($arg)*);
        app_io::print!(concat!($fmt, "\n"), $($arg)*);
    };
}

/// Checks whether the given `vaddr` falls within a stack guard page, indicating stack overflow. 
fn is_stack_overflow(vaddr: VirtualAddress) -> bool {
    let page = Page::containing_address(vaddr);
    task::with_current_task(|t|
        t.with_kstack(|kstack| kstack.guard_page().contains(&page))
    ).unwrap_or(false)
}


/// exception 0x00
extern "x86-interrupt" fn divide_error_handler(stack_frame: InterruptStackFrame) {
    println_both!("\nEXCEPTION: DIVIDE ERROR\n{:#X?}\n", stack_frame);
    kill_and_halt(0x0, &stack_frame, None, true)
}

/// exception 0x01
extern "x86-interrupt" fn debug_handler(stack_frame: InterruptStackFrame) {
    println_both!("\nEXCEPTION: DEBUG EXCEPTION\n{:#X?}", stack_frame);
    // don't halt here, this isn't a fatal/permanent failure, just a brief pause.
}

/// exception 0x02, also used for TLB Shootdown IPIs and sampling interrupts.
///
/// # Important Note
/// Acquiring ANY locks in this function, even irq-safe ones, could cause a deadlock
/// because this interrupt takes priority over everything else and can interrupt
/// another regular interrupt. 
/// This includes printing to the log (e.g., `debug!()`) or the screen.
extern "x86-interrupt" fn nmi_handler(stack_frame: InterruptStackFrame) {
    let mut expected_nmi = false;

    // currently we're using NMIs to send TLB shootdown IPIs
    {
        let pages_to_invalidate = tlb_shootdown::TLB_SHOOTDOWN_IPI_PAGES.read().clone();
        if let Some(pages) = pages_to_invalidate {
            // trace!("nmi_handler (AP {})", cpu::current_cpu());
            tlb_shootdown::handle_tlb_shootdown_ipi(pages);
            expected_nmi = true;
        }
    }

    // Performance monitoring hardware uses NMIs to trigger a sampling interrupt.
    match pmu_x86::handle_sample(&stack_frame) {
        // A PMU sample did occur and was properly handled, so this NMI was expected. 
        Ok(true) => expected_nmi = true,
        // No PMU sample occurred, so this NMI was unexpected.
        Ok(false) => { }
        // A PMU sample did occur but wasn't properly handled, so this NMI was expected. 
        Err(_e) => {
            println_both!("nmi_handler: pmu_x86 failed to record sample: {:?}", _e);
            expected_nmi = true;
        }
    }

    if expected_nmi {
        return;
    }

    println_both!("\nEXCEPTION: NON-MASKABLE INTERRUPT at {:#X}\n{:#X?}\n",
        stack_frame.instruction_pointer,
        stack_frame,
    );

    log_exception(0x2, stack_frame.instruction_pointer.as_u64() as usize, None, None);
    kill_and_halt(0x2, &stack_frame, None, true)
}


/// exception 0x03
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println_both!("\nEXCEPTION: BREAKPOINT\n{:#X?}", stack_frame);
    // don't halt here, this isn't a fatal/permanent failure, just a brief pause.
}

/// exception 0x04
extern "x86-interrupt" fn overflow_handler(stack_frame: InterruptStackFrame) {
    println_both!("\nEXCEPTION: OVERFLOW\n{:#X?}", stack_frame);
    kill_and_halt(0x4, &stack_frame, None, true)
}

// exception 0x05
extern "x86-interrupt" fn bound_range_exceeded_handler(stack_frame: InterruptStackFrame) {
    println_both!("\nEXCEPTION: BOUND RANGE EXCEEDED\n{:#X?}", stack_frame);
    kill_and_halt(0x5, &stack_frame, None, true)
}

/// exception 0x06
extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: InterruptStackFrame) {
    println_both!("\nEXCEPTION: INVALID OPCODE\n{:#X?}", stack_frame);
    kill_and_halt(0x6, &stack_frame, None, true)
}

/// exception 0x07
///
/// For more information about "spurious interrupts", 
/// see [here](http://wiki.osdev.org/I_Cant_Get_Interrupts_Working#I_keep_getting_an_IRQ7_for_no_apparent_reason).
extern "x86-interrupt" fn device_not_available_handler(stack_frame: InterruptStackFrame) {
    println_both!("\nEXCEPTION: DEVICE NOT AVAILABLE\n{:#X?}", stack_frame);
    kill_and_halt(0x7, &stack_frame, None, true)
}

/// exception 0x08
extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    let accessed_vaddr = Cr2::read_raw();
    println_both!("\nEXCEPTION: DOUBLE FAULT\n{:#X?}\nTried to access {:#X}
        Note: double faults in Theseus are typically caused by stack overflow, is the stack large enough?",
        stack_frame, accessed_vaddr,
    );
    if is_stack_overflow(VirtualAddress::new_canonical(accessed_vaddr as usize)) {
        println_both!("--> This double fault was definitely caused by stack overflow, tried to access {:#X}.\n", accessed_vaddr);
    }
    
    kill_and_halt(0x8, &stack_frame, Some(error_code.into()), false);
    loop {}
}

/// exception 0x0A
extern "x86-interrupt" fn invalid_tss_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    println_both!("\nEXCEPTION: INVALID TSS\n{:#X?}\nError code: {:#b}", stack_frame, error_code);
    kill_and_halt(0xA, &stack_frame, Some(error_code.into()), true)
}

/// exception 0x0B
extern "x86-interrupt" fn segment_not_present_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    println_both!("\nEXCEPTION: SEGMENT NOT PRESENT\n{:#X?}\nError code: {:#b}", stack_frame, error_code);
    kill_and_halt(0xB, &stack_frame, Some(error_code.into()), true)
}

/// exception 0x0C
extern "x86-interrupt" fn stack_segment_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    println_both!("\nEXCEPTION: STACK SEGMENT FAULT\n{:#X?}\nError code: {:#b}", stack_frame, error_code);
    kill_and_halt(0xC, &stack_frame, Some(error_code.into()), true)
}

/// exception 0x0D
extern "x86-interrupt" fn general_protection_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    println_both!("\nEXCEPTION: GENERAL PROTECTION FAULT\n{:#X?}\nError code: {:#b}", stack_frame, error_code);
    kill_and_halt(0xD, &stack_frame, Some(error_code.into()), true)
}

/// exception 0x0E
extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    let accessed_vaddr = Cr2::read_raw() as usize;

    #[cfg(not(downtime_eval))] {
        println_both!("\nEXCEPTION: PAGE FAULT while accessing {:#x}\n\
            error code: {:?}\n{:#X?}",
            accessed_vaddr,
            error_code,
            stack_frame
        );
        if is_stack_overflow(VirtualAddress::new_canonical(accessed_vaddr)) {
            println_both!("--> Page fault was caused by stack overflow, tried to access {:#X}\n.", accessed_vaddr);
        }
    }
    
    kill_and_halt(0xE, &stack_frame, Some(ErrorCode::PageFaultError { accessed_address: accessed_vaddr, pf_error: error_code }), true)
}


/// exception 0x10
extern "x86-interrupt" fn x87_floating_point_handler(stack_frame: InterruptStackFrame) {
    println_both!("\nEXCEPTION: x87 FLOATING POINT\n{:#X?}", stack_frame);
    kill_and_halt(0x10, &stack_frame, None, true)
}

/// exception 0x11
extern "x86-interrupt" fn alignment_check_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    println_both!("\nEXCEPTION: ALIGNMENT CHECK\n{:#X?}\nError code: {:#b}", stack_frame, error_code);
    kill_and_halt(0x11, &stack_frame, Some(error_code.into()), true)
}

/// exception 0x12
extern "x86-interrupt" fn machine_check_handler(stack_frame: InterruptStackFrame) -> ! {
    println_both!("\nEXCEPTION: MACHINE CHECK\n{:#X?}", stack_frame);
    kill_and_halt(0x12, &stack_frame, None, true);
    loop {}
}

/// exception 0x13
extern "x86-interrupt" fn simd_floating_point_handler(stack_frame: InterruptStackFrame) {
    println_both!("\nEXCEPTION: SIMD FLOATING POINT\n{:#X?}", stack_frame);
    kill_and_halt(0x13, &stack_frame, None, true)
}

/// exception 0x14
extern "x86-interrupt" fn virtualization_handler(stack_frame: InterruptStackFrame) {
    println_both!("\nEXCEPTION: VIRTUALIZATION\n{:#X?}", stack_frame);
    kill_and_halt(0x14, &stack_frame, None, true)
}

/// exception 0x1D
extern "x86-interrupt" fn vmm_communication_exception_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    println_both!("\nEXCEPTION: VMM COMMUNICATION EXCEPTION\n{:#X?}\nError code: {:#b}", stack_frame, error_code);
    kill_and_halt(0x1D, &stack_frame, Some(error_code.into()),true)
}

/// exception 0x1E
extern "x86-interrupt" fn security_exception_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    println_both!("\nEXCEPTION: SECURITY EXCEPTION\n{:#X?}\nError code: {:#b}", stack_frame, error_code);
    kill_and_halt(0x1E, &stack_frame, Some(error_code.into()), true)
}
