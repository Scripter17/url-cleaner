//! Utilities.

use crate::prelude::*;

mod stdin;
mod params_diff;
mod tool;
mod site_client;

pub use stdin::*;
pub use params_diff::*;
pub use tool::*;
pub use site_client::*;

/// Delete and remake a directory.
pub fn fresh_dir<P: AsRef<Path>>(dir: P) {
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
}
