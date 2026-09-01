//! [`JobConfig`].

use serde::{Serialize, Deserialize};

use super::*;

/** Serde helper. **/ pub(crate) fn is_default<T: Default + PartialEq>(x: &T) -> bool {x == &T::default()}
/** Serde helper. **/ pub(crate) fn is_true (x: &bool) -> bool {*x}
/** Serde helper. **/ pub(crate) fn get_true(        ) -> bool {true}

/// Config for a `/clean` job.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JobConfig {
    /// The password to use.
    ///
    /// Defaults to [`None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub password: Option<String>,
    /// The [`JobContext`] to use.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub context: JobContext,
    /// The profile to use.
    ///
    /// Defaults to [`None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub profile: Option<String>,
    /// The [`ParamsDiff`] to use on top of the profile.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "ParamsDiff::is_empty")]
    pub params_diff: ParamsDiff,
    /// If [`true`], unchanged lines are replaced with `=`.
    ///
    /// Defaults to false.
    #[serde(default, skip_serializing_if = "is_default")]
    pub brief_unchanged: bool,
    /// If [`true`], error lines are replaced with `-`.
    ///
    /// Defaults to false.
    #[serde(default, skip_serializing_if = "is_default")]
    pub brief_error: bool,
    /// If [`true`], hide the threads.
    ///
    /// Defaults to [`false`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub hide_threads: bool,
    /// If [`false`], disable the HTTP Client.
    ///
    /// Defaults to [`true`].
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub http: bool,
    /// If [`false`], disable reading from the cache.
    ///
    /// Defaults to [`true`].
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub read_cache: bool,
    /// If [`false`], disable writing to the cache.
    ///
    /// Defaults to [`true`].
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub write_cache: bool,
    /// If [`true`], hide the cache.
    ///
    /// Defaults to [`false`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub hide_cache: bool,
}

impl Default for JobConfig {
    fn default() -> Self {
        Self {
            password       : None,
            context        : Default::default(),
            profile        : None,
            params_diff    : Default::default(),
            brief_unchanged: false,
            brief_error    : false,
            hide_threads   : false,
            http           : true,
            read_cache     : true,
            write_cache    : true,
            hide_cache     : false,
        }
    }
}

/// The error from failing to get a [`JobConfig`].
#[derive(Debug, Error)]
pub enum GetJobConfigError {
    /// [`serde_json::Error`].
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    /// Returned when a request has a `config` query param with no value.
    #[error("The request had a `config` query param with no value.")]
    EmptyConfigParam,
    /// Returned when a request attempted to set the [`JobConfig`] twice.
    #[error("The request attempted to set the JobConfig twice.")]
    ConfigSetTwice,
}

impl IntoResponse for GetJobConfigError {
    fn into_response(self) -> Response<Body> {
        (StatusCode::BAD_REQUEST, format!("{self:?}")).into_response()
    }
}

impl<S: Sync> FromRequestParts<S> for JobConfig {
    type Rejection = GetJobConfigError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        Ok(match (MaybeSpecialQuery::from(parts.uri.query()).find("config", 0), parts.headers.get("x-config")) {
            (None        , None        ) => Default::default(),
            (None        , Some(config)) => serde_json::from_slice(config.as_bytes())?,
            (Some(config), None        ) => serde_json::from_str(&config.into_value().lossy_decode().ok_or(GetJobConfigError::EmptyConfigParam)?)?,
            (Some(_)     , Some(_)     ) => Err(GetJobConfigError::ConfigSetTwice)?,
        })
    }
}
