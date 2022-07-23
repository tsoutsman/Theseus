//! This crate contains a condition variable implementation.
//!
//! The implementation is based on the following paper:
//! [http://birrell.org/andrew/papers/ImplementingCVs.pdf]

#![no_std]

use core::sync::atomic::{AtomicUsize, Ordering};

use semaphore::Semaphore;

pub struct Condvar {
    waiters: AtomicUsize,
    semaphore: Semaphore,
}

impl Condvar {
    pub fn new() -> Self {
        Self {
            waiters: AtomicUsize::new(0),
            semaphore: Semaphore::new(0),
        }
    }

    pub fn notify_one(&self) {
        let old_waiters =
            self.waiters
                .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |waiters| {
                    if waiters > 0 {
                        Some(waiters - 1)
                    } else {
                        None
                    }
                });
        // If there was a waiter
        if old_waiters.is_ok() {
            self.semaphore.release();
        }
    }

    pub fn notify_all(&self) {
        let counter = self.waiters.swap(0, Ordering::SeqCst);
        for _ in 0..counter {
            // TODO: Expose function on semaphore that releases `n` times. Currently each
            // call to the function relocks a spinlock.
            self.semaphore.release();
        }
    }

    pub fn wait(&self) {
        self.waiters.fetch_add(1, Ordering::SeqCst);
    }

    pub fn wait_timeout(&self) {}
}
