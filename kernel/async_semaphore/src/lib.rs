#![no_std]

use core::{
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    task::{Context, Poll, Waker},
};

use mpmc_queue::Queue;
use sync::DeadlockPrevention;
use sync_spin::Spin;

/// An asynchronous counting semaphore.
///
/// A counting semaphore maintains a number of permits. When [`acquire`] is
/// called, if there are enough permits, the function immediately returns a
/// [`Permit`]. Otherwise, it asynchronously waits for a [`Permit`] to become
/// available.
///
/// # Examples
///
/// ```rust
/// # use async_semaphore::Semaphore;
///
/// # fn main() {
/// # let rt = tokio::runtime::Builder::new_current_thread().build().unwrap();
/// # rt.block_on(async {
/// let semaphore = Semaphore::new(2);
///
/// let permit_a = semaphore.acquire().await;
/// let permit_b = semaphore.acquire().await;
///
/// assert!(semaphore.try_acquire().is_none());
///
/// drop(permit_b);
///
/// assert!(semaphore.try_acquire().is_some());
/// # });
/// # }
/// ```
pub struct Semaphore<P = Spin>
where
    P: DeadlockPrevention,
{
    permits: AtomicUsize,
    queue: Queue<Node, P>,
}

struct Node {
    is_woken: AtomicBool,
    waker: Waker,
}

impl Semaphore {
    pub const fn new(permits: usize) -> Self {
        Self {
            permits: AtomicUsize::new(permits),
            queue: Queue::new(),
        }
    }

    pub const fn acquire(&self) -> Acquire<'_> {
        Acquire {
            semaphore: self,
            queued: false,
        }
    }

    pub fn try_acquire(&self) -> Option<Acquire<'_>> {
        todo!();
    }

    pub fn release(&self) {
        todo!();
    }
}

#[must_use]
pub struct Acquire<'a> {
    semaphore: &'a Semaphore,
    queued: bool,
}

impl<'a> Future for Acquire<'a> {
    type Output = Permit<'a>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut current = self.semaphore.permits.load(Ordering::Acquire);

        loop {
            if current > 0 {
                match self.semaphore.permits.compare_exchange(
                    current,
                    current - 1,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                ) {
                    Ok(_) => {
                        return Poll::Ready(Permit {
                            semaphore: self.semaphore,
                        })
                    }
                    Err(next) => current = next,
                }
            } else {
                todo!("enqueue self");
            }
        }
    }
}

impl Drop for Acquire<'_> {
    fn drop(&mut self) {
        // If the future is dropped, before we acquire the semaphore, we must remove
        // ourselves from the wait queue.

        if !self.queued {
            return;
        }

        todo!();
    }
}

pub struct Permit<'a> {
    semaphore: &'a Semaphore,
}

impl Drop for Permit<'_> {
    fn drop(&mut self) {
        self.semaphore.release();
    }
}
