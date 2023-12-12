use alloc::boxed::Box;

pub trait FileSystem {
    type File: File;
    type Directory: Directory<File = Self::File>;

    fn root(&self) -> Self::Directory;

    // fn to_dyn(self) -> Box<dyn FileSystem<File = dyn File<File = dyn File, Directory = dyn Directory>, Directory = dyn Directory>>;
}

static X: Box<dyn FileSystem<File = dyn File, Directory = dyn Directory<File = dyn File>>> =
    todo!();

pub trait Node {
    // type Directory: Directory;
    fn parent(&self);
    fn name(&self) -> &str;
    // fn as_type(&self) -> NodeType<Self::File, Self::Directory>;
}

pub enum NodeType<F, D>
where
    Self: ?Sized,
{
    File(F),
    Directory(D),
}

pub trait Directory: Node {
    type File: File;

    fn create_dir(&self, name: &str);

    fn create_file(&self, name: &str);

    fn mount<T>(&self, file_system: T)
    where
        T: FileSystem;

    fn get(&self, name: &str) -> NodeType<Self::File, Self>;
}

pub trait File: Node {
    // TODO
    fn read(&self);
}
