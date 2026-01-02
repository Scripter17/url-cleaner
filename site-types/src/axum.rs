//! Implementations for [`axum_core`].

use axum_core::extract::FromRequestParts;
use axum_core::response::{IntoResponse, Response};
use axum_core::body::Body;
use http::status::StatusCode;
use http::request::Parts;
use thiserror::Error;

use crate::prelude::*;

/// The error from failing to get a [`JobConfig`].
#[derive(Debug, Error)]
pub enum GetJobConfigError {
    /// Returned when a [`serde_json::Error`] is encountered.
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    /// Returned when an [`std::str::Utf8Error`] is encountered.
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
    /// Returned when a request attempted to set the [`JobConfig`] twice.
    #[error("The request attempted to set the JobConfig twice.")]
    ConfigSetTwice
}

impl IntoResponse for GetJobConfigError {
    fn into_response(self) -> Response<Body> {
        (StatusCode::BAD_REQUEST, format!("{self:?}")).into_response()
    }
}

impl<S: Sync> FromRequestParts<S> for JobConfig {
    type Rejection = GetJobConfigError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        Ok(match (parts.uri.query(), parts.headers.get("x-config")) {
            (Some(query), None) => serde_json::from_str(
                // TODO: better_url::Query.
                // Which would let BetterUrl query manipulation apply all at once by having f(&mut self) -> impl Drop.
                &BetterUrl::parse(&format!("https://example.com?{query}")).expect("???")
                    .query_param("config", 0).unwrap_or(None).unwrap_or(None).unwrap_or("{}".into())
            )?,
            (None   , Some(config)) => serde_json::from_str(str::from_utf8(config.as_bytes())?)?,
            (None   , None        ) => Default::default(),
            (Some(_), Some(_)     ) => Err(GetJobConfigError::ConfigSetTwice)?
        })
    }
}
