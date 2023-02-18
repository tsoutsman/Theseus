#![no_std]

use fault_log::log_exception;
use log::{warn, debug, trace};
use memory_structs::VirtualAddress;
use signal_handler::{Signal, SignalContext};
use x86_64::structures::idt::InterruptStackFrame;

pub use signal_handler::ErrorCode;

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

/// Converts the given `exception_number` into a [`Signal`] category, if relevant.
fn exception_to_signal(exception_number: u8) -> Option<Signal> {
    match exception_number {
        0x00 | 0x04 | 0x10 | 0x13         => Some(Signal::ArithmeticError),
        0x05 | 0x0E | 0x0C                => Some(Signal::InvalidAddress),
        0x06 | 0x07 | 0x08 | 0x0A | 0x0D  => Some(Signal::IllegalInstruction),
        0x0B | 0x11                       => Some(Signal::BusError),
        _                                 => None,
    }
}

/// Kills the current task (the one that caused an exception) by unwinding it.
/// 
/// # Important Note
/// Currently, unwinding a task after an exception does not fully work like it does for panicked tasks.
/// The problem is that unwinding cleanup routines (landing pads) are generated *only if* a panic can actually occur. 
/// Since machine exceptions can occur anywhere at any time (beneath the language level),
/// 
/// Currently, what will happen is that all stack frames will be unwound properly **except**
/// for the one during which the exception actually occurred; 
/// the "excepted"/interrupted frame may be cleaned up properly, but it is unlikely. 
/// 
/// However, stack traces / backtraces work, so we are correctly traversing call stacks with exception frames.
/// 
#[inline(never)]
pub fn kill_and_halt(
    exception_number: u8,
    stack_frame: &InterruptStackFrame,
    error_code: Option<ErrorCode>,
    print_stack_trace: bool
) {
    // First, log the exception that merits a kill operation.
    {
        let (err, addr) = match error_code {
            Some(ErrorCode::PageFaultError {accessed_address, pf_error}) => (Some(pf_error.bits()), Some(accessed_address)),
            Some(ErrorCode::Other(e)) => (Some(e), None),
            None => (None, None),
        };
        log_exception(exception_number, stack_frame.instruction_pointer.as_u64() as usize, err, addr);
    }


    #[cfg(all(unwind_exceptions, not(downtime_eval)))] {
        println_both!("Unwinding {:?} due to exception {}.", task::get_my_current_task(), exception_number);
    }
    #[cfg(not(unwind_exceptions))] {
        println_both!("Killing task without unwinding {:?} due to exception {}. (cfg `unwind_exceptions` is not set.)", task::get_my_current_task(), exception_number);
    }
    
    // Dump some info about the this loaded app crate
    // and test out using debug info for recovery
    if false {
        let curr_task = task::get_my_current_task().expect("kill_and_halt: no current task");
        let app_crate = curr_task.app_crate.as_ref().expect("kill_and_halt: no app_crate").clone_shallow();
        let debug_symbols_file = {
            let krate = app_crate.lock_as_ref();
            trace!("============== Crate {} =================", krate.crate_name);
            for s in krate.sections.values() {
                trace!("   {:?}", s);
            }
            krate.debug_symbols_file.clone()
        };

        if false {
            let mut debug = debug_info::DebugSymbols::Unloaded(debug_symbols_file);
            let debug_sections = debug.load(&app_crate, curr_task.get_namespace()).unwrap();
            let instr_ptr = stack_frame.instruction_pointer.as_u64() as usize - 1; // points to the next instruction (at least for a page fault)

            let res = debug_sections.find_subprogram_containing(VirtualAddress::new_canonical(instr_ptr));
            debug!("Result of find_subprogram_containing: {:?}", res);
        }
    }

    // print a stack trace
    #[cfg(not(downtime_eval))] {
        if print_stack_trace {
            println_both!("------------------ Stack Trace (DWARF) ---------------------------");
            let stack_trace_result = stack_trace::stack_trace(
                &mut |stack_frame, stack_frame_iter| {
                    let symbol_offset = stack_frame_iter.namespace().get_section_containing_address(
                        VirtualAddress::new_canonical(stack_frame.call_site_address() as usize),
                        false
                    ).map(|(sec, offset)| (sec.name.clone(), offset));
                    if let Some((symbol_name, offset)) = symbol_offset {
                        println_both!("  {:>#018X} in {} + {:#X}", stack_frame.call_site_address(), symbol_name, offset);
                    } else {
                        println_both!("  {:>#018X} in ??", stack_frame.call_site_address());
                    }
                    true
                },
                None,
            );
            match stack_trace_result {
                Ok(()) => { println_both!("  Beginning of stack"); }
                Err(e) => { println_both!("  {}", e); }
            }
            println_both!("---------------------- End of Stack Trace ------------------------");
        }
    }

    let cause = task::KillReason::Exception(exception_number);

    // Call this task's kill handler, if it has one.
    if let Some(ref kh_func) = task::take_kill_handler() {
        #[cfg(not(downtime_eval))]
        debug!("Found kill handler callback to invoke in Task {:?}", task::get_my_current_task());
        kh_func(&cause);
    } else {
        #[cfg(not(downtime_eval))]
        debug!("No kill handler callback in Task {:?}", task::get_my_current_task());
    }

    // Invoke the proper signal handler registered for this task, if one exists.
    if let Some(signal) = exception_to_signal(exception_number) {
        if let Some(handler) = signal_handler::take_signal_handler(signal) {
            warn!("Invoking signal handler for {:?}", signal);
            let signal_context = SignalContext {
                instruction_pointer: VirtualAddress::new_canonical(stack_frame.instruction_pointer.as_u64() as usize),
                stack_pointer: VirtualAddress::new_canonical(stack_frame.stack_pointer.as_u64() as usize),
                signal,
                error_code,
            };
            if handler(&signal_context).is_ok() {
                warn!("Signal handler for {:?} returned Ok. Returning from exception handler is disabled and untested.", signal);
                // TODO: test and enable this return;
            }
        }
    }

    // Unwind the current task that failed due to the given exception.
    // This doesn't always work perfectly, so it's disabled by default for now.
    #[cfg(unwind_exceptions)] {
        // skip 2 frames: `start_unwinding` and `kill_and_halt`
        match unwind::start_unwinding(cause, 2) {
            Ok(_) => {
                println_both!("BUG: when handling exception {}, start_unwinding() returned an Ok() value, \
                    which is unexpected because it means no unwinding actually occurred. Task: {:?}.", 
                    exception_number,
                    task::get_my_current_task()
                );
            }
            Err(e) => {
                println_both!("Task {:?} was unable to start unwinding procedure after exception {}, error: {}.",
                    task::get_my_current_task(), exception_number, e
                );
            }
        }
    }
    #[cfg(not(unwind_exceptions))] {
        let res = task::with_current_task(|t| {
            let kill_result = t.kill(cause);
            match kill_result {
                Ok(()) => { println_both!("Task {:?} killed itself successfully", t); }
                Err(e) => { println_both!("Task {:?} was unable to kill itself. Error: {:?}", t, e); }
            }
            kill_result
        });
        if res.is_err() {
            println_both!("BUG: kill_and_halt(): Couldn't get current task in order to kill it.");
        }
    }

    // If we failed to handle the exception and unwind the task, there's not really much we can do about it,
    // other than just let the thread spin endlessly (which doesn't hurt correctness but is inefficient). 
    // But in general, this task should have already been marked as killed and thus no longer schedulable,
    // so it should not reach this point. 
    // Only exceptions during the early OS initialization process will get here, meaning that the OS will basically stop.
    loop { }
}
