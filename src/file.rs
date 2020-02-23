use std::fs;
use std::io::{self, Read, Seek, Write};
use std::path::{Path, PathBuf};

use crate::errors::{Error, ErrorKind};

/// Wrapper around [`std::fs::File`][std::fs::File] which adds more helpful
/// information to all errors.
///
/// [std::fs::File]: https://doc.rust-lang.org/stable/std/fs/struct.File.html
#[derive(Debug)]
pub struct File {
    file: fs::File,
    path: PathBuf,
}

/// Wrappers for methods from [`std::fs::File`][std::fs::File].
///
/// [std::fs::File]: https://doc.rust-lang.org/stable/std/fs/struct.File.html
impl File {
    /// Wrapper for [`File::open`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.open).
    pub fn open<P>(path: P) -> Result<Self, io::Error>
    where
        P: AsRef<Path> + Into<PathBuf>,
    {
        match fs::File::open(path.as_ref()) {
            Ok(file) => Ok(File::from_parts(file, path.into())),
            Err(source) => Err(Error::new(source, ErrorKind::OpenFile, path)),
        }
    }

    /// Wrapper for [`File::create`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.create).
    pub fn create<P>(path: P) -> Result<Self, io::Error>
    where
        P: AsRef<Path> + Into<PathBuf>,
    {
        match fs::File::create(path.as_ref()) {
            Ok(file) => Ok(File::from_parts(file, path.into())),
            Err(source) => Err(Error::new(source, ErrorKind::CreateFile, path)),
        }
    }

    /// Wrapper for [`OpenOptions::open`](https://doc.rust-lang.org/stable/std/fs/struct.OpenOptions.html#method.open).
    pub fn from_options<P>(path: P, options: &fs::OpenOptions) -> Result<Self, io::Error>
    where
        P: AsRef<Path> + Into<PathBuf>,
    {
        match options.open(path.as_ref()) {
            Ok(file) => Ok(File::from_parts(file, path.into())),
            Err(source) => Err(Error::new(source, ErrorKind::OpenFile, path)),
        }
    }

    /// Wrapper for [`File::sync_all`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.sync_all).
    pub fn sync_all(&self) -> Result<(), io::Error> {
        self.file
            .sync_all()
            .map_err(|source| self.error(source, ErrorKind::SyncFile))
    }

    /// Wrapper for [`File::sync_data`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.sync_data).
    pub fn sync_data(&self) -> Result<(), io::Error> {
        self.file
            .sync_data()
            .map_err(|source| self.error(source, ErrorKind::SyncFile))
    }

    /// Wrapper for [`File::set_len`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.set_len).
    pub fn set_len(&self, size: u64) -> Result<(), io::Error> {
        self.file
            .set_len(size)
            .map_err(|source| self.error(source, ErrorKind::SetLen))
    }

    /// Wrapper for [`File::metadata`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.metadata).
    pub fn metadata(&self) -> Result<fs::Metadata, io::Error> {
        self.file
            .metadata()
            .map_err(|source| self.error(source, ErrorKind::Metadata))
    }

    /// Wrapper for [`File::try_clone`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.try_clone).
    pub fn try_clone(&self) -> Result<Self, io::Error> {
        self.file
            .try_clone()
            .map(|file| File {
                file,
                path: self.path.clone(),
            })
            .map_err(|source| self.error(source, ErrorKind::Clone))
    }

    /// Wrapper for [`File::set_permissions`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.set_permissions).
    pub fn set_permissions(&self, perm: fs::Permissions) -> Result<(), io::Error> {
        self.file
            .set_permissions(perm)
            .map_err(|source| self.error(source, ErrorKind::SetPermissions))
    }

    /// Creates a [`File`](struct.File.html) from a raw file and its path.
    pub fn from_parts<P>(file: fs::File, path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        File {
            file,
            path: path.into(),
        }
    }
}

/// Methods added by fs-err that are not available on
/// [`std::fs::File`][std::fs::File].
///
/// [std::fs::File]: https://doc.rust-lang.org/stable/std/fs/struct.File.html
impl File {
    /// Returns a reference to the underlying [`std::fs::File`][std::fs::File].
    ///
    /// [std::fs::File]: https://doc.rust-lang.org/stable/std/fs/struct.File.html
    pub fn file(&self) -> &fs::File {
        &self.file
    }

    /// Returns a reference to the path that this file was created with.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Wrap the error in information specific to this `File` object.
    fn error(&self, source: io::Error, kind: ErrorKind) -> io::Error {
        Error::new(source, kind, &self.path)
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.file
            .read(buf)
            .map_err(|source| self.error(source, ErrorKind::Read))
    }

    fn read_vectored(&mut self, bufs: &mut [std::io::IoSliceMut<'_>]) -> std::io::Result<usize> {
        self.file
            .read_vectored(bufs)
            .map_err(|source| self.error(source, ErrorKind::Read))
    }
}

impl<'a> Read for &'a File {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        (&(**self).file)
            .read(buf)
            .map_err(|source| self.error(source, ErrorKind::Read))
    }

    fn read_vectored(&mut self, bufs: &mut [std::io::IoSliceMut<'_>]) -> std::io::Result<usize> {
        (&(**self).file)
            .read_vectored(bufs)
            .map_err(|source| self.error(source, ErrorKind::Read))
    }
}

impl Seek for File {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.file
            .seek(pos)
            .map_err(|source| self.error(source, ErrorKind::Seek))
    }
}

impl<'a> Seek for &'a File {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        (&(**self).file)
            .seek(pos)
            .map_err(|source| self.error(source, ErrorKind::Seek))
    }
}

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.file
            .write(buf)
            .map_err(|source| self.error(source, ErrorKind::Write))
    }

    fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize> {
        self.file
            .write_vectored(bufs)
            .map_err(|source| self.error(source, ErrorKind::Write))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.file
            .flush()
            .map_err(|source| self.error(source, ErrorKind::Flush))
    }
}

impl<'a> Write for &'a File {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        (&(**self).file)
            .write(buf)
            .map_err(|source| self.error(source, ErrorKind::Write))
    }

    fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize> {
        (&(**self).file)
            .write_vectored(bufs)
            .map_err(|source| self.error(source, ErrorKind::Write))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        (&(**self).file)
            .flush()
            .map_err(|source| self.error(source, ErrorKind::Flush))
    }
}
