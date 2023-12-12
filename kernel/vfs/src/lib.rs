// The tracking issue (#36887) has been open since 2016; I don't think it's
// getting removed any time soon.
#![allow(invalid_type_param_default)]
#![feature(associated_type_defaults)]
#![no_std]

mod implementor;
mod temp;

extern crate alloc;

use alloc::boxed::Box;

struct FileSystem {
    // inner: Box<dyn implementor::FileSystem>,
}