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

impl<'a> CleanPayload<'a> {
    /// Make each contained [`LazyTaskConfig`] owned.
    pub fn into_owned(self) -> CleanPayload<'static> {
        CleanPayload {
            tasks: self.tasks.into_iter().map(LazyTaskConfig::into_owned).collect(),
            ..self
        }
    }
}

/// The config or a [`CleanPayload`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    /// If [`true`], enable reading from the cache.
    ///
    /// Defaults to [`true`].
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub read_cache: bool,
    /// If [`true`], enable writing to the cache.
    ///
    /// Defaults to [`true`].
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub write_cache: bool,
    /// If [`true`], enable cache delays.
    ///
    /// Defaults to [`false`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub cache_delay: bool,
    /// If [`true`], enables unhtreading.
    ///
    /// Defaults to [`false`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub unthread: bool
}

impl Default for CleanPayloadConfig {
    fn default() -> Self {
        Self {
            auth       : None,
            context    : Default::default(),
            profile    : None,
            params_diff: None,
            read_cache : true,
            write_cache: true,
            cache_delay: false,
            unthread   : false
        }
    }
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
