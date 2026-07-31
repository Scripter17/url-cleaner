//! [`CacheConfig`].

use crate::prelude::*;

/// Configuration for a [`CacheClient`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheConfig {
    /// If [`false`], make all reads do nothing and return [`None`].
    ///
    /// Defaulted to [`true`].
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub read: bool,
    /// If [`false`], make all writes do nothing and return [`Ok`].
    ///
    /// Defaulted to [`true`].
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub write: bool,
    /// If [`true`], make all reads take at least as long as the entry's duration column.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub delay: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            read : true ,
            write: true ,
            delay: false,
        }
    }
}
