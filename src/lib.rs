//! This crate provides the [`File`](struct.File.html) type, a wrapper
//! around a file handle and its path which wraps all operations with
//! more helpful error messages.

#![doc(html_root_url = "https://docs.rs/fs-err/0.1.0")]
#![deny(missing_debug_implementations, missing_docs)]

/// A wrapper around a file handle and its path which wraps all
/// operations with more helpful error messages.
#[derive(Debug)]
pub struct File;
