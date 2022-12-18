#![feature(new_uninit)]
#![no_std]

extern crate alloc;

use core::mem::MaybeUninit;

use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};
use hashbrown::HashMap;
use spin::RwLock;

// A vec is more performant than a hashmap for small sizes.
static FILE_SYSTEMS: RwLock<Vec<(String, Arc<MountedFileSystem>)>> = RwLock::new(Vec::new());

pub fn mount<T>(name: String, file_system: T) -> Result<(), ()>
where
    T: FileSystem + 'static,
{
    let mut file_systems = FILE_SYSTEMS.write();
    for (existing_name, _) in file_systems.iter() {
        if *existing_name == name {
            return Err(());
        }
    }
    file_systems.push((
        name,
        Arc::new(MountedFileSystem {
            opened: HashMap::new(),
            inner: Box::new(file_system),
        }),
    ));
    Ok(())
}

pub fn unmount(name: &str) {
    todo!();
}

pub struct MountedFileSystem {
    opened: HashMap<String, Arc<dyn File>>,
    // TODO: This doesn't technically have to be wrapped in a box because MountedFileSystem is
    // always stored on the heap and can hence be a DST, but constructing DSTs with a trait object
    // as the last field isn't currently possible AFAICT.
    inner: Box<dyn FileSystem>,
}

impl MountedFileSystem {
    //
}

pub trait FileSystem: Send + Sync {
    fn root(&self) -> dyn Directory;
}

/// A file.
///
/// The methods don't take a mutable reference to `self` as types implementing
/// `File` should use interior mutability.
pub trait File: Node + Send + Sync {
    /// Read some bytes from the file into the specified buffer, returning how
    /// many bytes were read.
    fn read(&self, buffer: &mut [u8]) -> Result<usize, &'static str>;

    /// Write a buffer into this writer, returning how many bytes were written.
    fn write(&self, buffer: &[u8]) -> Result<usize, &'static str>;

    /// Seek to an offset, in bytes, in a stream.
    ///
    /// A seek beyond the end of a stream is allowed, but behaviour is defined
    /// by the implementation.
    ///
    /// If the seek operation completed successfully, this method returns the
    /// new position from the start of the stream.
    fn seek(&self, pos: SeekFrom) -> Result<usize, &'static str>;
}

/// Enumeration of possible methods to seek within a file.
///
/// It is used by the [`File::seek`] method.
pub enum SeekFrom {
    /// Sets the offset to the provided number of bytes.
    Start(usize),

    /// Sets the offset to the size of this object plus the specified number of
    /// bytes.
    ///
    /// It is possible to seek beyond the end of an object, but it's an error to
    /// seek before byte 0.
    End(isize),

    /// Sets the offset to the current position plus the specified number of
    /// bytes.
    ///
    /// It is possible to seek beyond the end of an object, but it's an error to
    /// seek before byte 0.
    Current(isize),
}
pub trait Directory {}
