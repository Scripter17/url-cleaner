//! Utilities.

use crate::prelude::*;

pub mod stdin;
pub mod tool;
pub mod site_client;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::stdin::*;
    pub use super::tool::prelude::*;
    pub use super::site_client::*;

    pub use super::fresh_dir;
}

/// Delete and remake a directory.
pub fn fresh_dir<P: AsRef<Path>>(dir: P) {
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
}
