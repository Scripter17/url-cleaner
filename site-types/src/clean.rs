//! `/clean` stuff.

use serde::{Serialize, Deserialize};

use url_cleaner_engine::prelude::*;
use better_url::BetterUrl;

use crate::util::*;

/// The payload of the `/clean` route.
///
/// Used to construct a [`Job`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(bound(deserialize = "'a: 'de, 'de: 'a"))]
pub struct CleanPayload<'a> {
    /// The [`LazyTaskConfig`]s to use.
    #[serde(borrow)]
    pub tasks: Vec<SmallLazyTaskConfig<'a>>,
    /// The [`CleanPayloadConfig`] with `#[serde(flatten)]` applied.
    #[serde(flatten)]
    pub config: CleanPayloadConfig
}

impl<'a> CleanPayload<'a> {
    /// Make each contained [`LazyTaskConfig`] owned.
    pub fn into_owned(self) -> CleanPayload<'static> {
        CleanPayload {
            tasks: self.tasks.into_iter().map(SmallLazyTaskConfig::into_owned).collect(),
            ..self
        }
    }
}

/// [`CleanResult`] but small.
pub type SmallCleanResult = Result<SmallCleanSuccess, CleanError>;

/// [`CleanSuccess`] but small.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SmallCleanSuccess {
    /// The [`Task`] results.
    pub urls: Vec<Result<String, String>>
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

/// Config for a `/clean` or `/clean_ws` payload.
///
/// When used in `/clean`, exists in a flattened form in the [`CleanPayload`].
///
/// When used in `/clean_ws`, each field is sent as a query parameter with JSON values.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CleanPayloadConfig {
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
    #[serde(default, skip_serializing_if = "is_default")]
    pub params_diff: Option<ParamsDiff>,
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
