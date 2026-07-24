//! `/clean`.

use super::*;

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
            Ok (wsu) => Self::Ws  (wsu),
            Err(_  ) => Self::Http(body)
        })
    }
}

impl FromRequestParts<&'static State> for &'static State {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(_: &mut Parts, state: &&'static State) -> Result<Self, Self::Rejection> {
        Ok(*state)
    }
}

/// `/clean`.
pub async fn clean(state: &'static State, job_config: JobConfig, clean_payload: CleanPayload) -> Result<Response, (StatusCode, &'static str)> {
    if !state.secrets.auth_info.check(job_config.username.as_deref(), job_config.password.as_deref()) {
        Err((StatusCode::UNAUTHORIZED, "Bad auth"))?;
    }

    let mut cleaner = state.profiled_cleaner.get(job_config.profile.as_deref()).ok_or((StatusCode::BAD_REQUEST, "Unknown profile"))?;
    job_config.params_diff.apply(&mut cleaner.params);

    let job = Job {
        context: job_config.context,
        cleaner,
        unthreader: state.unthreader.filter(job_config.unthread),
        secrets: &state.secrets,
        #[cfg(feature = "cache")]
        cache: Cache {
            inner: &state.inner_cache,
            config: CacheConfig {
                read : job_config.read_cache ,
                write: job_config.write_cache,
                delay: job_config.cache_delay,
            }
        },
        #[cfg(feature = "http")]
        http_client: &state.http_client
    };

    Ok(match clean_payload {
        CleanPayload::Ws  (wsu ) => ws  ::clean_ws  (state, job, job_config.brief_unchanged, job_config.brief_error, wsu ).await,
        CleanPayload::Http(body) => http::clean_http(state, job, job_config.brief_unchanged, job_config.brief_error, body).await,
    })
}
