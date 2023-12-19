//! A bounded, multi-producer, single-consumer asynchronous channel.
//!
//! See [`Channel`] for more details.

#![no_std]

extern crate alloc;

use alloc::sync::Arc;
use core::{
    pin::Pin,
    task::{Context, Poll},
};

use async_semaphore::Semaphore;
use futures::{
    stream::{FusedStream, Stream},
    task::AtomicWaker,
};
use mpmc::Queue;
use sync::DeadlockPrevention;
use sync_spin::Spin;

#[derive(Clone)]
pub struct Sender<T, P = Spin>
where
    T: Send,
    P: DeadlockPrevention,
{
    inner: Arc<Channel<T, P>>,
}

pub struct Receiver<T, P = Spin>
where
    T: Send,
    P: DeadlockPrevention,
{
    inner: Arc<Channel<T, P>>,
}

impl<T, P> !Sync for Receiver<T, P> {}

pub fn channel<T, P>(capacity: usize) -> (Sender<T, P>, Receiver<T, P>)
where
    T: Send,
    P: DeadlockPrevention,
{
    let inner = Arc::new(Channel::new(capacity));
    (
        Sender {
            inner: inner.clone(),
        },
        Receiver { inner },
    )
}

/// A bounded, multi-producer, single-consumer asynchronous channel.
///
/// The channel can also be used outside of an asynchronous runtime with the
/// [`blocking_send`], and [`blocking_recv`] methods.
///
/// [`blocking_send`]: Self::blocking_send
/// [`blocking_recv`]: Self::blocking_recv
#[derive(Clone)]
struct Channel<T, P = Spin>
where
    T: Send,
    P: DeadlockPrevention,
{
    inner: Queue<T>,
    senders: Semaphore<P>,
    receiver: AtomicWaker,
}

impl<T, P> Sender<T, P>
where
    T: Send,
    P: DeadlockPrevention,
{
    /// Sends `value`.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe, in that if it is dropped prior to
    /// completion, `value` is guaranteed to have not been set. However, in that
    /// case `value` will be dropped.
    pub async fn send(&self, mut value: T) {
        todo!();
    }

    /// Tries to send `value`.
    ///
    /// # Errors
    ///
    /// Returns an error containing `value` if the channel was full.
    pub fn try_send(&self, value: T) -> Result<(), T> {
        self.inner.push(value)?;
        self.inner.receiver.wake();
        Ok(())
    }

    /// Blocks the current thread until `value` is sent.
    pub fn blocking_send(&self, value: T) {
        dreadnought::block_on(self.send(value))
    }
}

impl<T, P> Receiver<T, P>
where
    T: Send,
    P: DeadlockPrevention,
{
    async fn recv(&self) -> T {
        todo!();
    }

    fn try_recv(&self) -> Option<T> {
        self.inner.inner.pop()
    }

    fn blocking_recv(&self) -> T {
        dreadnought::block_on(self.recv())
    }
}
impl<T, P> Stream for Receiver<T, P>
where
    T: Send,
    P: DeadlockPrevention,
{
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        macro_rules! try_recv {
            () => {
                if let Some(value) = self.try_recv() {
                    return Poll::Ready(Some(value));
                }
            };
        }

        try_recv!();
        self.inner.receiver.register(ctx.waker());
        // TODO: Use push_if_fail instead?
        try_recv!();
        Poll::Pending
    }
}

impl<T, P> FusedStream for Receiver<T, P>
where
    T: Send,
    P: DeadlockPrevention,
{
    fn is_terminated(&self) -> bool {
        // NOTE: If we ever implement disconnections, this will need to be modified.
        false
    }
}
