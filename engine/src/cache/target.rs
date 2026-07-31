//! [`CacheTarget`].

use crate::prelude::*;

use std::path::{Path, PathBuf};

/// Where to put a cache.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CacheTarget {
    /** In a file. **/ File(PathBuf),
    /** In memory. **/ Memory,
}

impl FromStr for CacheTarget {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            ":memory:" => Self::Memory,
            _ => Self::File(s.parse()?)
        })
    }
}

impl From<&Path  > for CacheTarget {fn from(value: &Path  ) -> Self {Self::File(value.into())}}
impl From<PathBuf> for CacheTarget {fn from(value: PathBuf) -> Self {Self::File(value       )}}
impl From<&str   > for CacheTarget {fn from(value: &str   ) -> Self {value.to_string().into()}}

impl From<String> for CacheTarget {
    fn from(value: String) -> Self {
        match &*value {
            ":memory:" => Self::Memory,
            _          => Self::File(value.into()),
        }
    }
}
