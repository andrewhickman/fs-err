//! Windows-specific extensions.

use crate::errors::{SourceDestError, SourceDestErrorKind};
use std::io;
use std::path::Path;

/// Creates a new file symbolic link on the filesystem.
///
/// Wrapper for [`async_fs::windows::symlink_dir`].
pub async fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    let (src, dst) = (src.as_ref(), dst.as_ref());
    async_fs::windows::symlink_dir(src, dst)
        .await
        .map_err(|err| SourceDestError::build(err, SourceDestErrorKind::SymlinkDir, src, dst))
}

/// Creates a new file symbolic link on the filesystem.
///
/// Wrapper for [`async_fs::windows::symlink_file`].
pub async fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    let (src, dst) = (src.as_ref(), dst.as_ref());
    async_fs::windows::symlink_file(src, dst)
        .await
        .map_err(|err| SourceDestError::build(err, SourceDestErrorKind::SymlinkFile, src, dst))
}
