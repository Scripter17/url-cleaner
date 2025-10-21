//! [`CacheHandleConfig`].

use serde::{Serialize, Deserialize};

use crate::prelude::*;

/// Configuration for how a [`CacheHandle`] should behave.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheHandleConfig {
    /// If [`true`], delay cache reads by about as long as the initial computation took.
    ///
    /// Used by URL Cleaner Site Userscript to reduce the ability of websites to tell if you have a URL cached.
    ///
    /// Defaults to [`false`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub delay: bool,
    /// If [`false`], make [`CacheHandle::read`] always return [`None`].
    ///
    /// Defaults to [`true`].
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub read: bool,
    /// If [`false`], make [`CacheHandle::write`] do nothing.
    ///
    /// Defaults to [`true`].
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub write: bool
}

impl Default for CacheHandleConfig {
    fn default() -> Self {
        Self {
            delay: false,
            read : true,
            write: true
        }
    }
}
