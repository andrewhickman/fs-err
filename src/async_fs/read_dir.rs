use crate::errors::{Error, ErrorKind};
use crate::private::Sealed;
use futures_lite::StreamExt;

use futures_lite::Stream;
use std::ffi::OsString;
use std::fs::{FileType, Metadata};
use std::io;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::task::{ready, Context, Poll};

#[cfg(unix)]
use async_fs::unix::DirEntryExt as _;

/// Returns a stream of entries in a directory.
///
/// Wrapper for [`async_fs::read_dir`].
pub async fn read_dir<P: AsRef<Path>>(path: P) -> io::Result<ReadDir> {
    let path = path.as_ref();
    let async_fs = async_fs::read_dir(path)
        .await
        .map_err(|err| Error::build(err, ErrorKind::ReadDir, path))?;
    Ok(ReadDir {
        async_fs,
        path: path.to_owned(),
    })
}

/// Reads the entries in a directory.
///
/// This is a wrapper around [`async_fs::ReadDir`].
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
#[cfg_attr(docsrs, doc(cfg(feature = "async_fs")))]
pub struct ReadDir {
    async_fs: async_fs::ReadDir,
    path: PathBuf,
}

impl Stream for ReadDir {
    type Item = io::Result<DirEntry>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(match ready!(self.async_fs.poll_next(cx)) {
            Some(Ok(entry)) => Some(Ok(DirEntry(entry))),
            Some(Err(err)) => Some(Err(Error::build(err, ErrorKind::ReadDir, &self.path))),
            None => None,
        })
    }
}

/// An entry in a directory.
///
/// This is a wrapper around [`async_fs::DirEntry`].
#[derive(Debug, Clone)]
pub struct DirEntry(async_fs::DirEntry);

impl DirEntry {
    /// Returns the full path to this entry.
    ///
    /// This is a wrapper around [`async_fs::DirEntry::path`].
    pub fn path(&self) -> PathBuf {
        self.0.path()
    }

    /// Reads the metadata for this entry.
    ///
    /// This is a wrapper around [`async_fs::DirEntry::metadata`].
    pub async fn metadata(&self) -> io::Result<Metadata> {
        self.0
            .metadata()
            .await
            .map_err(|err| Error::build(err, ErrorKind::Metadata, self.path()))
    }

    /// Reads the file type for this entry.
    ///
    /// This is a wrapper around [`async_fs::DirEntry::file_type`].
    pub async fn file_type(&self) -> io::Result<FileType> {
        self.0
            .file_type()
            .await
            .map_err(|err| Error::build(err, ErrorKind::Metadata, self.path()))
    }

    /// Returns the bare name of this entry without the leading path.
    ///
    /// This is a wrapper around [`async_fs::DirEntry::file_name`].
    pub fn file_name(&self) -> OsString {
        self.0.file_name()
    }
}

impl Sealed for DirEntry {}

#[cfg(unix)]
impl crate::os::unix::fs::DirEntryExt for DirEntry {
    /// Returns the underlying `d_ino` field in the contained `dirent` structure.
    ///
    /// This is a wrapper around [`async_fs::unix::DirEntryExt::ino`]
    fn ino(&self) -> u64 {
        self.0.ino()
    }
}
