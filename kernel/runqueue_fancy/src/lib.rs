//! This code is *heavily* based on Tokio's multithreaded runqueue.

#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use core::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::atomic::{AtomicU32, AtomicU64, Ordering},
};
use task::TaskRef;

const LOCAL_QUEUE_CAPACITY: usize = 256;
const MASK: usize = LOCAL_QUEUE_CAPACITY - 1;

pub struct Inner {
    head: AtomicU64,
    tail: AtomicU32,
    // TODO: Unbox since queue can be unsized.
    buffer: Box<[UnsafeCell<MaybeUninit<TaskRef>>; 256]>,
}

impl Inner {
    fn is_stealable(&self) -> bool {
        todo!();
    }

    /// Pushes a task to the back of the local queue, skipping the LIFO slot.
    pub(crate) fn push_back(&mut self, mut task: TaskRef, inject: &Inject<T>) {
        let tail = loop {
            let head = self.head.load(Ordering::Acquire);
            let (steal, real) = unpack(head);

            // TODO: This can be an unsync load.
            let tail = self.tail.load(Ordering::SeqCst);

            if tail.wrapping_sub(steal) < LOCAL_QUEUE_CAPACITY as u32 {
                // There is capacity for the task
                break tail;
            } else if steal != real {
                // Concurrently stealing, this will free up capacity, so only
                // push the task onto the inject queue
                inject.push(task);
                return;
            } else {
                // Push the current task and half of the queue into the
                // inject queue.
                match self.push_overflow(task, real, tail, inject) {
                    Ok(_) => return,
                    // Lost the race, try again
                    Err(v) => {
                        task = v;
                    }
                }
            }
        };

        // Map the position to a slot index.
        let idx = tail as usize & MASK;

        // Write the task to the slot
        //
        // Safety: There is only one producer and the above `if`
        // condition ensures we don't touch a cell if there is a
        // value, thus no consumer.
        unsafe {
            // TODO: Why do we need the dereference followed by as_mut_ptr immediately?
            core::ptr::write((*self.buffer[idx].get_mut()).as_mut_ptr(), task);
        }

        // Make the task available. Synchronizes with a load in
        // `steal_into2`.
        self.tail.store(tail.wrapping_add(1), Ordering::Release);
    }

    fn push_overflow(
        &mut self,
        task: TaskRef,
        head: u32,
        tail: u32,
        inject: &Inject<T>,
    ) -> Result<(), TaskRef> {
        /// How many elements are we taking from the local queue.
        ///
        /// This is one less than the number of tasks pushed to the inject
        /// queue as we are also inserting the `task` argument.
        const NUM_TASKS_TAKEN: u32 = (LOCAL_QUEUE_CAPACITY / 2) as u32;

        assert_eq!(
            tail.wrapping_sub(head) as usize,
            LOCAL_QUEUE_CAPACITY,
            "queue is not full; tail = {}; head = {}",
            tail,
            head
        );

        let prev = pack(head, head);

        // Claim a bunch of tasks
        //
        // We are claiming the tasks **before** reading them out of the buffer.
        // This is safe because only the **current** thread is able to push new
        // tasks.
        //
        // There isn't really any need for memory ordering... Relaxed would
        // work. This is because all tasks are pushed into the queue from the
        // current thread (or memory has been acquired if the local queue handle
        // moved).
        if self
            .head
            .compare_exchange(
                prev,
                pack(
                    head.wrapping_add(NUM_TASKS_TAKEN),
                    head.wrapping_add(NUM_TASKS_TAKEN),
                ),
                Ordering::Release,
                Ordering::Relaxed,
            )
            .is_err()
        {
            // We failed to claim the tasks, losing the race. Return out of
            // this function and try the full `push` routine again. The queue
            // may not be full anymore.
            return Err(task);
        }

        /// An iterator that takes elements out of the run queue.
        struct BatchTaskIter<'a> {
            buffer: &'a [UnsafeCell<MaybeUninit<TaskRef>>; LOCAL_QUEUE_CAPACITY],
            head: u64,
            i: u64,
        }

        impl<'a> Iterator for BatchTaskIter<'a> {
            type Item = TaskRef;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                if self.i == u64::from(NUM_TASKS_TAKEN) {
                    None
                } else {
                    let i_idx = self.i.wrapping_add(self.head) as usize & MASK;
                    let slot = &self.buffer[i_idx];

                    // safety: Our CAS from before has assumed exclusive ownership
                    // of the task pointers in this range.
                    // TODO: Why do we need the dereference followed by as_ptr immediately?
                    let task = unsafe { core::ptr::read((*slot.get()).as_ptr()) };

                    self.i += 1;
                    Some(task)
                }
            }
        }

        // safety: The CAS above ensures that no consumer will look at these
        // values again, and we are the only producer.
        let batch_iter = BatchTaskIter {
            buffer: &*self.buffer,
            head: head as u64,
            i: 0,
        };
        inject.push_batch(batch_iter.chain(core::iter::once(task)));

        Ok(())
    }
}

/// Split the head value into the real head and the index a stealer is working
/// on.
fn unpack(n: u64) -> (u32, u32) {
    let real = n & u32::MAX as u64;
    let steal = n >> (core::mem::size_of::<u32>() * 8);

    (steal as u32, real as u32)
}

/// Join the two head values
fn pack(steal: u32, real: u32) -> u64 {
    (real as u64) | ((steal as u64) << (core::mem::size_of::<u32>() * 8))
}
