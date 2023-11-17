//! Provides Theseus OS functionality without a direct dependency on Theseus
//! kernel crates.
//!
//! It does so by declaring functions that are implemented by the `libtheseus`
//! kernel crate. These functions are referenced as relocations in the dependent
//! (i.e. `std`) object file that `mod_mgmt` then fills in at runtime.

#![no_std]
#![feature(extern_types)]

#[cfg(not(feature = "rustc-dep-of-std"))]
extern crate alloc;

use alloc::string::String;

pub use theseus_ffi::{
    Error, FatPointer, FfiOption, FfiResult, FfiSlice, FfiSliceMut, FfiStr, FfiString,
};

type Result<T> = core::result::Result<T, Error>;

#[no_mangle]
extern "C" fn _Unwind_Resume(arg: usize) -> ! {
    loop {}
}

#[link_name = "libtheseus::next_u64"]
pub fn next_u64() -> u64 {
    unreachable!()
}

#[link_name = "libtheseus::getcwd"]
pub fn getcwd() -> FfiString {
    unreachable!()
}

#[link_name = "libtheseus::chdir"]
pub fn chdir(path: FfiStr<'_>) -> FfiResult<(), Error> {
    unreachable!()
}

#[link_name = "libtheseus::getenv"]
pub fn getenv(key: FfiStr<'_>) -> FfiOption<FfiString> {
    unreachable!()
}

#[link_name = "libtheseus::setenv"]
pub fn setenv(key: FfiStr<'_>, value: FfiStr<'_>) -> FfiResult<(), Error> {
    unreachable!()
}

#[link_name = "libtheseus::unsetenv"]
pub fn unsetenv(key: FfiStr<'_>) -> FfiResult<(), Error> {
    unreachable!()
}

#[link_name = "libtheseus::exit"]
pub fn exit(code: i32) -> ! {
    unreachable!()
}

#[link_name = "libtheseus::getpid"]
pub fn getpid() -> u32 {
    unreachable!()
}

#[link_name = "libtheseus::register_dtor"]
pub fn register_dtor(t: *mut u8, dtor: unsafe extern "C" fn(*mut u8)) {
    unreachable!()
}

#[link_name = "libtheseus::stdin"]
pub fn stdin() -> FfiResult<FatPointer, Error> {
    unreachable!()
}

#[link_name = "libtheseus::stdout"]
pub fn stdout() -> FfiResult<FatPointer, Error> {
    unreachable!()
}

#[link_name = "libtheseus::stderr"]
pub fn stderr() -> FfiResult<FatPointer, Error> {
    unreachable!()
}

#[link_name = "libtheseus::read"]
pub fn read(reader: FatPointer, buf: FfiSliceMut<'_, u8>) -> FfiResult<usize, Error> {
    unreachable!()
}

#[link_name = "libtheseus::write"]
pub fn write(writer: FatPointer, buf: FfiSlice<'_, u8>) -> FfiResult<usize, Error> {
    unreachable!()
}

#[link_name = "libtheseus::flush"]
pub fn flush(writer: FatPointer) -> FfiResult<(), Error> {
    unreachable!()
}

#[link_name = "libtheseus::drop_reader"]
pub fn drop_reader(reader: FatPointer) {
    unreachable!()
}

#[link_name = "libtheseus::drop_writer"]
pub fn drop_writer(writer: FatPointer) {
    unreachable!()
}

// const _: theseus_ffi::next_u64 = next_u64;
// const _: theseus_ffi::getcwd = getcwd;
// const _: theseus_ffi::chdir = chdir;
// const _: theseus_ffi::getenv = getenv;
// const _: theseus_ffi::setenv = setenv;
// const _: theseus_ffi::unsetenv = unsetenv;
// const _: theseus_ffi::exit = exit;
// const _: theseus_ffi::getpid = getpid;
// const _: theseus_ffi::register_dtor = register_dtor;
// const _: theseus_ffi::stdin = stdin;
// const _: theseus_ffi::stdout = stdout;
// const _: theseus_ffi::stderr = stderr;
// const _: theseus_ffi::read = read;
// const _: theseus_ffi::write = write;
// const _: theseus_ffi::flush = flush;
// const _: theseus_ffi::drop_reader = drop_reader;
// const _: theseus_ffi::drop_writer = drop_writer;
