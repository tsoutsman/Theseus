//! A simple library that handles stdio queues for applications running in
//! terminal instances.
//!
//! This provides some APIs similar to Rust's `std::io` for applications to
//! access those queues.
//!
//! Usage example:
//! 1. shell spawns a new app, and creates queues of `stdin`, `stdout`, and
//! `stderr` for that app. 2. shell stores the reader for `stdin` and writers
//! for `stdout` and `stderr` to `app_io`,    along with the reader of the key
//! events queue and references to the running terminal instance. 3. app calls
//! [`stdin()`] to get the reader of `stdin`, and can perform reading just like
//!    using the standard library
//! 4. app calls [`stdout()`] to get the writer of `stdin`, and can perform
//! output just like    using the standard library
//! 5. after app exits, shell would set `EOF` flags to its `stdin`, `stdout`,
//! and `stderr` queues. 6. once all apps in a job exit, app shell removes all
//! the structure stored in `app_io` and    destructs all stdio queues

#![no_std]

extern crate alloc;

use alloc::{format, sync::Arc};
use core2::io::{self, Error, ErrorKind, Read, Write};
use stdio::{StdioReader, StdioWriter};
use tty::{LineDiscipline, Slave};

pub trait ImmutableRead: Send + Sync + 'static {
    fn read(&self, buf: &mut [u8]) -> io::Result<usize>;
}

pub trait ImmutableWrite: Send + Sync + 'static {
    fn write(&self, buf: &[u8]) -> io::Result<usize>;

    fn write_all(&self, mut buf: &[u8]) -> io::Result<()> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => {
                    return Err(Error::new(
                        ErrorKind::WriteZero,
                        "failed to write whole buffer",
                    ));
                }
                Ok(n) => buf = &buf[n..],
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

impl ImmutableRead for StdioReader {
    fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.lock().read(buf)
    }
}

impl ImmutableWrite for StdioWriter {
    fn write(&self, buf: &[u8]) -> io::Result<usize> {
        self.lock().write(buf)
    }
}

impl ImmutableRead for Slave {
    fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.read(buf)
    }
}

impl ImmutableWrite for Slave {
    fn write(&self, buf: &[u8]) -> io::Result<usize> {
        self.write(buf)
    }
}

/// Stores the stdio queues, key event queue and the pointer to the terminal
/// for applications. This structure is provided for application's use and only
/// contains necessary one-end readers/writers to queues. On the shell side, we
/// have full control to queues.
#[derive(Clone)]
pub struct IoStreams {
    /// The reader to stdin.
    pub stdin: Arc<dyn ImmutableRead>,
    /// The writer to stdout.
    pub stdout: Arc<dyn ImmutableWrite>,
    /// The writer to stderr.
    pub stderr: Arc<dyn ImmutableWrite>,
    pub discipline: Option<Arc<LineDiscipline>>,
}

mod shared_maps {
    use super::IoStreams;
    use hashbrown::HashMap;
    use mutex_sleep::{MutexSleep as Mutex, MutexSleepGuard as MutexGuard};

    lazy_static::lazy_static! {
        /// Map a task id to its IoStreams structure.
        /// Shells should call `insert_child_streams` when spawning a new app,
        /// which effectively stores a new key value pair to this map.
        /// After a shell's child app exits, the shell should call
        /// `remove_child_streams` to clean it up.
        static ref APP_IO_STREAMS: Mutex<HashMap<usize, IoStreams>> = Mutex::new(HashMap::new());
    }

    /// Lock and returns the `MutexGuard` of `APP_IO_STREAMS`. Use
    /// `lock_all_maps()` if you want to lock both of the maps to avoid
    /// deadlock.
    pub fn lock_stream_map() -> MutexGuard<'static, HashMap<usize, IoStreams>> {
        APP_IO_STREAMS.lock().unwrap()
    }
}

/// Shells call this function to store queue stdio streams for applications. If
/// there are any existing readers/writers for the task (which should not
/// happen in normal practice), it returns the old one, otherwise returns None.
pub fn insert_child_streams(task_id: usize, streams: IoStreams) -> Option<IoStreams> {
    shared_maps::lock_stream_map().insert(task_id, streams)
}

