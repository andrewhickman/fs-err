use crate::async_fs::file::File;
use crate::errors::{Error, ErrorKind};
use crate::private::Sealed;
#[cfg(unix)]
use async_fs::unix::OpenOptionsExt;
#[cfg(windows)]
use async_fs::windows::OpenOptionsExt;
use std::io;
use std::path::Path;

/// A builder for opening files with configurable options.
///
/// This is a wrapper around [`async_fs::OpenOptions`].
#[derive(Clone, Debug)]
pub struct OpenOptions(async_fs::OpenOptions);

impl OpenOptions {
    /// Creates a blank set of options.
    ///
    /// This is a wrapper around [`async_fs::OpenOptions::new`].
    pub fn new() -> OpenOptions {
        OpenOptions(async_fs::OpenOptions::new())
    }

    /// Configures the option for read mode.
    ///
    /// This is a wrapper around [`async_fs::OpenOptions::read`].
    pub fn read(&mut self, read: bool) -> &mut OpenOptions {
        self.0.read(read);
        self
    }

    /// Configures the option for write mode.
    ///
    /// This is a wrapper around [`async_fs::OpenOptions::write`].
    pub fn write(&mut self, write: bool) -> &mut OpenOptions {
        self.0.write(write);
        self
    }

    /// Configures the option for append mode.
    ///
    /// This is a wrapper around [`async_fs::OpenOptions::append`].
    pub fn append(&mut self, append: bool) -> &mut OpenOptions {
        self.0.append(append);
        self
    }

    /// Configures the option for truncating the previous file.
    ///
    /// This is a wrapper around [`async_fs::OpenOptions::truncate`].
    pub fn truncate(&mut self, truncate: bool) -> &mut OpenOptions {
        self.0.truncate(truncate);
        self
    }

    /// Configures the option for creating a new file if it doesn't exist.
    ///
    /// This is a wrapper around [`async_fs::OpenOptions::create`].
    pub fn create(&mut self, create: bool) -> &mut OpenOptions {
        self.0.create(create);
        self
    }

    /// Configures the option for creating a new file or failing if it already exists.
    ///
    /// This is a wrapper around [`async_fs::OpenOptions::create_new`].
    pub fn create_new(&mut self, create_new: bool) -> &mut OpenOptions {
        self.0.create_new(create_new);
        self
    }

    /// Opens a file with the configured options.
    ///
    /// This is a wrapper around [`async_fs::OpenOptions::open`].
    pub async fn open<P: AsRef<Path>>(&self, path: P) -> io::Result<File> {
        let path = path.as_ref();
        Ok(File::from_parts(
            self.0
                .open(path)
                .await
                .map_err(|err| Error::build(err, ErrorKind::OpenFile, path))?,
            path,
        ))
    }
}

impl Default for OpenOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl Sealed for OpenOptions {}

#[cfg(unix)]
impl crate::os::unix::fs::OpenOptionsExt for OpenOptions {
    fn mode(&mut self, mode: u32) -> &mut Self {
        self.0.mode(mode);
        self
    }

    fn custom_flags(&mut self, flags: i32) -> &mut Self {
        self.0.custom_flags(flags);
        self
    }
}

#[cfg(windows)]
impl crate::os::windows::fs::OpenOptionsExt for OpenOptions {
    fn access_mode(&mut self, access: u32) -> &mut Self {
        self.0.access_mode(access);
        self
    }

    fn share_mode(&mut self, val: u32) -> &mut Self {
        self.0.share_mode(val);
        self
    }

    fn custom_flags(&mut self, flags: u32) -> &mut Self {
        self.0.custom_flags(flags);
        self
    }

    fn attributes(&mut self, val: u32) -> &mut Self {
        self.0.attributes(val);
        self
    }

    fn security_qos_flags(&mut self, flags: u32) -> &mut Self {
        self.0.security_qos_flags(flags);
        self
    }
}
