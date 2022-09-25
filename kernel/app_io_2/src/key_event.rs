use alloc::{collections::VecDeque, sync::Arc};
use keycodes_ascii::KeyEvent;
use mutex_sleep::{MutexSleep as Mutex, MutexSleepGuard as MutexGuard};

struct Handle {
    inner: Arc<Mutex<State>>,
}

struct State {
    events: VecDeque<KeyEvent>,
    // TODO: What does eof mean in this context?
    _eof: bool,
}

pub struct Reader {
    inner: Handle,
}

impl Reader {
    pub fn lock(&self) -> ReadGuard {
        ReadGuard { inner: self.inner.inner.lock().unwrap() }
    }
}

pub struct ReadGuard<'a> {
    inner: MutexGuard<'a, State>,
}

impl ReadGuard<'_> {
    pub fn read_one(&mut self) -> Option<KeyEvent> {
        self.inner.events.pop_front()
    }

    pub fn read_all(&mut self) -> VecDeque<KeyEvent> {
        core::mem::take(&mut self.inner.events)
    }
}

pub struct Writer {
    inner: Handle,
}

impl Writer {
    pub fn lock(&self) -> WriteGuard {
        WriteGuard { inner: self.inner.inner.lock().unwrap() }
    }
}

pub struct WriteGuard<'a> {
    inner: MutexGuard<'a, State>,
}

impl WriteGuard<'_> {
    pub fn write(&mut self, event: KeyEvent) {
        self.inner.events.push_back(event);
    }
}
