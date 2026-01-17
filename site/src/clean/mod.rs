//! `/clean`.

use std::sync::Arc;

use axum::{
    http::StatusCode,
    extract::{State, Request, WebSocketUpgrade, FromRequest, FromRequestParts},
    response::Response,
    body::Body
};

use url_cleaner_engine::prelude::*;
use url_cleaner_site_types::prelude::*;

mod ws;
mod http;

/// Unified API for the WebSocket and HTTP APIs.
#[derive(Debug)]
pub enum CleanPayload {
    /// WebSocket.
    Ws(WebSocketUpgrade),
    /// HTTP.
    Http(Body)
}

impl<S: Send + Sync> FromRequest<S> for CleanPayload {
    type Rejection = std::convert::Infallible;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();

        Ok(match WebSocketUpgrade::from_request_parts(&mut parts, state).await {
            Ok (wsu) => Self::Ws(wsu),
            Err(_  ) => Self::Http(body)
        })
    }
}

/// `/clean`.
pub async fn clean(state: State<&'static crate::State>, job_config: JobConfig, x: CleanPayload) -> Result<Response, (StatusCode, &'static str)> {
    match (&state.passwords, job_config.password) {
        (None           , None          ) => {},
        (None           , Some(_       )) => Err((StatusCode::UNAUTHORIZED, "Requires no password"))?,
        (Some(_        ), None          ) => Err((StatusCode::UNAUTHORIZED, "Requires password"))?,
        (Some(passwords), Some(password)) => if !passwords.contains(&password) {Err((StatusCode::UNAUTHORIZED, "Invalid password"))?}
    }

    let mut cleaner = state.profiled_cleaner.get(job_config.profile.as_deref()).ok_or((StatusCode::BAD_REQUEST, "Unknown profile"))?;
    job_config.params_diff.apply(&mut cleaner.params);

    let job = Arc::new(Job {
        context: job_config.context,
        cleaner,
        unthreader: state.unthreader.filter(job_config.unthread),
        #[cfg(feature = "cache")]
        cache: Cache {
            inner: &state.inner_cache,
            config: CacheConfig {
                read : job_config.read_cache,
                write: job_config.write_cache,
                delay: job_config.cache_delay
            }
        },
        #[cfg(feature = "http")]
        http_client: &state.http_client
    });

    match x {
        CleanPayload::Ws  (wsu ) => ws  ::clean_ws  (state, job, wsu ).await,
        CleanPayload::Http(body) => http::clean_http(state, job, body).await,
    }
}

