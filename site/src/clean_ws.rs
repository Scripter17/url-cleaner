//! The `/clean_ws` route.

use rocket_ws as ws;
use rocket::futures::{SinkExt, StreamExt};

use url_cleaner_engine::prelude::*;

use crate::*;

/// Either a task from a message or a marker that the message is done.
#[derive(Debug, Clone, PartialEq, Eq)]
enum TaskStreamThing {
    /// A task.
    Task(Vec<u8>),
    /// A marker that the message is done.
    DoneMessage
}

/// Either a result from a task or a marker that the the message the last result came from is done.
#[derive(Debug, Clone, PartialEq, Eq)]
enum ResultStreamThing {
    /// A result.
    Result(String),
    /// A marker that the message the last result came from is done.
    DoneMessage
}

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
        let (ms , mut mr ) = tokio::sync::mpsc::channel::<Vec<u8>>(8);
        let (iss,     irs) = (0..state.config.threads.get()).map(|_| std  ::sync::mpsc::channel::<TaskStreamThing  >(   )).collect::<(Vec<_>, Vec<_>)>();
        let (oss, mut ors) = (0..state.config.threads.get()).map(|_| tokio::sync::mpsc::channel::<ResultStreamThing>(128)).collect::<(Vec<_>, Vec<_>)>();

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

            std::thread::scope(move |s| {
                for (ir, os) in irs.into_iter().zip(oss) {
                    s.spawn(move || {
                        while let Ok(task) = ir.recv() {
                            os.blocking_send(match task {
                                TaskStreamThing::Task(task) => ResultStreamThing::Result(match job.r#do(task) {
                                    Ok (x) => x.into(),
                                    Err(e) => format!("-{e:?}")
                                }),
                                TaskStreamThing::DoneMessage => ResultStreamThing::DoneMessage
                            }).expect("???");
                        }
                    });
                }

                let mut isi = iss.iter().cycle();
                while let Some(message) = mr.blocking_recv() {
                    for (line, is) in message.split(|x| *x == b'\n').map(|x| x.strip_suffix(b"\r").unwrap_or(x)).filter(|x| !x.is_empty()).zip(&mut isi) {
                        is.send(TaskStreamThing::Task(line.to_owned())).expect("???");
                    }
                    isi.next().expect("???").send(TaskStreamThing::DoneMessage).expect("???");
                }
            });
        });

        let mut i = (0..ors.len()).cycle();

        while let Some(message) = stream.next().await {
            let message = match message? {
                ws::Message::Binary(bytes) => bytes,
                ws::Message::Text(text) => text.into(),
                _ => continue
            };

            ms.send(message).await.expect("???");
            let mut next = Box::pin(ors.get_mut(i.next().expect("???")).expect("???").recv());

            let mut buf = String::new();

            loop {
                match tokio::time::timeout(std::time::Duration::from_millis(10), &mut next).await {
                    Ok(Some(ResultStreamThing::Result(x))) => {
                        if !buf.is_empty() {buf.push('\n');}
                        buf.push_str(&x);
                        if buf.len() > 65536 {
                            stream.send(buf.into()).await?;
                            buf = String::new();
                        }
                        drop(next);
                        next = Box::pin(ors.get_mut(i.next().expect("???")).expect("???").recv());
                    },
                    Ok(None | Some(ResultStreamThing::DoneMessage)) => {
                        if !buf.is_empty() {
                            stream.send(buf.into()).await?;
                        }
                        break;
                    },
                    Err(_) => if !buf.is_empty() {
                        stream.send(buf.into()).await?;
                        buf = String::new();
                    }
                }
            }
        }

        Ok(())
    })))
}
