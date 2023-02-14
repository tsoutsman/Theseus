use core::sync::atomic::{AtomicU64, Ordering};
use x86_64::structures::idt::InterruptStackFrame;

const ZERO: AtomicU64 = AtomicU64::new(0);

static IS_IDLE: [AtomicU64; u8::MAX as usize / 64] = [ZERO; u8::MAX as usize / 64];

pub fn entry(_: ()) {
    log::info!("entering idle task");

    let core = cpu::current_cpu();
    let index = core as usize / 64;

    loop {
        preemption::disable_preemption();

        assert_eq!(
            IS_IDLE[index].fetch_or(0x1 << (core % 64), Ordering::AcqRel) >> (core % 64) & 0x1,
            0
        );

        while !needs_rescheduling(core) {
            log::info!("about to halt: {core}");
            unsafe { core::arch::asm!("hlt") };
            log::info!("post halt: {core}");
        }

        if IS_IDLE[index].fetch_and(!(0x1 << (core % 64)), Ordering::AcqRel) >> (core % 64) & 0x1
            == 1
        {
            log::error!("11111111111111111111111111: {core}");
            // Something other than a scheduler IPI woke us up.
            preemption::enable_preemption();
        } else {
            log::error!("22222222222222222222222222: {core}");
        }

        crate::schedule();
    }
}

pub extern "x86-interrupt" fn scheduler_ipi_handler(_: InterruptStackFrame) {
    let core = cpu::current_cpu();
    let index = core as usize / 64;

    if IS_IDLE[index].fetch_and(!(0x1 << (core % 64)), Ordering::AcqRel) >> (core % 64) & 0x1 == 1 {
        log::error!("33333333333333333333333333: {core}");
        preemption::enable_preemption();
    } else {
        log::error!("44444444444444444444444444: {core}");
    }

    if let Some(my_apic) = apic::get_my_apic() {
        my_apic.write().eoi();
    } else {
        log::error!("BUG: couldn't get my LocalApic instance to send EOI!");
    }
}

pub(crate) fn is_idle(core: u8) -> bool {
    IS_IDLE[core as usize / 64].load(Ordering::Acquire) >> (core % 64) == 1
}

fn needs_rescheduling(cpu: u8) -> bool {
    if let Some(run_queue) = crate::get_run_queue(cpu) {
        !run_queue.read().is_empty()
    } else {
        true
    }
}
