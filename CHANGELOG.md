# fs-err Changelog

## 2.2.0
* Added `metadata`. ([#15](https://github.com/andrewhickman/fs-err/pull/15))

## 2.1.0
* Updated crate-level documentation. ([#8](https://github.com/andrewhickman/fs-err/pull/8))
* Added `read_dir`, `ReadDir`, and `DirEntry`. ([#9](https://github.com/andrewhickman/fs-err/pull/9))

## 2.0.1 (2020-02-22)
* Added `copy`. ([#7](https://github.com/andrewhickman/fs-err/pull/7))

## 2.0.0 (2020-02-19)
* Removed custom error type in favor of `std::io::Error`. ([#2](https://github.com/andrewhickman/fs-err/pull/2))

## 1.0.1 (2020-02-15)
* Fixed bad documentation link in `Cargo.toml`.

## 1.0.0 (2020-02-15)
* No changes from 0.1.2.

## 0.1.2 (2020-02-10)
* Added `Error::cause` implementation for `fs_err::Error`.

## 0.1.1 (2020-02-05)
* Added wrappers for `std::fs::*` functions.

## 0.1.0 (2020-02-02)
* Initial release, containing a wrapper around `std::fs::File`.
