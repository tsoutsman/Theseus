#![no_std]

extern "Rust" {
    #[link_name = "terminal_print::print_to_stdout_args::h2e6f0d1c9e7a336c"]
    pub fn _print_to_stdout_args(fmt_args: core::fmt::Arguments);
}

pub fn print_to_stdout_args(fmt_args: core::fmt::Arguments) {
    unsafe { _print_to_stdout_args(fmt_args) }
}

