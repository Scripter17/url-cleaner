//! [`CacheConfig`].

use serde::{Serialize, Deserialize};

use crate::prelude::*;

/// Configuration for a [`Cache`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheConfig {
    /// If true, read from the cache. If false, always return [`None`].
    ///
    /// Defaults to true.
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub read: bool,
    /// If true, write to the cache. If false, always succeed without doing anything.
    ///
    /// Defaults to true.
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub write: bool,
    /// If true, artificailly delays [`Cache::read`] to take about as long as the original cached operation.
    ///
    /// Defaults to false.
    #[serde(default = "get_false", skip_serializing_if = "is_false")]
    pub delay: bool
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            read: true,
            write: true,
            delay: false,
        }
    }
}
