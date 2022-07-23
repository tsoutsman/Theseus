//! This crate contains a mutex implementation.

#![no_std]

// FIXME: None of this has been properly checked.

pub struct Mutex {
    locked: AtomicBool,
    queue: WaitQueue,
}

impl Mutex {
    pub fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
            queue: WaitQueue::new(),
        }
    }

    pub fn lock(&self) {
        // If trying to lock and already locked mutex
        if self.locked.swap(true, Ordering::SeqCst) {
            queue.wait().unwrap();
        }
    }

    pub fn unlock(&self) {
        // If noone was waiting for the mutex
        if !self.notify_one() {
            let was_locked = self.locked.swap(false, Ordering::SeqCst);
            debug_assert!(was_locked);
        }
    }

    pub fn try_lock(&self) -> bool {
        self.locked.swap(true, Ordering::SeqCst) == false
    }
}
