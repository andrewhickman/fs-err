#[cfg(unix)]
use crate::async_fs::unix;
use crate::errors::{Error, ErrorKind};
use crate::private::Sealed;
#[cfg(unix)]
use async_fs::unix::DirBuilderExt;
use std::io;
use std::path::Path;

/// A builder for creating directories with configurable options.
///
/// This is a wrapper around [`async_fs::DirBuilder`].
#[derive(Debug, Default)]
#[cfg_attr(docsrs, doc(cfg(feature = "async-fs")))]
pub struct DirBuilder(async_fs::DirBuilder);

impl DirBuilder {
    /// Creates a blank set of options.
    ///
    /// This is a wrapper around [`async_fs::DirBuilder::new`].
    pub fn new() -> Self {
        Self(async_fs::DirBuilder::new())
    }

    /// Sets the option for recursive mode.
    ///
    /// This is a wrapper around [`async_fs::DirBuilder::recursive`].
    pub fn recursive(&mut self, recursive: bool) -> &mut Self {
        self.0.recursive(recursive);
        self
    }

    /// Creates a directory with the configured options.
    ///
    /// This is a wrapper around [`async_fs::DirBuilder::create`].
    pub async fn create<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        self.0
            .create(path)
            .await
            .map_err(|err| Error::build(err, ErrorKind::CreateDir, path))
    }
}

impl Sealed for DirBuilder {}

#[cfg(unix)]
impl unix::DirBuilderExt for DirBuilder {
    fn mode(&mut self, mode: u32) -> &mut Self {
        self.0.mode(mode);
        self
    }
}
