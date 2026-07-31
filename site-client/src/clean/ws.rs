//! WebSockets.

use tokio_tungstenite::tungstenite;
use tungstenite::Message;
use futures_util::{StreamExt, SinkExt};
use tokio::io::AsyncReadExt;
use bytes::Bytes;

use better_url::prelude::*;

/// Do a WebSocket connection.
pub async fn r#do(instance: BetterUrl) {
    let (websocket, response) = tokio_tungstenite::connect_async(instance.as_str()).await.unwrap();

    assert_eq!(response.status(), 101);

    let (mut sink, mut stream) = websocket.split();

    let tasks = tokio::spawn(async move {
        let stdin = &mut tokio::io::stdin();
        let mut buf  = Vec::new();
        let mut tasks = 0;

        while tokio::time::timeout(std::time::Duration::from_millis(1), stdin.take(2u64.pow(18)).read_to_end(&mut buf)).await.map(Result::unwrap) != Ok(0) {
            if let Some(i) = memchr::memrchr(b'\n', &buf) {
                let temp = buf.split_off(i + 1);

                let mut rest = &*buf;

                while let Some(i) = memchr::memchr(b'\n', rest) {
                    if !matches!(unsafe {rest.get_unchecked(..i)}, b"" | b"\r") {
                        tasks += 1;
                    }
                    rest = unsafe {rest.get_unchecked(i+1..)};
                }

                sink.send(Bytes::from_owner(buf).into()).await.unwrap();

                buf = temp;
            }
        }

        if !buf.is_empty() {
            tasks += 1;
            sink.send(buf.into()).await.unwrap();
        }

        sink.close().await.unwrap();
        sink.flush().await.unwrap();

        tasks
    });

    let mut results = 0;

    while let Some(msg) = stream.next().await {
        match msg {
            Err(tungstenite::error::Error::Protocol(tungstenite::error::ProtocolError::ResetWithoutClosingHandshake)) => break,
            msg => if let Message::Text(x) = msg.unwrap() {
                println!("{x}");
                results += memchr::memchr_iter(b'\n', x.as_bytes()).count() + 1;
            }
        }
    }

    assert_eq!(tasks.await.unwrap(), results);
}
