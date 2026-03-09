//! `/clean` HTTP.

use std::sync::Arc;

use axum::{
    extract::State,
    response::{Response, IntoResponse},
    body::Body
};
use futures_util::StreamExt;
use async_stream::stream;
use bytes::Bytes;

use url_cleaner_engine::prelude::*;

/// `/clean` HTTP.
pub async fn clean_http(State(state): State<&'static crate::State>, job: Job<'static>, body: Body) -> Response {
    let (iss,     irs) = (0..state.workers).map(|_| tokio::sync::mpsc::channel::<Bytes >(512)).collect::<(Vec<_>, Vec<_>)>();
    let (oss, mut ors) = (0..state.workers).map(|_| tokio::sync::mpsc::channel::<String>(512)).collect::<(Vec<_>, Vec<_>)>();

    let job = Arc::new(job);

    // https://github.com/tokio-rs/axum/discussions/3625#discussioncomment-15788097
    let mut body = body.into_data_stream();
    let mut body = futures_util::stream::iter(body.next().await).chain(body);

    tokio::spawn(async move {
        let mut isi = (0..iss.len()).cycle();
        let mut buf = Vec::new();

        while let Some(data) = body.next().await.map(Result::unwrap) {
            let mut x = data.split_inclusive(|b| *b == b'\n');

            if !buf.is_empty() && let Some(y) = x.next() {
                buf.extend_from_slice(y);
                if buf.ends_with(b"\n") {
                    buf.pop();
                    buf.pop_if(|b| *b == b'\r');
                    if !buf.is_empty() {
                        iss.get(isi.next().expect("???")).expect("???").send(buf.into()).await.expect("The in receiver to still exist");
                        buf = Vec::new();
                    }
                }
            }

            for y in x {
                match y.strip_suffix(b"\n").map(|z| z.strip_suffix(b"\r").unwrap_or(z)) {
                    Some(line) => if !line.is_empty() {
                        iss.get(isi.next().expect("???")).expect("???").send(data.slice_ref(line)).await.expect("The in receiver to still exist");
                    },
                    None => buf = y.into()
                }
            }
        }

        if buf.ends_with(b"\n") {
            buf.pop();
            buf.pop_if(|b| *b == b'\r');
        }

        if !buf.is_empty() {
            iss.get(isi.next().expect("???")).expect("???").send(buf.into()).await.expect("The in receiver to still exist");
        }
    });

    for (mut ir, os) in irs.into_iter().zip(oss) {
        let job = job.clone();
        std::thread::spawn(move || {
            while let Some(task) = ir.blocking_recv() {
                os.blocking_send(match job.r#do(&*task) {
                    Ok (x) => x.into(),
                    Err(e) => format!("-{e:?}")
                }).expect("The out receiver to still exist.");
            }
        });
    }

    Body::from_stream(stream!(
        let mut buf = String::new();
        let mut ori = (0..ors.len()).cycle();
        let mut or  = ors.get_mut(ori.next().expect("???")).expect("???");

        loop {
            match tokio::time::timeout(std::time::Duration::from_millis(1), or.recv()).await {
                Ok(Some(x)) => {
                    buf.push_str(&x);
                    buf.push('\n');

                    if buf.len() > 2usize.pow(18) {
                        yield buf;
                        buf = String::new();
                    }

                    or = ors.get_mut(ori.next().expect("???")).expect("???");
                }
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
    ).map(Ok::<_, std::convert::Infallible>)).into_response()
}
