//! [`CacheLocation`].

use std::path::{Path, PathBuf};
use std::ffi::{OsStr, OsString};

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

macro_rules! into_cache_location_impl {
    ($($t:ty),*) => {
        $(
            impl From<$t> for CacheLocation {
                /// [`Self::Path`].
                fn from(path: $t) -> Self {
                    Self::Path(path.into())
                }
            }

            impl From<Option<$t>> for CacheLocation {
                /// If [`Some`], [`Self::Path`]. If [`None`], [`Self::Memory`].
                fn from(path: Option<$t>) -> Self {
                    match path {
                        Some(path) => path.into(),
                        None => Self::Memory
                    }
                }
            }
        )*
    }
}

into_cache_location_impl!{&str, String, &OsStr, OsString, &Path, PathBuf}
