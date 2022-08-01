// The tracking issue (#36887) has been open since 2016; I don't think it's
// getting removed any time soon.
#![allow(invalid_type_param_default)]
#![feature(associated_type_defaults)]
#![no_std]

extern crate alloc;

use alloc::{string::String, sync::Arc, vec::Vec};
use memory::MappedPages;
use path::Path;
use spin::Mutex;

pub type NodeRef<Fs, Kind> = Arc<Mutex<dyn Node<Fs, FileSystem = Fs, Kind = Kind>>>;

pub trait Node<T>
where
    T: FileSystem,
{
    type FileSystem: FileSystem<T> = T;

    type Kind: NodeKind;

    fn name(&self) -> String;

    fn parent(&self) -> Option<NodeRef<Self::FileSystem, Directory>>;

    fn absolute_path(&self) -> Path;

    fn as_specific(&self) -> SpecificNodeKind<Self::FileSystem>;
}

pub trait FileNode<T>: Node<T>
where
    T: FileSystem,
{
    fn as_mapping(&self) -> Result<&MappedPages, &'static str>;
}

pub trait DirectoryNode<T>: Node<T>
where
    T: FileSystem,
{
    fn get<Kind = Any>(&self, path: &Path) -> Option<NodeRef<Self::FileSystem, Kind>>
    where
        Kind: NodeKind;

    fn insert<Kind>(
        &mut self,
        node: NodeRef<Self::FileSystem, Kind>,
    ) -> Result<Option<NodeRef<Self::FileSystem, Kind>>, &'static str>
    where
        Kind: NodeKind;

    fn remove<Kind>(
        &mut self,
        node: NodeRef<Self::FileSystem, Kind>,
    ) -> Option<NodeRef<Self::FileSystem, Kind>>
    where
        Kind: NodeKind;

    fn list(&self) -> Vec<NodeRef<Self::FileSystem, Any>>;
}

pub struct Any;
impl private::Sealed for Any {}
impl<T> FileSystem<T> for Any where T: FileSystem {}

impl NodeKind for Any {
    fn from_any<Fs>(node: NodeRef<Fs, Any>) -> Option<NodeRef<Fs, Self>>
    where
        Fs: FileSystem,
    {
        Some(node)
    }
}

pub struct Directory;

impl private::Sealed for Directory {}
impl NodeKind for Directory {
    fn from_any<Fs>(node: NodeRef<Fs, Any>) -> Option<NodeRef<Fs, Self>>
    where
        Fs: FileSystem,
    {
        match node.lock().as_specific() {
            SpecificNodeKind::Directory(d) => Some(d),
            _ => None,
        }
    }
}

pub struct File;

impl private::Sealed for File {}
impl NodeKind for File {
    fn from_any<Fs>(node: NodeRef<Fs, Any>) -> Option<NodeRef<Fs, Self>>
    where
        Fs: FileSystem,
    {
        match node.lock().as_specific() {
            SpecificNodeKind::File(f) => Some(f),
            _ => None,
        }
    }
}

pub trait DirectoryGetKind: private::Sealed {}

pub trait NodeKind: private::Sealed {
    fn from_any<Fs>(node: NodeRef<Fs, Any>) -> Option<NodeRef<Fs, Self>>
    where
        Fs: FileSystem;
}

pub enum SpecificNodeKind<Fs> {
    File(NodeRef<Fs, File>),
    Directory(NodeRef<Fs, Directory>),
}

pub trait FileSystem<Allowed = Self>
where
    Allowed: FileSystem,
{
}

mod private {
    pub trait Sealed {}
}