/// Shells call this function to remove queues and pointer to terminal for
/// applications. It returns the removed streams in the return value if the key
/// matches, otherwise returns None.
pub fn remove_child_streams(task_id: usize) -> Option<IoStreams> {
    shared_maps::lock_stream_map().remove(&task_id)
}

pub fn streams() -> Result<IoStreams, &'static str> {
    let task_id = task::get_my_current_task_id();
    let locked_streams = shared_maps::lock_stream_map();
    match locked_streams.get(&task_id) {
        Some(streams) => Ok(streams.clone()),
        None => Err("no stdin for this task"),
    }
}

/// Applications call this function to acquire a reader to its stdin queue.
///
/// Errors can occur in two cases. One is when it fails to get the task_id of
/// the calling task, and the second is that there's no stdin reader stored in
/// the map. Shells should make sure to store IoStreams for the newly spawned
/// app first, and then unblocks the app to let it run.
pub fn stdin() -> Result<Arc<dyn ImmutableRead>, &'static str> {
    let task_id = task::get_my_current_task_id();
    let locked_streams = shared_maps::lock_stream_map();
    match locked_streams.get(&task_id) {
        Some(queues) => Ok(queues.stdin.clone()),
        None => Err("no stdin for this task"),
    }
}

/// Applications call this function to acquire a writer to its stdout queue.
///
/// Errors can occur in two cases. One is when it fails to get the task_id of
/// the calling task, and the second is that there's no stdout writer stored in
/// the map. Shells should make sure to store IoStreams for the newly spawned
/// app first, and then unblocks the app to let it run.
pub fn stdout() -> Result<Arc<dyn ImmutableWrite>, &'static str> {
    let task_id = task::get_my_current_task_id();
    let locked_streams = shared_maps::lock_stream_map();
    match locked_streams.get(&task_id) {
        Some(queues) => Ok(queues.stdout.clone()),
        None => Err("no stdout for this task"),
    }
}

/// Applications call this function to acquire a writer to its stderr queue.
///
/// Errors can occur in two cases. One is when it fails to get the task_id of
/// the calling task, and the second is that there's no stderr writer stored in
/// the map. Shells should make sure to store IoStreams for the newly spawned
/// app first, and then unblocks the app to let it run.
pub fn stderr() -> Result<Arc<dyn ImmutableWrite>, &'static str> {
    let task_id = task::get_my_current_task_id();
    let locked_streams = shared_maps::lock_stream_map();
    match locked_streams.get(&task_id) {
        Some(queues) => Ok(queues.stderr.clone()),
        None => Err("no stderr for this task"),
    }
}

pub fn line_discipline() -> Result<Arc<LineDiscipline>, &'static str> {
    let task_id = task::get_my_current_task_id();
    let locked_streams = shared_maps::lock_stream_map();
    match locked_streams.get(&task_id) {
        Some(IoStreams {
            discipline: Some(discipline),
            ..
        }) => Ok(discipline.clone()),
        _ => Err("no line discipline for this task"),
    }
}

/// Calls `print!()` with an extra newline ('\n') appended to the end.
#[macro_export]
macro_rules! println {
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));

}

/// The main printing macro, which simply writes to the current task's stdout
/// stream.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::print_to_stdout_args(format_args!($($arg)*));
    });
}

/// Converts the given `core::fmt::Arguments` to a `String` and enqueues the
/// string into the correct terminal print-producer
pub fn print_to_stdout_args(fmt_args: core::fmt::Arguments) {
    let task_id = task::get_my_current_task_id();

    // Obtains the correct stdout stream and push the output bytes.
    let locked_streams = shared_maps::lock_stream_map();
    match locked_streams.get(&task_id) {
        Some(queues) => {
            if queues
                .stdout
                .write_all(format!("{}", fmt_args).as_bytes())
                .is_err()
            {
                let _ = logger::write_str("\x1b[31m [E] failed to write to stdout \x1b[0m\n");
            }
            let _ = logger::write_str("\x1b[31m [E] wrote to stdout \x1b[0m\n");
        }
        None => {
            let _ = logger::write_str("\x1b[31m [E] error in print!/println! macro: no stdout queue for current task \x1b[0m\n");
        }
    };
}
