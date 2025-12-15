//! Filesystem stuff.

use super::prelude::*;

pub use std::os::fd::AsRawFd;

/// Make a temporary file.
pub fn tmp_file(path: &str) -> TmpFileHandle {
    if let Some(dir) = Path::new(path).parent() {std::fs::create_dir_all(dir).unwrap();}
    TmpFileHandle {
        path: path.to_owned(),
        file: std::fs::OpenOptions::new().create(true).truncate(true).read(true).write(true).open(path).unwrap()
    }
}

/// A temporary file that's removed when dropped.
#[derive(Debug)]
pub struct TmpFileHandle {
    /// The path of the file.
    path: String,
    /// The file.
    file: File
}

impl std::ops::Drop for TmpFileHandle {
    fn drop(&mut self) {
        std::fs::remove_file(&self.path).unwrap();
    }
}

impl TmpFileHandle {
    /// Get the path of the file.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Get the file.
    pub fn file(& self) -> &File {
        &self.file
    }

    /// Get the file descriptor.
    pub fn fd(&self) -> std::os::fd::RawFd {
        self.file.as_raw_fd()
    }
}
