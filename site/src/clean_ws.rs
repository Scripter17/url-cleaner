//! The `/clean_ws` route.

use std::fmt::Write;

use rocket_ws as ws;
use rocket::futures::{SinkExt, StreamExt};

use url_cleaner_engine::prelude::*;

use crate::*;

/// The `/clean_ws` route.
#[get("/clean_ws")]
pub async fn clean_ws(state: &State<&'static ServerState>, config: JobConfig, ws: ws::WebSocket) -> Result<ws::Channel<'static>, CleanError> {
    let state = *state.inner();

    match (&state.config.passwords, &config.password) {
        (None           , None          ) => {},
        (None           , Some(_)       ) => Err(CleanError {status: 400, message: "This instance requires no password.".into()})?,
        (Some(_)        , None          ) => Err(CleanError {status: 401, message: "Password required.".into()})?,
        (Some(passwords), Some(password)) => if !passwords.contains(password) {Err(CleanError {status: 401, message: "Invalid password".into()})?}
    }

    let Some(mut cleaner) = state.config.profiled_cleaner.get(config.profile.as_deref()) else {
        Err(CleanError {status: 400, message: format!("Unknown profile: {:?}", config.profile)})?
    };

    if let Some(params_diff) = config.params_diff {
        params_diff.apply(&mut cleaner.params);
    }

    Ok(ws.channel(move |mut stream| Box::pin(async move {
        let job = &Job {
            context: config.context,
            cleaner,
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
                    let mut ret = String::new();
                    let mut lines = text.lines().filter(|line| !line.is_empty());
                    if let Some(line) = lines.next() {
                        match job.r#do(line) {
                            Ok (x) => ret = x.into(),
                            Err(e) => ret = format!("-{e:?}")
                        }
                    }
                    for line in lines {
                        ret.push('\n');
                        match job.r#do(line) {
                            Ok (x) => ret.push_str(x.as_str()),
                            Err(e) => write!(ret, "-{e:?}").expect("This to always work.")
                        }
                    }
                    stream.send(ret.into()).await?;
                },
                ws::Message::Binary(bytes) => {
                    let mut ret = String::new();
                    let mut lines = crate::util::ByteLines(&bytes).filter(|line| !line.is_empty());
                    if let Some(line) = lines.next() {
                        match job.r#do(line) {
                            Ok (x) => ret = x.into(),
                            Err(e) => ret = format!("-{e:?}")
                        }
                    }
                    for line in lines {
                        ret.push('\n');
                        match job.r#do(line) {
                            Ok (x) => ret.push_str(x.as_str()),
                            Err(e) => write!(ret, "-{e:?}").expect("This to always work.")
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
