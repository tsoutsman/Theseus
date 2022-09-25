use alloc::string::String;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NamespaceNotFound,
    AppNotFound(String),
    MultipleAppsFound,
    SpawnFailed(&'static str),
}
