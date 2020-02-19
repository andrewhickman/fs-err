use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub(crate) enum ErrorKind {
    OpenFile,
    CreateFile,
    SyncFile,
    SetLen,
    Metadata,
    Clone,
    SetPermissions,
    Read,
    Seek,
    Write,
    Flush,
}

/// Contains an IO error that has a file path attached.
///
/// This type is never returned directly, but is instead wrapped inside yet
/// another IO error.
#[derive(Debug)]
pub(crate) struct Error {
    kind: ErrorKind,
    source: io::Error,
    path: PathBuf,
}

impl Error {
    pub fn new<P: Into<PathBuf>>(source: io::Error, kind: ErrorKind, path: P) -> io::Error {
        io::Error::new(
            source.kind(),
            Self {
                kind,
                source,
                path: path.into(),
            },
        )
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        use ErrorKind::*;

        let path = self.path.display();

        match self.kind {
            OpenFile => write!(formatter, "failed to open file `{}`", path),
            CreateFile => write!(formatter, "failed to create file `{}`", path),
            SyncFile => write!(formatter, "failed to sync file `{}`", path),
            SetLen => write!(formatter, "failed to set length of file `{}`", path),
            Metadata => write!(formatter, "failed to query metadata of file `{}`", path),
            Clone => write!(formatter, "failed to clone handle for file `{}`", path),
            SetPermissions => write!(formatter, "failed to set permissions for file `{}`", path),
            Read => write!(formatter, "failed to read from file `{}`", path),
            Seek => write!(formatter, "failed to seek in file `{}`", path),
            Write => write!(formatter, "failed to write to file `{}`", path),
            Flush => write!(formatter, "failed to flush file `{}`", path),
        }
    }
}

impl StdError for Error {
    fn cause(&self) -> Option<&dyn StdError> {
        self.source()
    }

    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.source)
    }
}
