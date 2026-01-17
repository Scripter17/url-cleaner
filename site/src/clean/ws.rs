//! `/clean` WebSocket.

use std::sync::Arc;

use axum::{
    response::Response,
    extract::ws::{WebSocketUpgrade, Message},
    extract::State,
    http::StatusCode
};
use futures_util::StreamExt;

use url_cleaner_engine::prelude::*;

/// `/clean` WebSocket.
pub async fn clean_ws(State(state): State<&'static crate::State>, job: Arc<Job<'static>>, ws: WebSocketUpgrade) -> Result<Response, (StatusCode, &'static str)> {
    let (mut iss,     irs) = (0..state.worker_threads.get()).map(|_| tokio::sync::mpsc::unbounded_channel::<Vec<u8>>()).collect::<(Vec<_>, Vec<_>)>();
    let (    rs , mut rr ) = tokio::sync::mpsc::channel::<Vec<u8>>(512);
    let (    oss, mut ors) = (0..state.worker_threads.get()).map(|_| tokio::sync::mpsc::unbounded_channel::<String>()).collect::<(Vec<_>, Vec<_>)>();

    for (mut ir, os) in irs.into_iter().zip(oss) {
        let job = job.clone();
        let rs = rs.clone();
        std::thread::spawn(move || {
            while let Some(buf) = ir.blocking_recv() {
                let task = (&buf).make_task();
                let _ = rs.try_send(buf);
                os.send(match job.r#do(task) {
                    Ok (x) => x.into(),
                    Err(e) => format!("-{e:?}")
                }).expect("The out receiver to still exist.");
            }
        });
    }

    Ok(ws.on_upgrade(async move |mut socket| {
        let mut isi = (0..iss.len()).cycle();
        let mut ori = (0..ors.len()).cycle();
        let mut or = ors.get_mut(ori.next().expect("???")).expect("???");

        while let Some(message) = socket.next().await {
            let message = match message.expect("Receiving messages to always work.") {
                Message::Binary(bytes) => bytes,
                Message::Text(text) => text.into(),
                _ => continue
            };

            let mut sent = 0;

            for line in {message}.split(|x| *x == b'\n').map(|x| x.strip_suffix(b"\r").unwrap_or(x)).filter(|x| !x.is_empty()) {
                sent += 1;

                let mut buf = rr.try_recv().unwrap_or_default();

                buf.clear();
                buf.extend_from_slice(line);

                iss.get_mut(isi.next().expect("???")).expect("???").send(buf).expect("The in receiver to still be open.");
            }

            let mut buf = String::new();
            let mut received = 0;

            while received != sent {
                match tokio::time::timeout(std::time::Duration::from_millis(1), or.recv()).await {
                    Ok(Some(x)) => {
                        if !buf.is_empty() {buf.push('\n');}
                        buf.push_str(&x);

                        let _ = rs.try_send(x.into());

                        if buf.len() >= 2usize.pow(20) {
                            socket.send(buf.into()).await.expect("Sending messages to work.");
                            buf = String::new();
                        }

                        or = ors.get_mut(ori.next().expect("???")).expect("???");
                        received += 1;
                    },
                    Ok(None) => unreachable!(),
                    Err(_) => if !buf.is_empty() {
                        socket.send(buf.into()).await.expect("Sending messages to work.");
                        buf = String::new();
                    }
                }
            }

            if !buf.is_empty() {
                socket.send(buf.into()).await.expect("Sending messages to work.");
            }
        }
    }))
}
