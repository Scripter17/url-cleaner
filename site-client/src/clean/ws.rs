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

        while tokio::time::timeout(std::time::Duration::from_millis(1), stdin.take(2u64.pow(17)).read_to_end(&mut buf)).await.map(Result::unwrap) != Ok(0) {
            if let Some(i) = better_url::util::memrchr(&buf, b'\n') {
                let bytes = Bytes::copy_from_slice(unsafe {buf.get_unchecked(..i)});

                sink.send(bytes.into()).await.unwrap();

                tasks += better_url::util::MemchrLines {remainder: Some(unsafe {buf.get_unchecked(..i)})}.filter(|line| !line.is_empty()).count();

                buf.drain(..=i);
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
            // https://github.com/snapview/tokio-tungstenite/issues/373
            Err(tungstenite::error::Error::Protocol(tungstenite::error::ProtocolError::ResetWithoutClosingHandshake)) => break,
            msg => if let Message::Text(x) = msg.unwrap() {
                println!("{x}");
                results += better_url::util::MemchrLines {remainder: Some(x.as_ref())}.count();
            }
        }
    }

    assert_eq!(tasks.await.unwrap(), results);
}
