//! This crate provides the [`File`](struct.File.html) type, a wrapper
//! around a file handle and its path which wraps all operations with
//! more helpful error messages.

#![doc(html_root_url = "https://docs.rs/fs-err/1.0.1")]
#![deny(missing_debug_implementations, missing_docs)]

use std::fmt;
use std::io::{Read, Seek, Write};
use std::path::{self, Path, PathBuf};

/// A wrapper around a file handle and its path which wraps all
/// operations with more helpful error messages.
#[derive(Debug)]
pub struct File {
    file: std::fs::File,
    path: PathBuf,
}

/// A wrapper around a `std::io::Error` from a filesystem operation,
/// with context including the path of the associated file.
#[derive(Debug)]
pub struct Error {
    source: std::io::Error,
    message: String,
}

impl File {
    /// Wrapper for [`File::open`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.open).
    pub fn open<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path> + Into<PathBuf>,
    {
        match std::fs::File::open(path.as_ref()) {
            Ok(file) => Ok(File::from_parts(file, path.into())),
            Err(error) => Err(Error::new(
                error,
                format!("failed to open file `{}`", path.as_ref().display()),
            )),
        }
    }

    /// Wrapper for [`File::create`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.create).
    pub fn create<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path> + Into<PathBuf>,
    {
        match std::fs::File::open(path.as_ref()) {
            Ok(file) => Ok(File::from_parts(file, path.into())),
            Err(error) => Err(Error::new(
                error,
                format!("failed to create file `{}`", path.as_ref().display()),
            )),
        }
    }

    /// Wrapper for [`OpenOptions::open`](https://doc.rust-lang.org/stable/std/fs/struct.OpenOptions.html#method.open).
    pub fn from_options<P>(path: P, options: &std::fs::OpenOptions) -> Result<Self, Error>
    where
        P: AsRef<Path> + Into<PathBuf>,
    {
        match options.open(path.as_ref()) {
            Ok(file) => Ok(File::from_parts(file, path.into())),
            Err(error) => Err(Error::new(
                error,
                format!("failed to open file `{}`", path.as_ref().display()),
            )),
        }
    }

    /// Wrapper for [`File::sync_all`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.sync_all).
    pub fn sync_all(&self) -> Result<(), Error> {
        self.try_exec(
            |file| file.sync_all(),
            |path| format!("failed to synchronize file `{}`", path),
        )
    }

    /// Wrapper for [`File::sync_data`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.sync_data).
    pub fn sync_data(&self) -> Result<(), Error> {
        self.try_exec(
            |file| file.sync_data(),
            |path| format!("failed to synchronize file `{}`", path),
        )
    }

    /// Wrapper for [`File::set_len`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.set_len).
    pub fn set_len(&self, size: u64) -> Result<(), Error> {
        self.try_exec(
            |file| file.set_len(size),
            |path| format!("failed to set length for file `{}`", path),
        )
    }

    /// Wrapper for [`File::metadata`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.metadata).
    pub fn metadata(&self) -> Result<std::fs::Metadata, Error> {
        self.try_exec(
            |file| file.metadata(),
            |path| format!("failed to query metadata for file `{}`", path),
        )
    }

    /// Wrapper for [`File::try_clone`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.try_clone).
    pub fn try_clone(&self) -> Result<File, Error> {
        let file = self.try_exec(
            |file| file.try_clone(),
            |path| format!("failed to clone handle for file `{}`", path),
        )?;
        Ok(File {
            file,
            path: self.path.clone(),
        })
    }

    /// Wrapper for [`File::set_permissions`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.set_permissions).
    pub fn set_permissions(&self, perm: std::fs::Permissions) -> Result<(), Error> {
        self.try_exec(
            |file| file.set_permissions(perm),
            |path| format!("failed to set permissions for file `{}`", path),
        )
    }

    /// Creates a [`File`](struct.File.html) from a raw file and its
    /// path.
    pub fn from_parts<P>(file: std::fs::File, path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        File {
            file,
            path: path.into(),
        }
    }

    /// Gets the wrapped file.
    pub fn file(&self) -> &std::fs::File {
        &self.file
    }

