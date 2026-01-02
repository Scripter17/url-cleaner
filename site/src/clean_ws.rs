//! `/clean_ws`.

use axum::{
    response::Response,
    extract::ws::{WebSocketUpgrade, Message},
    extract::State,
    http::StatusCode
};
use futures_util::StreamExt;

use url_cleaner_engine::prelude::*;
use url_cleaner_site_types::prelude::*;

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

/// `/clean_ws`.
pub async fn clean_ws(State(state): State<&'static crate::State>, job_config: JobConfig, ws: WebSocketUpgrade) -> Result<Response, (StatusCode, &'static str)> {
    println!("2");
    match (&state.passwords, job_config.password) {
        (None           , None          ) => {},
        (None           , Some(_       )) => Err((StatusCode::UNAUTHORIZED, "Requires no password"))?,
        (Some(_        ), None          ) => Err((StatusCode::UNAUTHORIZED, "Requires password"))?,
        (Some(passwords), Some(password)) => if !passwords.contains(&password) {Err((StatusCode::UNAUTHORIZED, "Invalid password"))?}
    }

    let (ms , mut mr ) = tokio::sync::mpsc::channel::<Vec<u8>>(8);
    let (iss,     irs) = (0..state.worker_threads.get()).map(|_| std  ::sync::mpsc::channel::<TaskStreamThing  >(   )).collect::<(Vec<_>, Vec<_>)>();
    let (oss, mut ors) = (0..state.worker_threads.get()).map(|_| tokio::sync::mpsc::channel::<ResultStreamThing>(128)).collect::<(Vec<_>, Vec<_>)>();

    let mut cleaner = state.profiled_cleaner.get(job_config.profile.as_deref()).ok_or((StatusCode::BAD_REQUEST, "Unknown profile"))?;
    job_config.params_diff.apply(&mut cleaner.params);

    Ok(ws.on_upgrade(async move |mut socket| {
        std::thread::spawn(move || {
            let job = &Job {
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
                http_client: &Default::default()
            };

            std::thread::scope(move |s| {
                for (ir, os) in irs.into_iter().zip(oss) {
                    s.spawn(move || {
                        while let Ok(task) = ir.recv() {
                            os.blocking_send(match task {
                                TaskStreamThing::Task(task) => ResultStreamThing::Result(match job.r#do(task) {
                                    Ok (x) => format!("{x}"),
                                    Err(e) => format!("-{e:?}")
                                }),
                                TaskStreamThing::DoneMessage => ResultStreamThing::DoneMessage
                            }).expect("The out receiver to still exist.");
                        }
                    });
                }

                let mut isi = iss.iter().cycle();
                while let Some(message) = mr.blocking_recv() {
                    for (line, is) in message.split(|x| *x == b'\n').map(|x| x.strip_suffix(b"\r").unwrap_or(x)).filter(|x| !x.is_empty()).zip(&mut isi) {
                        is.send(TaskStreamThing::Task(line.to_owned())).expect("The out receiver to still exist.");
                    }
                    isi.next().expect("???").send(TaskStreamThing::DoneMessage).expect("The out receiver to still exist.");
                }
            });
        });

        let mut ori = (0..ors.len()).cycle();

        while let Some(message) = socket.next().await {
            let message = match message.expect("The message to be valid.") {
                Message::Binary(bytes) => bytes,
                Message::Text(text) => text.into(),
                _ => continue
            }.to_vec();

            ms.send(message).await.expect("The message receiver to still exist.");
            let mut next = Box::pin(ors.get_mut(ori.next().expect("???")).expect("???").recv());

            let mut buf = String::new();

            loop {
                match tokio::time::timeout(std::time::Duration::from_millis(10), &mut next).await {
                    Ok(Some(ResultStreamThing::Result(x))) => {
                        if !buf.is_empty() {buf.push('\n');}
                        buf.push_str(&x);
                        if buf.len() > 65536 {
                            socket.send(buf.into()).await.expect("The socket to still be valud");
                            buf = String::new();
                        }
                        drop(next);
                        next = Box::pin(ors.get_mut(ori.next().expect("???")).expect("???").recv());
                    },
                    Ok(None | Some(ResultStreamThing::DoneMessage)) => {
                        if !buf.is_empty() {
                            socket.send(buf.into()).await.expect("The socket to still be valid.");
                        }
                        break;
                    },
                    Err(_) => if !buf.is_empty() {
                        socket.send(buf.into()).await.expect("The socket to still be valid.");
                        buf = String::new();
                    }
                }
            }
        }
    }))
}
