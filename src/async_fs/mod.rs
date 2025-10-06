//! async-fs-specific wrappers that use `fs_err` error messages.

mod dir_builder;
mod file;
mod open_options;
mod read_dir;
#[cfg(unix)]
pub mod unix;
#[cfg(windows)]
pub mod windows;

use crate::errors::{Error, ErrorKind, SourceDestError, SourceDestErrorKind};
#[doc(no_inline)]
pub use std::fs::{FileType, Metadata, Permissions};
use std::io;
use std::path::{Path, PathBuf};

pub use self::open_options::OpenOptions;
pub use self::read_dir::{read_dir, DirEntry, ReadDir};
pub use dir_builder::DirBuilder;
pub use file::File;

/// Returns the canonical form of a path.
///
/// Wrapper for [`async_fs::canonicalize`].
pub async fn canonicalize<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    let path = path.as_ref();
    async_fs::canonicalize(path)
        .await
        .map_err(|err| Error::build(err, ErrorKind::Canonicalize, path))
}

/// Copies a file to a new location.
///
/// Wrapper for [`async_fs::copy`].
pub async fn copy<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<u64> {
    let (src, dst) = (src.as_ref(), dst.as_ref());
    async_fs::copy(src, dst)
        .await
        .map_err(|err| SourceDestError::build(err, SourceDestErrorKind::Copy, src, dst))
}

/// Creates a new, empty directory at the provided path
///
/// Wrapper for [`async_fs::create_dir`].
pub async fn create_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();
    async_fs::create_dir(path)
        .await
        .map_err(|err| Error::build(err, ErrorKind::CreateDir, path))
}

/// Recursively create a directory and all of its parent components if they
/// are missing.
///
/// Wrapper for [`async_fs::create_dir_all`].
pub async fn create_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();
    async_fs::create_dir_all(path)
        .await
        .map_err(|err| Error::build(err, ErrorKind::CreateDir, path))
}

/// Creates a hard link on the filesystem.
///
/// Wrapper for [`async_fs::hard_link`].
pub async fn hard_link<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    let (src, dst) = (src.as_ref(), dst.as_ref());
    async_fs::hard_link(src, dst)
        .await
        .map_err(|err| SourceDestError::build(err, SourceDestErrorKind::HardLink, src, dst))
}

/// Reads metadata for a path.
///
/// Wrapper for [`async_fs::metadata`].
pub async fn metadata<P: AsRef<Path>>(path: P) -> io::Result<Metadata> {
    let path = path.as_ref();
    async_fs::metadata(path)
        .await
        .map_err(|err| Error::build(err, ErrorKind::Metadata, path))
}

/// Reads the entire contents of a file as raw bytes.
///
/// Wrapper for [`async_fs::read`].
pub async fn read<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let path = path.as_ref();
    async_fs::read(path)
        .await
        .map_err(|err| Error::build(err, ErrorKind::Read, path))
}

/// Reads a symbolic link and returns the path it points to.
///
/// Wrapper for [`async_fs::read_link`].
pub async fn read_link<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    let path = path.as_ref();
    async_fs::read_link(path)
        .await
        .map_err(|err| Error::build(err, ErrorKind::ReadLink, path))
}

/// Reads the entire contents of a file as a string.
///
/// Wrapper for [`async_fs::read_to_string`].
pub async fn read_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let path = path.as_ref();
    async_fs::read_to_string(path)
        .await
        .map_err(|err| Error::build(err, ErrorKind::Read, path))
}

/// Removes an empty directory.
///
/// Wrapper for [`async_fs::remove_dir`].
pub async fn remove_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();
    async_fs::remove_dir(path)
        .await
        .map_err(|err| Error::build(err, ErrorKind::RemoveDir, path))
}

/// Removes a directory and all of its contents.
///
/// Wrapper for [`async_fs::remove_dir_all`].
pub async fn remove_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();
    async_fs::remove_dir_all(path)
        .await
        .map_err(|err| Error::build(err, ErrorKind::RemoveDir, path))
}

/// Removes a file.
///
/// Wrapper for [`async_fs::remove_file`].
pub async fn remove_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();
    async_fs::remove_file(path)
        .await
        .map_err(|err| Error::build(err, ErrorKind::RemoveFile, path))
}

/// Renames a file or directory to a new location.
///
/// Wrapper for [`async_fs::rename`].
pub async fn rename<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    let (src, dst) = (src.as_ref(), dst.as_ref());
    async_fs::rename(src, dst)
        .await
        .map_err(|err| SourceDestError::build(err, SourceDestErrorKind::Rename, src, dst))
}

/// Changes the permissions of a file or directory.
///
/// Wrapper for [`async_fs::set_permissions`].
pub async fn set_permissions<P: AsRef<Path>>(path: P, perm: Permissions) -> io::Result<()> {
    let path = path.as_ref();
    async_fs::set_permissions(path, perm)
        .await
        .map_err(|err| Error::build(err, ErrorKind::SetPermissions, path))
}

/// Reads metadata for a path without following symbolic links.
///
/// Wrapper for [`async_fs::symlink_metadata`].
pub async fn symlink_metadata<P: AsRef<Path>>(path: P) -> io::Result<Metadata> {
    let path = path.as_ref();
    async_fs::symlink_metadata(path)
        .await
        .map_err(|err| Error::build(err, ErrorKind::SymlinkMetadata, path))
}

/// Writes a slice of bytes as the new contents of a file.
///
/// Wrapper for [`async_fs::write`].
pub async fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> io::Result<()> {
    let (path, contents) = (path.as_ref(), contents.as_ref());
    async_fs::write(path, contents)
        .await
        .map_err(|err| Error::build(err, ErrorKind::Write, path))
}
