#![no_std]

mod arch;

use core::sync::atomic::{AtomicU64, Ordering};

const ZERO: AtomicU64 = AtomicU64::new(0);

// Atomic bit-map
static IS_IDLE: [AtomicU64; u8::MAX as usize / 64] = [ZERO; u8::MAX as usize / 64];

pub fn entry(_: ()) {
    log::info!("entering idle task");

    let core = cpu::current_cpu();
    let index = core as usize / 64;

    loop {
        let guard = preemption::hold_preemption();

        IS_IDLE[index].fetch_or(0x1 << (core % 64), Ordering::Release);

        while !needs_rescheduling(core) {
            unsafe { core::arch::asm!("hlt") };
            log::info!("checking again");
        }

        IS_IDLE[index].fetch_xor(0x1 << (core % 64), Ordering::Release);
        drop(guard);

        scheduler::schedule();
    }
}

pub fn is_idle(core: u8) -> bool {
    let index = core as usize / 64;
    let u64 = IS_IDLE[index].load(Ordering::Acquire);
    (u64 >> (core % 64) & 0x1) == 1
}

fn needs_rescheduling(cpu: u8) -> bool {
    if let Some(run_queue) = runqueue::get_runqueue(cpu) {
        !run_queue.read().is_empty()
    } else {
        true
    }
}
