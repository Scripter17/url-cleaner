//! The `/clean_ws` route.

use rocket_ws as ws;
use rocket::futures::{SinkExt, StreamExt};

use url_cleaner_engine::prelude::*;

use crate::*;

/// The `/clean_ws` route.
///
/// A WebSocket channel that's set up by supplying each field of [`CleanPayloadConfig`] as a query parameter with JSON values, takes lines of [`SmallLazyTaskConfig::Str`], and returns lines of either `Ok\t{url}` or `Err\t{e:?}`.
///
/// Currently only single threaded due to complexities of how Rust and Rocket and WebSockets all work together.
#[get("/clean_ws")]
pub async fn clean_ws<'a>(auth: Auth, config: CleanPayloadConfig, state: &'a State<ServerState>, ws: ws::WebSocket) -> Result<ws::Channel<'a>, CleanError> {
    if !state.config.accounts.check(&auth) {
        Err(Status::Unauthorized)?
    }

    let Some(mut cleaner) = state.config.profiled_cleaner.get(config.profile.as_deref()) else {
        Err(CleanError {status: 400, message: format!("Unknown profile: {:?}", config.profile)})?
    };

    if let Some(params_diff) = config.params_diff {
        params_diff.apply(&mut cleaner.params);
    }

    let unthreader = match config.unthread {
        false => &NO_UNTHREADER,
        true  => &state.unthreader
    };

    Ok(ws.channel(move |mut stream| Box::pin(async move {
        let job_config = JobConfig {
            context: &config.context,
            cleaner: &cleaner,
            unthreader,
            #[cfg(feature = "cache")]
            cache: Cache {
                inner: &state.inner_cache,
                config: CacheConfig {
                    read : config.read_cache,
                    write: config.write_cache,
                    delay: config.cache_delay
                }
            },
            #[cfg(feature = "http")]
            http_client: &state.http_client
        };

        while let Some(message) = stream.next().await {
            match message? {
                ws::Message::Text(text) => for line in text.lines() {
                    match job_config.make_small_lazy_task(line.to_string().into()).make().map_err(Into::into).and_then(Task::r#do) {
                        Ok (url) => stream.send(format!("Ok\t{url}" ).into()).await.expect("Sending a WebSocket response to work."),
                        Err(e  ) => stream.send(format!("Err\t{e:?}").into()).await.expect("Sending a WebSocket response to work."),
                    }
                },
                ws::Message::Close(x) => {let _ = stream.send(ws::Message::Close(x)).await; break},
                ws::Message::Ping(x) => stream.send(ws::Message::Pong(x)).await.expect("Sending a WebSocket pong to work."),
                _ => {}
            }
        }

        Ok(())
    })))
}
