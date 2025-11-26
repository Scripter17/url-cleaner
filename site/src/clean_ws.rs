//! The `/clean_ws` route.

use std::fmt::Write;

use rocket_ws as ws;
use rocket::futures::{SinkExt, StreamExt};

use url_cleaner_engine::prelude::*;

use crate::*;

/// The `/clean_ws` route.
#[get("/clean_ws")]
pub async fn clean_ws(state: &State<&'static ServerState>, config: CleanConfig, ws: ws::WebSocket) -> Result<ws::Channel<'static>, CleanError> {
    let state = *state.inner();

    let Some(mut cleaner) = state.config.profiled_cleaner.get(config.profile.as_deref()) else {
        Err(CleanError {status: 400, message: format!("Unknown profile: {:?}", config.profile)})?
    };

    if let Some(params_diff) = config.params_diff {
        params_diff.apply(&mut cleaner.params);
    }

    Ok(ws.channel(move |mut stream| Box::pin(async move {
        let job_config = JobConfig {
            context: &config.context,
            cleaner: &cleaner,
            unthreader: state.unthreader.filter(config.unthread),
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
                ws::Message::Text(text) => {
                    let mut ret = String::with_capacity(64 * text.len().checked_ilog2().unwrap_or(0).pow(2) as usize);
                    for line in text.lines() {
                        match job_config.do_lazy_task_config(line) {
                            Ok (x) => writeln!(ret, "{x}"   ).expect("This to always work."),
                            Err(e) => writeln!(ret, "-{e:?}").expect("This to always work.")
                        }
                    }
                    stream.send(ret.into()).await?;
                },
                ws::Message::Binary(bytes) => {
                    let mut ret = String::with_capacity(64 * bytes.len().checked_ilog2().unwrap_or(0).pow(2) as usize);
                    for line in crate::util::ByteLines::new(&bytes) {
                        match job_config.do_lazy_task_config(line) {
                            Ok (x) => writeln!(ret, "{x}").expect("This to always work."),
                            Err(e) => writeln!(ret, "-{e:?}").expect("This to always work.")
                        }
                    }
                    stream.send(ret.into()).await?;
                },
                _ => {}
            }
        }

        Ok(())
    })))
}
