//! A library for defining CPU-local variables.
//!
//! See [`cpu_local`] for more details.

#![feature(int_roundings)]
#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use core::{iter, ptr};

pub use cls_macros::cpu_local;
use cpu::CpuId;
use memory::PteFlags;
use sync_spin::SpinMutex;

/// A trait abstracting over guards that ensure atomicity with respect to the
/// current CPU.
///
/// This trait is "sealed" and cannot be implemented by anything outside this
/// crate.
pub trait CpuAtomicGuard: sealed::Sealed {}

impl sealed::Sealed for irq_safety::HeldInterrupts {}
impl CpuAtomicGuard for irq_safety::HeldInterrupts {}

impl sealed::Sealed for preemption::PreemptionGuard {}
impl CpuAtomicGuard for preemption::PreemptionGuard {}

mod sealed {
    pub trait Sealed {}
}

#[doc(hidden)]
pub mod __private {
    #[cfg(target_arch = "aarch64")]
    pub use cortex_a;
    pub use preemption;
    #[cfg(target_arch = "aarch64")]
    pub use tock_registers;
    #[cfg(target_arch = "x86_64")]
    pub use x86_64;
}

struct CpuLocalDataRegion {
    cpu: CpuId,
    _mapped_page: memory::MappedPages,
}

// TODO: Store size of used CLS in gs:[0].
static STATE: SpinMutex<State> = SpinMutex::new(State {
    sections: Vec::new(),
    image: Vec::new(),
    used: 0,
});

struct State {
    sections: Vec<CpuLocalDataRegion>,
    image: Vec<u8>,
    used: usize,
}

pub fn init(cpu: CpuId) {
    use core::arch::asm;
    log::info!("a");

    let page = memory::allocate_pages(1).expect("couldn't allocate page for CLS section");
    log::info!("b");
    let address = page.start_address().value();
    log::info!("c");
    log::error!("(cpu {cpu:?}) allocated page: {page:?}");
    let mapped_page = memory::get_kernel_mmi_ref()
        .unwrap()
        .lock()
        .page_table
        .map_allocated_pages(page, PteFlags::VALID | PteFlags::WRITABLE)
        .unwrap();

    let cls_start_pointer = mapped_page.start_address().value() as *mut u8;

    let mut locked = STATE.lock();
    locked.sections.push(CpuLocalDataRegion {
        cpu,
        _mapped_page: mapped_page,
    });

    unsafe {
        ptr::copy_nonoverlapping(locked.image.as_ptr(), cls_start_pointer, locked.image.len())
    };

    #[cfg(target_arch = "x86_64")]
    {
        use x86_64::registers::control::{Cr4, Cr4Flags};
        unsafe { Cr4::update(|flags| flags.insert(Cr4Flags::FSGSBASE)) };

        unsafe {
            asm!(
                "wrgsbase {}",
                in(reg) address,
                options(nomem, preserves_flags, nostack),
            )
        }
    };
    #[cfg(target_arch = "aarch64")]
    unsafe {
        asm!(
            "msr tpidr_el1, {}",
            in(reg) address,
            options(nomem, preserves_flags, nostack),
        )
    };
    log::info!("done init");
}

pub fn allocate(len: usize, alignment: usize, image: &[u8]) -> usize {
    log::info!("start alloc: {len:0x?} {alignment:0x?}");
    log::info!("{image:0x?}");
    let mut locked = STATE.lock();

    let offset = locked.used.next_multiple_of(alignment);
    let num_alignment_bytes = offset - locked.used;

    assert!(offset + len <= 4096);

    locked
        .image
        .extend(iter::repeat(0).take(num_alignment_bytes));
    locked.image.extend(image);
    locked.used = offset + len;

    log::info!("end alloc");
    offset
}
