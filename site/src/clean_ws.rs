//! The `/clean_ws` route.

use std::sync::Arc;
use std::fmt::Write;
use std::sync::mpsc::TryRecvError;

use tokio::sync::Mutex;
use rocket_ws as ws;
use rocket::futures::{SinkExt, StreamExt};

use url_cleaner_engine::prelude::*;

use crate::*;

/// The `/clean_ws` route.
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

    Ok(ws.channel(move |stream| Box::pin(async move {
        let (sink, mut stream) = stream.split();

        let sink = Arc::new(Mutex::new(sink));
        let sink2 = sink.clone();
        let (os, or) = std::sync::mpsc::channel::<Result<String, String>>();

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(1)).await;
                let mut buf = String::new();
                loop {
                    match or.try_recv() {
                        Ok(x) => match x {
                            Ok (x) => writeln!(buf, "Ok\t{x}"),
                            Err(e) => writeln!(buf, "Err\t{e}")
                        }.expect("Write a result to the buffer to never fail."),
                        Err(TryRecvError::Empty) => break,
                        Err(TryRecvError::Disconnected) => return
                    }
                }
                if !buf.is_empty() {
                    let _ = sink2.lock().await.send(buf.into()).await;
                }
            }
        });

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
                    os.send(match job_config.make_small_lazy_task(line.into()).make() {
                        Ok(task) => match task.r#do() {
                            Ok(x) => Ok(x.into()),
                            Err(e) => Err(format!("{e:?}"))
                        },
                        Err(e) => Err(format!("{:?}", DoTaskError::from(e)))
                    }).expect("The receiver to still be open.");
                },
                ws::Message::Close(x) => {let _ = sink.lock().await.send(ws::Message::Close(x)).await; break},
                ws::Message::Ping(x) => sink.lock().await.send(ws::Message::Pong(x)).await?,
                _ => {}
            }
        }

        Ok(())
    })))
}
