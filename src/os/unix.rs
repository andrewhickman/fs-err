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

    /// Wrapper for [`std::os::unix::fs::FileExt`](https://doc.rust-lang.org/std/os/unix/fs/trait.FileExt.html).
    ///
    /// The std traits might be extended in the future (See issue [#49961](https://github.com/rust-lang/rust/issues/49961#issuecomment-382751777)).
    /// This trait is sealed and can not be implemented by other crates.
    pub trait FileExt: crate::Sealed {
        /// Wrapper for [`FileExt::read_at`](https://doc.rust-lang.org/std/os/unix/fs/trait.FileExt.html#tymethod.read_at)
        fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize>;
        /// Wrapper for [`FileExt::write_at`](https://doc.rust-lang.org/std/os/unix/fs/trait.FileExt.html#tymethod.write_at)
        fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize>;
    }
}
