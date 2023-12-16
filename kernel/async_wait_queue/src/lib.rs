//! An asynchronous wait queue.
//!
//! See [`WaitQueue`] for more details.

#![no_std]

extern crate alloc;

use alloc::sync::Arc;
use core::{
    future::poll_fn,
    task::{Context, Poll, Waker},
};

use mpmc_queue::Queue;
use sync::DeadlockPrevention;
use sync_spin::Spin;

/// An asynchronous queue of tasks waiting to be notified.
#[derive(Clone)]
pub struct WaitQueue<P = Spin>
where
    P: DeadlockPrevention,
{
    inner: Arc<Queue<Waker, P>>,
}

impl<P> Default for WaitQueue<P>
where
    P: DeadlockPrevention,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<P> WaitQueue<P>
where
    P: DeadlockPrevention,
{
    /// Creates a new empty wait queue.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Queue::new()),
        }
    }

    pub async fn wait_until<F, T>(&self, mut condition: F) -> T
    where
        F: FnMut() -> Option<T>,
    {
        poll_fn(move |context| self.poll_wait_until(context, &mut condition)).await
    }

    pub fn poll_wait_until<F, T>(&self, ctx: &mut Context, condition: &mut F) -> Poll<T>
    where
        F: FnMut() -> Option<T>,
    {
        let wrapped_condition = || {
            if let Some(value) = condition() {
                Ok(value)
            } else {
                Err(())
            }
        };

        match self
            .inner
            .push_if_fail(ctx.waker().clone(), wrapped_condition)
        {
            Ok(value) => Poll::Ready(value),
            Err(()) => Poll::Pending,
        }
    }

    pub fn blocking_wait_until<F, T>(&self, condition: F) -> T
    where
        F: FnMut() -> Option<T>,
    {
        dreadnought::block_on(self.wait_until(condition))
    }

    /// Notifies the first task in the wait queue.
    ///
    /// Returns whether or not a task was awoken.
    pub fn notify_one(&self) -> bool {
        match self.inner.pop() {
            Some(waker) => {
                waker.wake();
                // From the `Waker` documentation:
                // > As long as the executor keeps running and the task is not
                // finished, it is guaranteed that each invocation of `wake()`
                // will be followed by at least one `poll()` of the task to
                // which this `Waker` belongs.
                true
            }
            None => false,
        }
    }

    /// Notifies all the tasks in the wait queue.
    pub fn notify_all(&self) {
        while self.notify_one() {}
    }
}
