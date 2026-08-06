//! FS stuff.

use crate::prelude::*;

/// Delete and remake a directory.
pub fn fresh_dir<P: AsRef<Path>>(dir: P) {
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
}

/// Make a new writable [`File`] and its directory if needed.
pub fn new_file<P: AsRef<Path>>(path: P) -> File {
    if let Some(dir) = path.as_ref().parent() {
        std::fs::create_dir_all(dir).unwrap();
    }

    std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(true).open(path).unwrap()
}
