//! `/clean` stuff.

use serde::{Serialize, Deserialize};

use crate::prelude::*;

/// Config for a `/clean` or `/clean_ws` payload.
///
/// Given as JSON text in either the `config` query parameter XOR the `X-Config` header.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JobConfig {
    /// The password to use.
    ///
    /// Defaults to [`None`].
    pub password: Option<String>,
    /// The [`JobContext`] to use.
    ///
    /// Defaults to [`JobContext::default`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub context: JobContext,
    /// The [`Profile`] to use.
    ///
    /// Applied before [`Self::params_diff`].
    ///
    /// Defaults to [`None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub profile: Option<String>,
    /// The [`ParamsDiff`] to use.
    ///
    /// Applied after [`Self::profile`].
    ///
    /// Defaults to [`None`].
    #[serde(default, skip_serializing_if = "ParamsDiff::is_empty")]
    pub params_diff: ParamsDiff,
    /// If [`true`], enable reading from the cache.
    ///
    /// Exists unconditionally even when the URL Cleaner Site has the `cache` feature disabled.
    ///
    /// It's just easier like this.
    ///
    /// Defaults to [`true`].
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub read_cache: bool,
    /// If [`true`], enable writing to the cache.
    ///
    /// Exists unconditionally even when the URL Cleaner Site has the `cache` feature disabled.
    ///
    /// It's just easier like this.
    ///
    /// Defaults to [`true`].
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub write_cache: bool,
    /// If [`true`], enable cache delays.
    ///
    /// Exists unconditionally even when the URL Cleaner Site has the `cache` feature disabled.
    ///
    /// It's just easier like this.
    ///
    /// Defaults to [`false`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub cache_delay: bool,
    /// If [`true`], enable unhtreading.
    ///
    /// Defaults to [`false`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub unthread: bool
}
