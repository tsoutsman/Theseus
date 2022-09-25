use alloc::{collections::VecDeque, sync::Arc};
use core2::io::{Read, Write};
use mutex_sleep::{MutexSleep as Mutex, MutexSleepGuard as MutexGuard};

pub fn new() -> (Writer, Reader) {
    let handle = Handle::default();
    (Writer { inner: handle.clone() }, Reader { inner: handle })
}

pub fn dummy() -> (Writer, Reader) {
    (
        // TODO: Writing to this writer will still unnecessarily write to inner.bytes.
        Writer {
            inner: Handle {
                inner: Arc::new(Mutex::new(State { bytes: VecDeque::new(), eof: false })),
            },
        },
        Reader {
            inner: Handle {
                inner: Arc::new(Mutex::new(State { bytes: VecDeque::new(), eof: true })),
            },
        },
    )
}

#[derive(Clone, Default)]
struct Handle {
    inner: Arc<Mutex<State>>,
}

#[derive(Clone, Default)]
struct State {
    bytes: VecDeque<u8>,
    eof: bool,
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

impl Read for ReadGuard<'_> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, core2::io::Error> {
        if self.inner.eof || buf.is_empty() {
            Ok(0)
        } else {
            let mut count = 0;

            while let Some(item) = self.inner.bytes.pop_front() {
                buf[count] = item;
                count += 1;

                if count == buf.len() {
                    return Ok(0);
                }
            }

            Ok(count)
        }
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

impl Write for WriteGuard<'_> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, core2::io::Error> {
        if self.inner.eof {
            Err(core2::io::Error::new(
                core2::io::ErrorKind::UnexpectedEof,
                "cannot write to a stream with EOF set",
            ))
        } else {
            self.inner.bytes.extend(buf);
            Ok(buf.len())
        }
    }

    fn flush(&mut self) -> Result<(), core2::io::Error> {
        Ok(())
    }
}
