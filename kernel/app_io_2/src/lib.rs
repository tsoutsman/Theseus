#![no_std]
#![feature(try_blocks)]

extern crate alloc;
extern crate core2;
extern crate keycodes_ascii;
extern crate mutex_sleep;
extern crate task;

pub mod key_event;
pub mod stdio;

use hashbrown::HashMap;
use mutex_sleep::MutexSleep as Mutex;

lazy_static::lazy_static! {
    static ref IO_STREAMS: Mutex<HashMap<usize, IoStreams>> = Mutex::new(HashMap::new());
}

pub struct IoStreams {
    pub stdin: stdio::Reader,
    pub stdout: stdio::Writer,
    pub stderr: stdio::Writer,
    pub key_events: key_event::Reader,
}

pub fn stdin() -> stdio::Reader {
    let reader: Option<stdio::Reader> = try {
        let task_id = task::get_my_current_task_id()?;
        IO_STREAMS.lock().ok()?.get(&task_id)?.stdin.clone()
    };
    reader.unwrap_or_else(stdio::Reader::dummy)
}

pub fn stdout() -> stdio::Writer {
    let writer: Option<stdio::Writer> = try {
        let task_id = task::get_my_current_task_id()?;
        IO_STREAMS.lock().ok()?.get(&task_id)?.stdout.clone()
    };
    writer.unwrap_or_else(stdio::Writer::dummy)
}

pub fn stderr() -> stdio::Writer {
    let writer: Option<stdio::Writer> = try {
        let task_id = task::get_my_current_task_id()?;
        IO_STREAMS.lock().ok()?.get(&task_id)?.stdout.clone()
    };
    writer.unwrap_or_else(stdio::Writer::dummy)
}

pub fn insert_streams(task_id: usize, streams: IoStreams) -> Option<IoStreams> {
    IO_STREAMS.lock().ok()?.insert(task_id, streams)
}

pub fn remove_streams(task_id: usize) -> Option<IoStreams> {
    IO_STREAMS.lock().ok()?.remove(&task_id)
}
