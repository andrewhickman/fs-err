use crate::errors::{Error, ErrorKind};
use async_fs::File as AsyncFsFile;
use futures_lite::{AsyncRead, AsyncSeek, AsyncWrite};
use std::fs::{Metadata, Permissions};
use std::io;
use std::io::SeekFrom;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::task::{ready, Context, Poll};

/// Wrapper around [`async_fs::File`] which adds more helpful
/// information to all errors.
#[derive(Debug)]
#[cfg_attr(docsrs, doc(cfg(feature = "async-fs")))]
pub struct File {
    async_fs: async_fs::File,
    path: PathBuf,
}

impl File {
    /// Opens a file in read-only mode.
    ///
    /// This is a wrapper around [`async_fs::File::open`].
    pub async fn open<P: AsRef<Path>>(path: P) -> io::Result<File> {
        let path = path.as_ref();
        let f = AsyncFsFile::open(path)
            .await
            .map_err(|err| Error::build(err, ErrorKind::OpenFile, &path))?;
        Ok(File::from_parts(f, path))
    }

    /// Opens a file in write-only mode.
    ///
    /// This is a wrapper around [`async_fs::File::create`].
    pub async fn create<P: AsRef<Path>>(path: P) -> io::Result<File> {
        let path = path.as_ref();
        match AsyncFsFile::create(&path).await {
            Ok(f) => Ok(File::from_parts(f, path)),
            Err(err) => Err(Error::build(err, ErrorKind::CreateFile, &path)),
        }
    }

    /// Synchronizes OS-internal buffered contents and metadata to disk.
    ///
    /// This is a wrapper around [`async_fs::File::sync_all`].
    pub async fn sync_all(&self) -> io::Result<()> {
        self.async_fs
            .sync_all()
            .await
            .map_err(|err| Error::build(err, ErrorKind::SyncFile, &self.path))
    }

    /// Synchronizes OS-internal buffered contents to disk.
    ///
    /// This is a wrapper around [`async_fs::File::sync_data`].
    pub async fn sync_data(&self) -> io::Result<()> {
        self.async_fs
            .sync_data()
            .await
            .map_err(|err| Error::build(err, ErrorKind::SyncFile, &self.path))
    }

    /// Truncates or extends the file.
    ///
    /// This is a wrapper around [`async_fs::File::set_len`].
    pub async fn set_len(&self, size: u64) -> io::Result<()> {
        self.async_fs
            .set_len(size)
            .await
            .map_err(|err| Error::build(err, ErrorKind::SetLen, &self.path))
    }

    /// Reads the file's metadata.
    ///
    /// This is a wrapper around [`async_fs::File::metadata`].
    pub async fn metadata(&self) -> io::Result<Metadata> {
        self.async_fs
            .metadata()
            .await
            .map_err(|err| Error::build(err, ErrorKind::Metadata, &self.path))
    }

    /// Changes the permissions on the file.
    ///
    /// This is a wrapper around [`async_fs::File::set_permissions`].
    pub async fn set_permissions(&self, perm: Permissions) -> io::Result<()> {
        self.async_fs
            .set_permissions(perm)
            .await
            .map_err(|err| Error::build(err, ErrorKind::SetPermissions, &self.path))
    }
}

/// Methods added by fs-err that are not available on
/// [`async_fs::File`].
impl File {
    /// Creates a [`File`](struct.File.html) from a raw file and its path.
    pub fn from_parts<P>(file: AsyncFsFile, path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        File {
            async_fs: file,
            path: path.into(),
        }
    }

    /// Extract the raw file and its path from this [`File`](struct.File.html).
    pub fn into_parts(self) -> (AsyncFsFile, PathBuf) {
        (self.async_fs, self.path)
    }

    /// Returns a reference to the underlying [`async_fs::File`].
    pub fn file(&self) -> &AsyncFsFile {
        &self.async_fs
    }

    /// Returns a mutable reference to the underlying [`async_fs::File`].
    pub fn file_mut(&mut self) -> &mut AsyncFsFile {
        &mut self.async_fs
    }

    /// Returns a reference to the path that this file was created with.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Wrap the error in information specific to this `File` object.
    fn error(&self, source: io::Error, kind: ErrorKind) -> io::Error {
        Error::build(source, kind, &self.path)
    }
}

impl From<crate::File> for File {
    fn from(inner: crate::File) -> File {
        let (std, path) = inner.into_parts();
        File::from_parts(std.into(), path)
    }
}

impl From<File> for AsyncFsFile {
    fn from(f: File) -> Self {
        f.into_parts().0
    }
}

#[cfg(unix)]
impl std::os::unix::io::AsRawFd for File {
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        self.async_fs.as_raw_fd()
    }
}

#[cfg(windows)]
impl std::os::windows::io::AsRawHandle for File {
    fn as_raw_handle(&self) -> std::os::windows::io::RawHandle {
        self.async_fs.as_raw_handle()
    }
}

#[cfg(unix)]
impl std::os::unix::io::AsFd for File {
    fn as_fd(&self) -> std::os::unix::io::BorrowedFd<'_> {
        self.async_fs.as_fd()
    }
}

#[cfg(windows)]
impl std::os::windows::io::AsHandle for File {
    fn as_handle(&self) -> std::os::windows::io::BorrowedHandle<'_> {
        self.async_fs.as_handle()
    }
}

impl AsyncRead for File {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Poll::Ready(
            ready!(Pin::new(&mut self.async_fs).poll_read(cx, buf))
                .map_err(|err| self.error(err, ErrorKind::Read)),
        )
    }
}

impl AsyncWrite for File {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Poll::Ready(
            ready!(Pin::new(&mut self.async_fs).poll_write(cx, buf))
                .map_err(|err| self.error(err, ErrorKind::Write)),
        )
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(
            ready!(Pin::new(&mut self.async_fs).poll_flush(cx))
                .map_err(|err| self.error(err, ErrorKind::Flush)),
        )
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(
            ready!(Pin::new(&mut self.async_fs).poll_close(cx))
                .map_err(|err| self.error(err, ErrorKind::Flush)),
        )
    }
}

impl AsyncSeek for File {
    fn poll_seek(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        pos: SeekFrom,
    ) -> Poll<io::Result<u64>> {
        Pin::new(&mut self.async_fs)
            .poll_seek(cx, pos)
            .map_err(|err| self.error(err, ErrorKind::Seek))
    }
}
