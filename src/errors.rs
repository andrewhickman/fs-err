use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub(crate) enum ErrorKind {
    OpenFile,
    CreateFile,
    CreateDir,
    SyncFile,
    SetLen,
    Metadata,
    Clone,
    SetPermissions,
    Read,
    Seek,
    Write,
    Flush,
    ReadDir,
    RemoveFile,
    RemoveDir,
    Canonicalize,
    ReadLink,
    SymlinkMetadata,
    #[allow(dead_code)]
    FileExists,

    #[cfg(windows)]
    SeekRead,
    #[cfg(windows)]
    SeekWrite,

    #[cfg(unix)]
    ReadAt,
    #[cfg(unix)]
    WriteAt,
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
    pub fn build(source: io::Error, kind: ErrorKind, path: impl Into<PathBuf>) -> io::Error {
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
        use ErrorKind as E;

        let path = self.path.display();

        match self.kind {
            E::OpenFile => write!(formatter, "failed to open file `{}`", path),
            E::CreateFile => write!(formatter, "failed to create file `{}`", path),
            E::CreateDir => write!(formatter, "failed to create directory `{}`", path),
            E::SyncFile => write!(formatter, "failed to sync file `{}`", path),
            E::SetLen => write!(formatter, "failed to set length of file `{}`", path),
            E::Metadata => write!(formatter, "failed to query metadata of file `{}`", path),
            E::Clone => write!(formatter, "failed to clone handle for file `{}`", path),
            E::SetPermissions => write!(formatter, "failed to set permissions for file `{}`", path),
            E::Read => write!(formatter, "failed to read from file `{}`", path),
            E::Seek => write!(formatter, "failed to seek in file `{}`", path),
            E::Write => write!(formatter, "failed to write to file `{}`", path),
            E::Flush => write!(formatter, "failed to flush file `{}`", path),
            E::ReadDir => write!(formatter, "failed to read directory `{}`", path),
            E::RemoveFile => write!(formatter, "failed to remove file `{}`", path),
            E::RemoveDir => write!(formatter, "failed to remove directory `{}`", path),
            E::Canonicalize => write!(formatter, "failed to canonicalize path `{}`", path),
            E::ReadLink => write!(formatter, "failed to read symbolic link `{}`", path),
            E::SymlinkMetadata => {
                write!(formatter, "failed to query metadata of symlink `{}`", path)
            }
            E::FileExists => write!(formatter, "failed to check file existance `{}`", path),

            #[cfg(windows)]
            E::SeekRead => write!(formatter, "failed to seek and read from `{}`", path),
            #[cfg(windows)]
            E::SeekWrite => write!(formatter, "failed to seek and write to `{}`", path),

            #[cfg(unix)]
            E::ReadAt => write!(formatter, "failed to read with offset from `{}`", path),
            #[cfg(unix)]
            E::WriteAt => write!(formatter, "failed to write with offset to `{}`", path),
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

#[derive(Debug, Clone, Copy)]
pub(crate) enum SourceDestErrorKind {
    Copy,
    HardLink,
    Rename,
    SoftLink,

    #[cfg(unix)]
    Symlink,

    #[cfg(windows)]
    SymlinkDir,
    #[cfg(windows)]
    SymlinkFile,
}

/// Error type used by functions like `fs::copy` that holds two paths.
#[derive(Debug)]
pub(crate) struct SourceDestError {
    kind: SourceDestErrorKind,
    source: io::Error,
    from_path: PathBuf,
    to_path: PathBuf,
}

impl SourceDestError {
    pub fn build(
        source: io::Error,
        kind: SourceDestErrorKind,
        from_path: impl Into<PathBuf>,
        to_path: impl Into<PathBuf>,
    ) -> io::Error {
        io::Error::new(
            source.kind(),
            Self {
                kind,
                source,
                from_path: from_path.into(),
                to_path: to_path.into(),
            },
        )
    }
}

impl fmt::Display for SourceDestError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let from = self.from_path.display();
        let to = self.to_path.display();
        match self.kind {
            SourceDestErrorKind::Copy => {
                write!(formatter, "failed to copy file from {} to {}", from, to)
            }
            SourceDestErrorKind::HardLink => {
                write!(formatter, "failed to hardlink file from {} to {}", from, to)
            }
            SourceDestErrorKind::Rename => {
                write!(formatter, "failed to rename file from {} to {}", from, to)
            }
            SourceDestErrorKind::SoftLink => {
                write!(formatter, "failed to softlink file from {} to {}", from, to)
            }

            #[cfg(unix)]
            SourceDestErrorKind::Symlink => {
                write!(formatter, "failed to symlink file from {} to {}", from, to)
            }

            #[cfg(windows)]
            SourceDestErrorKind::SymlinkFile => {
                write!(formatter, "failed to symlink file from {} to {}", from, to)
            }
            #[cfg(windows)]
            SourceDestErrorKind::SymlinkDir => {
                write!(formatter, "failed to symlink dir from {} to {}", from, to)
            }
        }
    }
}

impl StdError for SourceDestError {
    fn cause(&self) -> Option<&dyn StdError> {
        self.source()
    }

    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.source)
    }
}
