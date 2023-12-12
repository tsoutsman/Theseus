// #![no_std]
//
// extern crate alloc;
//
// use alloc::{string::String, sync::Arc, vec::Vec};
// use memory::MappedPages;
// use path::{Path, PathBuf};
// use spin::Mutex;
//
// pub type NodeRef<Fs, Kind> = Arc<dyn Node<Fs, FileSystem = Fs, Kind = Kind>>;
//
// impl<T, Fs> Node<dyn FileSystem> for T
// where
//     T: Node<Fs>,
// {
//     type Kind = ();
//
//     fn name(&self) -> String {
//         todo!()
//     }
//
//     fn parent(&self) -> Option<NodeRef<Self::FileSystem, Directory>> {
//         todo!()
//     }
//
//     fn absolute_path(&self) -> PathBuf {
//         todo!()
//     }
//
//     fn as_specific(&self) -> SpecificNodeKind<Self::FileSystem> {
//         todo!()
//     }
// }
//
// pub trait Node<T>
// where
//     T: FileSystem,
// {
//     type FileSystem: FileSystem = T;
//
//     type Kind: NodeKind;
//
//     fn name(&self) -> String;
//
//     fn parent(&self) -> Option<NodeRef<Self::FileSystem, Directory>>;
//
//     fn absolute_path(&self) -> PathBuf;
//
//     fn as_specific(&self) -> SpecificNodeKind<Self::FileSystem>;
// }
//
// pub trait FileNode<T>: Node<T>
// where
//     T: FileSystem,
// {
//     fn as_mapping(&self) -> Result<&MappedPages, &'static str>;
// }
//
// pub trait DirectoryNode<T>: Node<T>
// where
//     T: FileSystem,
// {
//     fn get<Kind = Any>(&self, path: &Path) -> Option<NodeRef<Self::FileSystem, Kind>>
//     where
//         Kind: NodeKind;
//
//     fn insert<Kind>(
//         &mut self,
//         node: NodeRef<Self::FileSystem, Kind>,
//     ) -> Result<Option<NodeRef<Self::FileSystem, Kind>>, &'static str>
//     where
//         Kind: NodeKind;
//
//     fn remove<Kind>(
//         &mut self,
//         node: NodeRef<Self::FileSystem, Kind>,
//     ) -> Option<NodeRef<Self::FileSystem, Kind>>
//     where
//         Kind: NodeKind;
//
//     fn list(&self) -> Vec<NodeRef<Self::FileSystem, Any>>;
// }
//
// pub struct Any;
// impl private::Sealed for Any {}
//
// impl NodeKind for Any {
//     fn from_any<Fs>(node: NodeRef<Fs, Any>) -> Option<NodeRef<Fs, Self>>
//     where
//         Fs: FileSystem,
//     {
//         Some(node)
//     }
// }
//
// pub struct Directory;
//
// impl private::Sealed for Directory {}
// impl NodeKind for Directory {
//     fn from_any<Fs>(node: NodeRef<Fs, Any>) -> Option<NodeRef<Fs, Self>>
//     where
//         Fs: FileSystem,
//     {
//         match node.lock().as_specific() {
//             SpecificNodeKind::Directory(d) => Some(d),
//             _ => None,
//         }
//     }
// }
//
// pub struct File;
//
// impl private::Sealed for File {}
// impl NodeKind for File {
//     fn from_any<Fs>(node: NodeRef<Fs, Any>) -> Option<NodeRef<Fs, Self>>
//     where
//         Fs: FileSystem,
//     {
//         match node.lock().as_specific() {
//             SpecificNodeKind::File(f) => Some(f),
//             _ => None,
//         }
//     }
// }
//
// pub trait DirectoryGetKind: private::Sealed {}
//
// pub trait NodeKind: private::Sealed {
//     fn from_any<Fs>(node: NodeRef<Fs, Any>) -> Option<NodeRef<Fs, Self>>
//     where
//         Fs: FileSystem;
// }
//
// pub enum SpecificNodeKind<Fs> {
//     File(NodeRef<Fs, File>),
//     Directory(NodeRef<Fs, Directory>),
// }
//
// pub trait FileSystem {}
//
// mod private {
//     pub trait Sealed {}
// }
