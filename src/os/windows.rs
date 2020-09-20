/// Windows-specific extensions to wrappers in `fs_err` for `std::fs` types.
pub mod fs {
    use crate::{SourceDestError, SourceDestErrorKind};
    use std::io;
    use std::path::{Path, PathBuf};
    /// Wrapper for [std::os::windows::fs::symlink_dir](https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_dir.html)
    pub fn symlink_dir<P: AsRef<Path> + Into<PathBuf>, Q: AsRef<Path> + Into<PathBuf>>(
        src: P,
        dst: Q,
    ) -> io::Result<()> {
        std::os::windows::fs::symlink_dir(src.as_ref(), dst.as_ref())
            .map_err(|err| SourceDestError::new(err, SourceDestErrorKind::SymlinkDir, src, dst))
    }

    /// Wrapper for [std::os::windows::fs::symlink_file](https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_file.html)
    pub fn symlink_file<P: AsRef<Path> + Into<PathBuf>, Q: AsRef<Path> + Into<PathBuf>>(
        src: P,
        dst: Q,
    ) -> io::Result<()> {
        std::os::windows::fs::symlink_file(src.as_ref(), dst.as_ref())
            .map_err(|err| SourceDestError::new(err, SourceDestErrorKind::SymlinkFile, src, dst))
    }
}
