#![no_std]

pub type Mutex<T> = sync::Mutex<Spin, T>;
pub type MutexGuard<'a, T> = sync::MutexGuard<'a, Spin, T>;

#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Spin {}

impl sync::DeadlockPrevention for Spin {
    type GuardMarker = sync::GuardSend;

    #[inline]
    fn enter() {}

    #[inline]
    fn exit() {}
}
