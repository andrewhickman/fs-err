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
    fn cause(&self) -> Option<&dyn StdError> {
        self.source()
    }

    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.source)
    }
}
