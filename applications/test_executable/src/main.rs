#![no_std]
#![feature(start)]

use app_io::println;
use heap as _;
use panic_entry as _;

#[start]
fn _start(_: isize, _: *const *const u8) -> isize {
    main();
    0
}

fn main() {
    println!("Hello, world!");
}
