#![no_std]

extern crate alloc;
extern crate core2;
extern crate keycodes_ascii;
extern crate mutex_sleep;

pub mod key_event;
pub mod stdio;

pub struct IoStreams {
    pub stdin: stdio::Reader,
    pub stdout: stdio::Writer,
    pub stderr: stdio::Writer,
    pub key_events: key_event::Reader,
}
