#![no_std]

extern crate alloc;

use alloc::{string::String, vec::Vec};

pub fn main(_: Vec<String>) -> isize {
    libtheseus::stdio::print_to_stdout_args(format_args!("printing from stub"));
    0
}
