use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::path::PathBuf;

/// Contains an IO error that has a file path attached.
///
/// This type is never returned directly, but is instead wrapped inside yet
/// another IO error.
#[derive(Debug)]
pub(crate) struct Error {
    source: io::Error,
    path: PathBuf,
}

impl Error {
    pub fn new<P: Into<PathBuf>>(source: io::Error, path: P) -> io::Error {
        io::Error::new(
            source.kind(),
            Self {
                source,
                path: path.into(),
            },
        )
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{} in path {}", self.source, self.path.display())
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.source)
    }
}

/// Contains an IO error from a copy operation, containing both paths.
#[derive(Debug)]
pub(crate) struct CopyError {
    source: io::Error,
    source_path: PathBuf,
    dest_path: PathBuf,
}

impl CopyError {
    pub fn new<P: Into<PathBuf>, Q: Into<PathBuf>>(
        source: io::Error,
        source_path: P,
        dest_path: Q,
    ) -> io::Error {
        io::Error::new(
            source.kind(),
            Self {
                source,
                source_path: source_path.into(),
                dest_path: dest_path.into(),
            },
        )
    }
}

impl fmt::Display for CopyError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{} copying path {} to {}",
            self.source,
            self.source_path.display(),
            self.dest_path.display()
        )
    }
}

impl StdError for CopyError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.source)
    }
}
