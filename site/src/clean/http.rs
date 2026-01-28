//! `/clean` HTTP.

use std::sync::Arc;

use axum::{
    http::StatusCode,
    extract::State,
    response::{Response, IntoResponse},
    body::Body
};
use futures_util::{StreamExt, TryStreamExt, AsyncBufReadExt};
use async_stream::stream;

use url_cleaner_engine::prelude::*;

/// `/clean` HTTP.
pub async fn clean_http(State(state): State<&'static crate::State>, job: Job<'static>, body: Body) -> Result<Response, (StatusCode, &'static str)> {
    let (mut iss,     irs) = (0..state.worker_threads.get()).map(|_| tokio::sync::mpsc::channel::<Vec<u8>>(128)).collect::<(Vec<_>, Vec<_>)>();
    let (    rs , mut rr ) = tokio::sync::mpsc::channel::<Vec<u8>>(512);
    let (    oss, mut ors) = (0..state.worker_threads.get()).map(|_| tokio::sync::mpsc::channel::<String>(128)).collect::<(Vec<_>, Vec<_>)>();

    let job = Arc::new(job);

    tokio::spawn(async move {
        let mut body = body.into_data_stream().map_err(std::io::Error::other).into_async_read();
        let mut isi = (0..iss.len()).cycle();

        let mut buf = Vec::new();

        while matches!(body.read_until(b'\n', &mut buf).await, Ok(1..)) {
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

            buf = rr.try_recv().unwrap_or_default();

            buf.clear();
        }
    });

    for (mut ir, os) in irs.into_iter().zip(oss) {
        let job = job.clone();
        let rs = rs.clone();
        std::thread::spawn(move || {
            while let Some(buf) = ir.blocking_recv() {
                let task = (&buf).make_task();
                let _ = rs.try_send(buf);
                os.blocking_send(match job.r#do(task) {
                    Ok (x) => x.into(),
                    Err(e) => format!("-{e:?}")
                }).expect("The out receiver to still exist.");
            }
        });
    }

    Ok(Body::from_stream(stream!(
        let mut buf = String::new();
        let mut ori = (0..ors.len()).cycle();
        let mut or = ors.get_mut(ori.next().expect("???")).expect("???");

        loop {
            match tokio::time::timeout(std::time::Duration::from_millis(1), or.recv()).await {
                Ok(Some(x)) => {
                    buf.push_str(&x);
                    buf.push('\n');

                    let _ = rs.try_send(x.into());

                    if buf.len() >= 2usize.pow(20) {
                        yield buf;
                        buf = String::new();
                    }

                    or = ors.get_mut(ori.next().expect("???")).expect("???");
                },
                Ok(None) => {
                    if !buf.is_empty() {
                        yield buf;
                    }
                    break;
                },
                Err(_) => if !buf.is_empty() {
                    yield buf;
                    buf = String::new();
                }
            }
        }
    ).map(Ok::<_, std::convert::Infallible>)).into_response())
}
