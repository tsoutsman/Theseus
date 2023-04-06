#![no_std]
#![feature(maybe_uninit_slice)]

extern crate alloc;

use boot_info::MemoryRegion;
use core::{
    alloc::Layout,
    cmp::max,
    mem::MaybeUninit,
    ops::Deref,
    slice,
    sync::atomic::{AtomicU64, AtomicUsize, Ordering},
};
use kernel_config::memory::FRAME_ALLOCATOR_MEMORY_START;
use memory_structs::{Frame, FrameRange, PhysicalAddress, VirtualAddress};
use page_allocator::{allocate_pages_by_bytes_at, AllocatedPages};
use spin::Once;

/// If the ith bit of the nth element is set, physical frame (n * 8) + i is
/// allocated.
// TODO: Use CacheDistributedArray?
// TODO: Demand page esque array?
static ALLOCATOR: Once<Allocator> = Once::new();
static NEXT_AVAILABLE: AtomicUsize = AtomicUsize::new(0);

struct Allocator {
    frames: &'static [AtomicU64],
    reserved_frames: &'static [AtomicU64],
}

/// Initialises the frame allocator.
///
/// The trickiest part about the frame allocator is allocating the bitmaps.
pub fn init<R, I, F>(memory_regions: I, mapper: F)
where
    R: MemoryRegion,
    I: Clone + Iterator<Item = R>,
    F: FnOnce(AllocatedPages, AllocatedFrames),
{
    ALLOCATOR.call_once(|| {
        let mut max_address = PhysicalAddress::ZERO;

        for region in memory_regions.clone() {
            if region.is_usable() {
                max_address = max(max_address, region.start() + region.len());
            } else {
                max_address = max(max_address, region.start() + region.len());
            }
        }

        // FIXME: Rounding
        let array_len = max_address.value() / core::mem::size_of::<AtomicU64>();
        let (layout, offset) = Layout::array::<AtomicU64>(array_len)
            .unwrap()
            .extend(Layout::array::<AtomicU64>(array_len).unwrap())
            .unwrap();

        // We need to manually allocate the bitmap arrays because alloc depends on frame
        // allocator (which is currently being initialised).

        let allocated_pages = allocate_pages_by_bytes_at(
            VirtualAddress::new_canonical(FRAME_ALLOCATOR_MEMORY_START),
            layout.size(),
        )
        .unwrap();
        let mut large_enough_region = None;
        for region in memory_regions.clone() {
            if region.is_usable() && region.len() >= layout.size() {
                large_enough_region = Some(region);
                break;
            }
        }
        let large_enough_region = large_enough_region.unwrap();

        // TODO: These don't need to be contiguous.
        let allocated_frames = AllocatedFrames {
            inner: FrameRange::from_phys_addr(large_enough_region.start(), layout.size()),
        };

        mapper(allocated_pages, allocated_frames);

        let uninit_frames = unsafe {
            slice::from_raw_parts_mut::<MaybeUninit<AtomicU64>>(
                FRAME_ALLOCATOR_MEMORY_START as *mut _,
                array_len,
            )
        };
        let uninit_reserved_frames = unsafe {
            slice::from_raw_parts_mut::<MaybeUninit<AtomicU64>>(
                (FRAME_ALLOCATOR_MEMORY_START + offset) as *mut _,
                array_len,
            )
        };

        for i in uninit_frames.iter_mut() {
            i.write(AtomicU64::new(0));
        }
        for i in uninit_reserved_frames.iter_mut() {
            i.write(AtomicU64::new(0));
        }

        Allocator {
            // SAFETY: We just initialised these values.
            frames: unsafe { MaybeUninit::slice_assume_init_ref(uninit_frames) },
            // SAFETY: We just initialised these values.
            reserved_frames: unsafe { MaybeUninit::slice_assume_init_ref(uninit_reserved_frames) },
        }
    });
}

#[derive(Debug)]
pub struct AllocatedFrame {
    inner: Frame,
}

impl Deref for AllocatedFrame {
    type Target = Frame;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl TryFrom<AllocatedFrames> for AllocatedFrame {
    type Error = &'static str;

    fn try_from(value: AllocatedFrames) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl AllocatedFrame {
    // FIXME: Rename to start.
    pub fn start_address(&self) -> PhysicalAddress {
        todo!();
    }

    pub fn to_frame(&self) -> Frame {
        self.inner
    }
}

#[derive(Debug)]
pub struct AllocatedFrames {
    inner: FrameRange,
}

impl Deref for AllocatedFrames {
    type Target = FrameRange;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<AllocatedFrame> for AllocatedFrames {
    fn from(value: AllocatedFrame) -> Self {
        todo!()
    }
}

impl AllocatedFrames {
    pub fn empty() -> Self {
        todo!();
    }

    pub fn start_address(&self) -> PhysicalAddress {
        todo!();
    }

    pub fn start(&self) -> Frame {
        todo!();
    }

    pub fn end(&self) -> Frame {
        todo!();
    }

    pub fn as_frame(&self) -> &AllocatedFrame {
        assert!(self.inner.size_in_frames() == 1);
        todo!();
    }
}

pub fn allocate_frame() -> Option<AllocatedFrame> {
    let allocator = ALLOCATOR.get()?;
    let mut next = NEXT_AVAILABLE.load(Ordering::Relaxed);

    loop {
        let next_index = next / 8;
        let next_remainder = next % 8;

        if allocator.reserved_frames[next_index].load(Ordering::SeqCst) >> next_remainder == 0 {
            let old_value =
                allocator.frames[next_index].fetch_or(1 << next_remainder, Ordering::SeqCst);

            if old_value >> next_remainder == 0 {
                todo!("set next available and return allocated frame");
            }

            next = next % 8 + old_value.trailing_ones() as usize;
            todo!("check if next overflowed the number of physical frames");
        }
    }
}

pub fn allocate_frames(bytes: usize) -> Result<AllocatedFrames, &'static str> {
    todo!();
}

pub fn allocate_reserved_frame(start: PhysicalAddress) -> Result<AllocatedFrame, &'static str> {
    todo!();
}

pub fn allocate_reserved_frames(
    start: PhysicalAddress,
    bytes: usize,
) -> Result<AllocatedFrames, &'static str> {
    todo!();
}

pub fn dump_frame_allocator_state() {
    todo!();
}
