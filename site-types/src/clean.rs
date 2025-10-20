//! `/clean` stuff.

use serde::{Serialize, Deserialize};

use url_cleaner_engine::prelude::*;

use crate::util::*;
use crate::auth::*;

/// The payload of the `/clean` route.
///
/// Used to construct a [`Job`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(bound(deserialize = "'a: 'de, 'de: 'a"))]
pub struct CleanPayload<'a> {
    /// The [`LazyTaskConfig`]s to use.
    #[serde(borrow)]
    pub tasks: Vec<LazyTaskConfig<'a>>,
    /// The [`CleanPayloadConfig`] to use.
    ///
    /// Flattened in serialization.
    #[serde(flatten)]
    pub config: CleanPayloadConfig
}

/// The config or a [`CleanPayload`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CleanPayloadConfig {
    /// The authentication to use.
    ///
    /// Defaults to [`None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub auth: Option<Auth>,
    /// The [`JobContext`] to use.
    ///
    /// Defaults to [`None`].
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
    #[serde(default, skip_serializing_if = "is_default")]
    pub params_diff: Option<ParamsDiff>,
    #[allow(rustdoc::broken_intra_doc_links, reason = "Fixing it would require bloating the dependency tree.")]
    /// if [`Some`], overwrite [`Job::cache_handle_config`]'s [`CacheHandleConfig::read`].
    ///
    /// Defaults to [`None`].
    #[cfg(feature = "cache")]
    #[serde(default, skip_serializing_if = "is_default")]
    pub read_cache: Option<bool>,
    #[allow(rustdoc::broken_intra_doc_links, reason = "Fixing it would require bloating the dependency tree.")]
    /// if [`Some`], overwrite [`Job::cache_handle_config`]'s [`CacheHandleConfig::write`].
    ///
    /// Defaults to [`None`].
    #[cfg(feature = "cache")]
    #[serde(default, skip_serializing_if = "is_default")]
    pub write_cache: Option<bool>,
    #[allow(rustdoc::broken_intra_doc_links, reason = "Fixing it would require bloating the dependency tree.")]
    /// if [`Some`], overwrite [`Job::cache_handle_config`]'s [`CacheHandleConfig::delay`].
    ///
    /// Defaults to [`None`].
    #[cfg(feature = "cache")]
    #[serde(default, skip_serializing_if = "is_default")]
    pub cache_delay: Option<bool>,
    /// If [`Some`], enables/disables unthreading.
    ///
    /// If [`None`], uses the default value set by the server.
    ///
    /// Defaults to [`None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub unthread: Option<bool>
}

/// The [`Result`] returned by the `/clean` route.
pub type CleanResult = Result<CleanSuccess, CleanError>;

/// The success state of doing a [`JobConfig`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CleanSuccess {
    /// The [`Task`] results.
    pub urls: Vec<Result<BetterUrl, String>>
}

/// The error state of doing a [`JobConfig`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CleanError {
    /// The HTTP status code.
    pub status: u16,
    /// The error message.
    pub message: String
}
