//! [`CacheLocation`].

use std::path::{Path, PathBuf};

use serde::{Serialize, Deserialize};

#[expect(unused_imports, reason = "Used in a doc comment.")]
use crate::prelude::*;

/// The location of a [`Cache`].
///
/// Defaults to [`Self::Memory`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum CacheLocation {
    /// In memory.
    ///
    /// The default.
    #[default]
    Memory,
    /// A [`PathBuf`].
    Path(PathBuf)
}

impl From<PathBuf> for CacheLocation {
    fn from(path: PathBuf) -> Self {
        Self::Path(path)
    }
}

impl From<&Path> for CacheLocation {
    fn from(path: &Path) -> Self {
        Self::Path(path.into())
    }
}
