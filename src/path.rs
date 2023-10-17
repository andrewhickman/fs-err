#[allow(unused_imports)]
use crate::errors::{Error, ErrorKind};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Defines aliases on [`Path`](https://doc.rust-lang.org/std/path/struct.Path.html) for `fs_err` functions.
///
/// This trait is sealed and can not be implemented by other crates.
//
// Because no one else can implement it, we can add methods backwards-compatibly.
pub trait PathExt: crate::Sealed {
    /// Wrapper for [`Path::try_exists`](https://doc.rust-lang.org/std/path/struct.Path.html#method.try_exists).
    #[cfg(feature = "path_try_exists")]
    fn fs_err_try_exists(&self) -> io::Result<bool>;
    /// Wrapper for [`crate::metadata`].
    fn fs_err_metadata(&self) -> io::Result<fs::Metadata>;
    /// Wrapper for [`crate::symlink_metadata`].
    fn fs_err_symlink_metadata(&self) -> io::Result<fs::Metadata>;
    /// Wrapper for [`crate::canonicalize`].
    fn fs_err_canonicalize(&self) -> io::Result<PathBuf>;
    /// Wrapper for [`crate::read_link`].
    fn fs_err_read_link(&self) -> io::Result<PathBuf>;
    /// Wrapper for [`crate::read_dir`].
    fn fs_err_read_dir(&self) -> io::Result<crate::ReadDir>;
}

impl PathExt for Path {
    #[cfg(feature = "path_try_exists")]
    fn fs_err_try_exists(&self) -> io::Result<bool> {
        self.try_exists()
            .map_err(|source| Error::build(source, ErrorKind::FileExists, self))
    }

    fn fs_err_metadata(&self) -> io::Result<fs::Metadata> {
        crate::metadata(self)
    }

    fn fs_err_symlink_metadata(&self) -> io::Result<fs::Metadata> {
        crate::symlink_metadata(self)
    }

    fn fs_err_canonicalize(&self) -> io::Result<PathBuf> {
        crate::canonicalize(self)
    }

    fn fs_err_read_link(&self) -> io::Result<PathBuf> {
        crate::read_link(self)
    }

    fn fs_err_read_dir(&self) -> io::Result<crate::ReadDir> {
        crate::read_dir(self)
    }
}
