//! The `/clean_ws` route.

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
        let (ms, mut mr) = tokio::sync::mpsc::channel::<Vec<u8>>(8);
        let (rs, mut rr) = tokio::sync::mpsc::channel::<Option<String>>(128);

        std::thread::spawn(move || {
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

            let (iss, irs) = (0..state.config.threads.get()).map(|_| std::sync::mpsc::channel()).collect::<(Vec<_>, Vec<_>)>();
            let (oss, ors) = (0..state.config.threads.get()).map(|_| std::sync::mpsc::channel()).collect::<(Vec<_>, Vec<_>)>();

            std::thread::scope(|s| {
                s.spawn(move || {
                    let mut isi = iss.iter().cycle();
                    while let Some(message) = mr.blocking_recv() {
                        for (line, is) in message.split(|x| *x == b'\n').map(|x| x.strip_suffix(b"\r").unwrap_or(x)).filter(|x| !x.is_empty()).zip(&mut isi) {
                            is.send(Some(line.to_owned())).expect("???");
                        }
                        isi.next().expect("???").send(None).expect("???");
                    }
                });

                for (ir, os) in irs.into_iter().zip(oss) {
                    s.spawn(move || {
                        while let Ok(task) = ir.recv() {
                            os.send(task.map(|task| match job.r#do(task) {
                                Ok (x) => x.into(),
                                Err(e) => format!("-{e:?}")
                            })).expect("???");
                        }
                    });
                }

                for or in ors.iter().cycle() {
                    match or.recv() {
                        Ok (x) => rs.blocking_send(x).expect("???"),
                        Err(_) => break
                    }
                }
            });
        });

        while let Some(message) = stream.next().await {
            let message = match message? {
                ws::Message::Binary(bytes) => bytes,
                ws::Message::Text(text) => text.into(),
                _ => continue
            };

            ms.send(message).await.expect("???");

            let mut buf = String::new();
            loop {
                tokio::select! {
                    result = rr.recv() => match result.flatten() {
                        Some(x) => {
                            if !buf.is_empty() {buf.push('\n');}
                            buf.push_str(&x);
                            if buf.len() > 65536 {
                                stream.send(buf.into()).await?;
                                buf = String::new();
                            }
                        },
                        None => {
                            if !buf.is_empty() {
                                stream.send(buf.into()).await?;
                            }
                            break;
                        }
                    },
                    _ = tokio::time::sleep(std::time::Duration::from_nanos(10)) => if !buf.is_empty() {
                        stream.send(buf.into()).await?;
                        buf = String::new();
                    }
                }
            }
        }

        Ok(())
    })))
}
