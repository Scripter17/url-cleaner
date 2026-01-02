//! `/clean`.

use axum::{
    http::StatusCode,
    extract::State,
    body::Body
};
use tokio_util::io::StreamReader;
use tokio::io::AsyncBufReadExt;
use futures_util::StreamExt;
use async_stream::stream;

use url_cleaner_engine::prelude::*;
use url_cleaner_site_types::prelude::*;

/// `/clean`.
pub async fn clean(State(state): State<&'static crate::State>, job_config: JobConfig, body: Body) -> Result<Body, (StatusCode, &'static str)> {
    match (&state.passwords, job_config.password) {
        (None           , None          ) => {},
        (None           , Some(_       )) => Err((StatusCode::UNAUTHORIZED, "Requires no password"))?,
        (Some(_        ), None          ) => Err((StatusCode::UNAUTHORIZED, "Requires password"))?,
        (Some(passwords), Some(password)) => if !passwords.contains(&password) {Err((StatusCode::UNAUTHORIZED, "Invalid password"))?}
    }

    let mut cleaner = state.profiled_cleaner.get(job_config.profile.as_deref()).ok_or((StatusCode::BAD_REQUEST, "Unknown profile"))?;
    job_config.params_diff.apply(&mut cleaner.params);

    let (mut iss,     irs) = (0..state.worker_threads.get()).map(|_| tokio::sync::mpsc::channel(128)).collect::<(Vec<_>, Vec<_>)>();
    let (    oss, mut ors) = (0..state.worker_threads.get()).map(|_| tokio::sync::mpsc::channel(128)).collect::<(Vec<_>, Vec<_>)>();

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
            for (mut ir, os) in irs.into_iter().zip(oss) {
                s.spawn(move || {
                    while let Some(task) = ir.blocking_recv() {
                        os.blocking_send(match job.r#do(task) {
                            Ok (x) => {let mut ret: String = x.into(); ret.push('\n'); ret},
                            Err(e) => format!("-{e:?}\n")
                        }).expect("The out receiver to still exist.");
                    }
                });
            }
        });
    });

    tokio::spawn(async move {
        let mut body = StreamReader::new(body.into_data_stream().map(|x| x.map_err(std::io::Error::other)));

        let mut isi = (0..iss.len()).cycle();

        let mut buf = Vec::new();
        while body.read_until(b'\n', &mut buf).await.expect("To be able to read a line from the body.") > 0 {
            if buf.ends_with(b"\n") {
                buf.pop();
                if buf.ends_with(b"\r") {
                    buf.pop();
                }
            }
            if buf.is_empty() {
                continue;
            }
            iss.get_mut(isi.next().expect("???")).expect("???").send(buf).await.expect("The in receiver to still exist.");
            buf = Vec::new();
        }
    });

    Ok(Body::from_stream(stream!(
        let mut buf = String::new();

        'a: loop {
            for or in &mut ors {
                let mut recv = Box::pin(or.recv());
                match tokio::time::timeout(std::time::Duration::from_millis(10), &mut recv).await {
                    Ok(Some(x)) => {
                        buf.push_str(&x);

                        if buf.len() > 65535 {
                            yield buf;
                            buf = String::new();
                        }
                    },
                    Ok(None) => {
                        if !buf.is_empty() {
                            yield buf;
                        }

                        break 'a;
                    },
                    Err(_) => {
                        if !buf.is_empty() {
                            yield buf;
                            buf = String::new();
                        }

                        if let Some(x) = recv.await {
                            buf = x;
                        }
                    }
                }
            }
        }
    ).map(Ok::<_, std::convert::Infallible>)))
}
