//! `/clean` WebSocket.

use std::sync::Arc;

use axum::{
    response::Response,
    extract::ws::{WebSocketUpgrade, Message},
    extract::State
};
use futures_util::StreamExt;
use bytes::Bytes;

use url_cleaner_engine::prelude::*;

/// `/clean` WebSocket.
pub async fn clean_ws(State(state): State<&'static crate::State>, job: Job<'static>, ws: WebSocketUpgrade) -> Response {
    let (iss,     irs) = (0..state.workers).map(|_| tokio::sync::mpsc::unbounded_channel::<Bytes >()).collect::<(Vec<_>, Vec<_>)>();
    let (oss, mut ors) = (0..state.workers).map(|_| tokio::sync::mpsc::unbounded_channel::<String>()).collect::<(Vec<_>, Vec<_>)>();

    let job = Arc::new(job);

    for (mut ir, os) in irs.into_iter().zip(oss) {
        let job = job.clone();
        std::thread::spawn(move || {
            while let Some(task) = ir.blocking_recv() {
                os.send(match job.r#do(&*task) {
                    Ok (x) => x.into(),
                    Err(e) => format!("-{e:?}")
                }).expect("The out receiver to still exist.");
            }
        });
    }

    ws.on_upgrade(async move |mut socket| {
        while let Some(message) = socket.next().await {
            let message = match message.expect("Receiving messages to always work.") {
                Message::Binary(bytes) => bytes,
                Message::Text(text) => text.into(),
                _ => continue
            };

            let mut tasks = 0;

            let lines = message.split_inclusive(|b| *b == b'\n')
                .map(|x| x.strip_suffix(b"\n").map(|y| y.strip_suffix(b"\r").unwrap_or(y)).unwrap_or(x))
                .filter(|line| !line.is_empty())
                .map(|line| message.slice_ref(line));

            for line in lines {
                iss.get(tasks % iss.len()).expect("???").send(line).expect("The in receiver to still be open.");
                tasks += 1;
            }

            let mut buf = String::new();

            for i in 0..tasks {
                let ori = i % ors.len();
                let or = ors.get_mut(ori).expect("???");

                match tokio::time::timeout(std::time::Duration::from_millis(1), or.recv()).await.map(Option::unwrap) {
                    Ok(x) => {
                        if !buf.is_empty() {buf.push('\n');}
                        buf.push_str(&x);
                    },
                    Err(_) => {
                        if !buf.is_empty() {
                            socket.send(buf.into()).await.expect("Sending messages to work.");
                        }
                        buf = or.recv().await.expect("???");
                    }
                }

                if buf.len() >= 2usize.pow(18) || i == tasks - 1 {
                    socket.send(buf.into()).await.expect("Sending messages to work.");
                    buf = String::new();
                }
            }
        }
    })
}