    /// Gets the path of the wrapped file.
    pub fn path(&self) -> &Path {
        &self.path
    }

    fn try_exec<R>(
        &self,
        op: impl FnOnce(&std::fs::File) -> std::io::Result<R>,
        context: impl FnOnce(&path::Display) -> String,
    ) -> Result<R, Error> {
        match op(&self.file) {
            Ok(result) => Ok(result),
            Err(error) => Err(Error::new(error, context(&self.path.display()))),
        }
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        (&mut &*self).read(buf)
    }

    fn read_vectored(&mut self, bufs: &mut [std::io::IoSliceMut<'_>]) -> std::io::Result<usize> {
        (&mut &*self).read_vectored(bufs)
    }
}

impl<'a> Read for &'a File {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.try_exec(
            |mut file| file.read(buf),
            |path| format!("failed to read from file `{}`", path),
        )
        .map_err(Error::into_io_error)
    }

    fn read_vectored(&mut self, bufs: &mut [std::io::IoSliceMut<'_>]) -> std::io::Result<usize> {
        self.try_exec(
            |mut file| file.read_vectored(bufs),
            |path| format!("failed to read from file `{}`", path),
        )
        .map_err(Error::into_io_error)
    }
}

impl Seek for File {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        (&mut &*self).seek(pos)
    }
}

impl<'a> Seek for &'a File {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.try_exec(
            |mut file| file.seek(pos),
            |path| format!("failed to seek in file `{}`", path),
        )
        .map_err(Error::into_io_error)
    }
}

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        (&mut &*self).write(buf)
    }

    fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize> {
        (&mut &*self).write_vectored(bufs)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        (&mut &*self).flush()
    }
}

impl<'a> Write for &'a File {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.try_exec(
            |mut file| file.write(buf),
            |path| format!("failed to write to file `{}`", path),
        )
        .map_err(Error::into_io_error)
    }

    fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize> {
        self.try_exec(
            |mut file| file.write_vectored(bufs),
            |path| format!("failed to write to file `{}`", path),
        )
        .map_err(Error::into_io_error)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.try_exec(
            |mut file| file.flush(),
            |path| format!("failed to flush file `{}`", path),
        )
        .map_err(Error::into_io_error)
    }
}

impl Error {
    /// Constructs an [`Error`](struct.Error.html).
    pub fn new(source: std::io::Error, message: String) -> Self {
        Error { source, message }
    }

    /// Gets a reference to the raw error.
    pub fn source(&self) -> &std::io::Error {
        &self.source
    }

    /// Convert into an `io::Error`.
    pub fn into_io_error(self) -> std::io::Error {
        std::io::Error::new(self.source.kind(), self)
    }
}

impl From<Error> for std::io::Error {
    fn from(err: Error) -> std::io::Error {
        err.into_io_error()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.message.fmt(f)
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source())
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source())
    }
}

fn initial_buffer_size(file: &File) -> usize {
    file.file()
        .metadata()
        .map(|m| m.len() as usize + 1)
        .unwrap_or(0)
}

/// Wrapper for [`fs::read`](https://doc.rust-lang.org/stable/std/fs/fn.read.html).
pub fn read<P: AsRef<Path> + Into<PathBuf>>(path: P) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::with_capacity(initial_buffer_size(&file));
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
}

/// Wrapper for [`fs::read_to_string`](https://doc.rust-lang.org/stable/std/fs/fn.read_to_string.html).
pub fn read_to_string<P: AsRef<Path> + Into<PathBuf>>(path: P) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut string = String::with_capacity(initial_buffer_size(&file));
    file.read_to_string(&mut string)?;
    Ok(string)
}

/// Wrapper for [`fs::write`](https://doc.rust-lang.org/stable/std/fs/fn.write.html).
pub fn write<P: AsRef<Path> + Into<PathBuf>, C: AsRef<[u8]>>(
    path: P,
    contents: C,
) -> std::io::Result<()> {
    File::create(path)?.write_all(contents.as_ref())
}

#[test]
fn open() {
    let err = File::open("doesn't exist").unwrap_err();
    assert_eq!(format!("{}", err), "failed to open file `doesn't exist`");
}
