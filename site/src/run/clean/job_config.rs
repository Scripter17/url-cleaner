//! [`JobConfig`].

use serde::{Serialize, Deserialize};

use super::*;

/** Serde helper. **/ pub(crate) fn is_default<T: Default + PartialEq>(x: &T) -> bool {x == &T::default()}
/** Serde helper. **/ pub(crate) fn is_true (x: &bool) -> bool {*x}
/** Serde helper. **/ pub(crate) fn get_true(        ) -> bool {true}

/// Config for a `/clean` job.
///
/// Given as JSON text in either the `config` query parameter XOR the `X-Config` header.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JobConfig {
    /// The username to use.
    ///
    /// Defaults to [`None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub username: Option<String>,
    /// The password to use.
    ///
    /// Defaults to [`None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub password: Option<String>,
    /// The [`JobContext`] to use.
    ///
    /// Defaults to [`JobContext::default`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub context: JobContext,
    /// The profile to use.
    ///
    /// Defaults to [`None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub profile: Option<String>,
    /// The [`ParamsDiff`] to use on top of the profile.
    ///
    /// Defaults to [`None`].
    #[serde(default, skip_serializing_if = "ParamsDiff::is_empty")]
    pub params_diff: ParamsDiff,
    /// If [`true`], enable brief unchanged mode.
    ///
    /// Defaults to false.
    #[serde(default, skip_serializing_if = "is_default")]
    pub brief_unchanged: bool,
    /// If [`true`], enable brief unchanged mode.
    ///
    /// Defaults to false.
    #[serde(default, skip_serializing_if = "is_default")]
    pub brief_error: bool,
    /// If [`true`], enable unhtreading.
    ///
    /// Defaults to [`false`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub unthread: bool,
    /// If [`true`], don't disable the HTTP Client.
    ///
    /// Defaults to [`true`].
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub http: bool,
    /// If [`true`], don't disable reading from the cache.
    ///
    /// Defaults to [`true`].
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub read_cache: bool,
    /// If [`true`], don't disable writing to the cache.
    ///
    /// Defaults to [`true`].
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub write_cache: bool,
    /// If [`true`], enable cache delays.
    ///
    /// Defaults to [`false`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub cache_delay: bool,
}

impl Default for JobConfig {
    fn default() -> Self {
        Self {
            username       : None,
            password       : None,
            context        : Default::default(),
            profile        : None,
            params_diff    : Default::default(),
            brief_unchanged: false,
            brief_error    : false,
            unthread       : false,
            http           : true,
            read_cache     : true,
            write_cache    : true,
            cache_delay    : false,
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
