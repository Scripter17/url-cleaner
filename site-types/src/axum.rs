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
        Ok(match (BetterMaybeRefQuery::from(parts.uri.query()).find_value("config", 0), parts.headers.get("x-config")) {
            (Some(Some(config)), None        ) => serde_json::from_str(&config)?,
            (None              , Some(config)) => serde_json::from_slice(config.as_bytes())?,
            (None              , None        ) => Default::default(),
            (Some(None)        , _           ) => Err(GetJobConfigError::EmptyConfigParam)?,
            (Some(_)           , Some(_)     ) => Err(GetJobConfigError::ConfigSetTwice)?,
        })
    }
}
