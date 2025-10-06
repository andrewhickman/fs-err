//! Unix-specific extensions.

use crate::errors::{SourceDestError, SourceDestErrorKind};
use crate::private::Sealed;
use std::io;
#[doc(no_inline)]
pub use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};
use std::path::Path;

/// Creates a new symbolic link on the filesystem.
///
/// Wrapper for [`async_fs::unix::symlink`].
pub async fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    let (src, dst) = (src.as_ref(), dst.as_ref());
    async_fs::unix::symlink(src, dst)
        .await
        .map_err(|err| SourceDestError::build(err, SourceDestErrorKind::Symlink, src, dst))
}

/// Unix-specific extensions to [`crate::async_fs::DirBuilder`].
pub trait DirBuilderExt: Sealed {
    /// Sets the mode to create new directories with.
    ///
    /// Wrapper for [`async_fs::unix::DirBuilderExt::mode`]
    fn mode(&mut self, mode: u32) -> &mut Self;
}
