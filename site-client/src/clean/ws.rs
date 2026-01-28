//! WebSockets.

use url::Url;
use tokio_tungstenite::tungstenite;
use tungstenite::Message;
use futures_util::{StreamExt, SinkExt};
use tokio::io::AsyncReadExt;

/// Do a WebSocket connection.
pub async fn r#do(instance: Url) {
    let (websocket, response) = tokio_tungstenite::connect_async(instance).await.unwrap();

    assert_eq!(response.status(), 101);

    let (mut sink, mut stream) = websocket.split();

    let writer = tokio::spawn(async move {
        let stdin = &mut tokio::io::stdin();
        let mut buf  = Vec::new();

        while tokio::time::timeout(std::time::Duration::from_millis(1), stdin.take(2u64.pow(18)).read_to_end(&mut buf)).await.map(Result::unwrap) != Ok(0) {
            // `i` is the index of the last `\n` plus one.
            if let Some(i) = buf.iter().rev().position(|b| *b == b'\n').map(|i| buf.len() - i) {
                let temp = buf.split_off(i);
                buf.pop();
                buf.pop_if(|b| *b == b'\r');
                sink.send(buf.into()).await.unwrap();
                buf = temp;
            }
        }

        if !buf.is_empty() {
            sink.send(buf.into()).await.unwrap();
        }

        sink.close().await.unwrap();
        sink.flush().await.unwrap();
    });

    while let Some(msg) = stream.next().await {
        if let Message::Text(x) = msg.unwrap() {
            println!("{x}");
        }
    }

    writer.await.unwrap();
}
