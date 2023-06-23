#![feature(let_chains)]

use std::{
    env,
    io::{self, Write},
};

fn main() {
    let fork = pty::fork::Fork::from_ptmx().unwrap();
    if let Some(master) = fork.is_parent().ok() {
        let name_ptr = master.ptsname().unwrap();
        // SAFETY: Guaranteed by `ptsname`.
        let name = unsafe { core::ffi::CStr::from_ptr(name_ptr) }.to_str().unwrap();
        println!("{name}");
    }

    // let mut args = env::args();
    // args.next().unwrap();
    // let terminal_command = args.next().unwrap();

    // let mut stdin = io::stdin();
    // let mut buffer = String::new();

    // stdin.read_line(&mut buffer).unwrap();
    // if let Some(remainder) = buffer.strip_prefix("char device redirected to ") {
    //     let tty = remainder.split_ascii_whitespace().next().unwrap();
    //     println!("here: {tty}");
    // }

    // let mut stdout = io::stdout();
    // for line in stdin.lines() {
    //     if let Ok(line) = line {
    //         stdout.write_all(line.as_bytes()).unwrap();
    //     } else {
    //         eprintln!("stdin error");
    //     }
    // }
}
