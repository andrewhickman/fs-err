/// Unix-specific extensions to wrappers in `fs_err` for `std::fs` types.
pub mod fs {
    use std::path::Path;
    use std::{io, path::PathBuf};

    use crate::SourceDestError;
    use crate::SourceDestErrorKind;

    /// Wrapper for [`std::os::unix::fs::symlink`](https://doc.rust-lang.org/std/os/unix/fs/fn.symlink.html)
    pub fn symlink<P: AsRef<Path> + Into<PathBuf>, Q: AsRef<Path> + Into<PathBuf>>(
        src: P,
        dst: Q,
    ) -> io::Result<()> {
        std::os::unix::fs::symlink(src.as_ref(), dst.as_ref())
            .map_err(|err| SourceDestError::new(err, SourceDestErrorKind::Symlink, src, dst))
    }
}
