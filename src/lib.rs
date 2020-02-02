//! This crate provides the [`File`](struct.File.html) type, a wrapper
//! around a file handle and its path which wraps all operations with
//! more helpful error messages.

#![doc(html_root_url = "https://docs.rs/fs-err/0.1.0")]
#![deny(missing_debug_implementations, missing_docs)]

use std::fmt;
use std::path::{Path, PathBuf};

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
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.message.fmt(f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source())
    }
}
