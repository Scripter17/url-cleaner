//! `/clean` stuff.

use serde::{Serialize, Deserialize};

use url_cleaner_engine::types::*;

use crate::util::*;

/// The payload of the `/clean` route.
/// 
/// Used to construct a [`Job`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobConfig {
    /// The [`LazyTaskConfig`]s to use.
    pub tasks: Vec<LazyTaskConfig>,
    /// The [`JobContext`] to use.
    #[serde(default, skip_serializing_if = "is_default")]
    pub context: JobContext,
    /// The [`ParamsDiff`] to use.
    #[serde(default, skip_serializing_if = "is_default")]
    pub params_diff: Option<ParamsDiff>,
    #[allow(rustdoc::broken_intra_doc_links, reason = "Fixing it would require bloating the dependency tree.")]
    /// if [`Some`], overwrite [`Job::cache_delay`].
    ///
    /// If untrusted parties can detect the presence of the cache delay, it should be enabled.
    #[cfg(feature = "cache")]
    #[serde(default, skip_serializing_if = "is_default")]
    pub cache_delay: Option<bool>
}

/// The [`Result`] returned by the `/clean` route.
pub type CleanResult = Result<CleanSuccess, CleanError>;

/// The success state of doing a [`JobConfig`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CleanSuccess {
    /// The [`Task`] results.
    pub urls: Vec<Result<BetterUrl, String>>
}

/// The error state of doing a [`JobConfig`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CleanError {
    /// The HTTP status code.
    pub status: u16,
    /// The error meesage.
    pub message: String
}
