//! WebSockets.

use url::Url;
use tokio_tungstenite::tungstenite;
use tungstenite::Message;
use futures_util::{StreamExt, SinkExt};
use tokio::io::{AsyncBufReadExt, BufReader};

/// Do a WebSocket connection.
pub async fn r#do(instance: Url) {
    let (mut sink, mut stream) = tokio_tungstenite::connect_async(instance).await.unwrap().0.split();

    let writer = tokio::spawn(async move {
        let mut stdin = BufReader::new(tokio::io::stdin());
        let mut buf  = Vec::new();
        let mut line = Vec::new();

        let mut task_lines = 0;

        loop {
            match tokio::time::timeout(std::time::Duration::from_millis(1), stdin.read_until(b'\n', &mut line)).await {
                Ok(x) => if x.unwrap() == 0 {
                    if !buf.is_empty() {
                        sink.send(Message::binary(buf)).await.unwrap();
                    }
                    break;
                } else {
                    if line.ends_with(b"\n") {
                        line.pop();
                        if line.ends_with(b"\r") {
                            line.pop();
                        }
                    }

                    if line.is_empty() {
                        continue;
                    }

                    task_lines += 1;

                    line.push(b'\n');
                    buf.extend_from_slice(&line);
                    line.clear();

                    if buf.len() >= 2usize.pow(18) {
                        sink.send(Message::binary(buf)).await.unwrap();
                        buf = Vec::new();
                    }
                },
                Err(_) => if !buf.is_empty() {
                    sink.send(Message::binary(buf)).await.unwrap();
                    buf = Vec::new();
                }
            }
        }

        sink.close().await.unwrap();
        sink.flush().await.unwrap();

        task_lines
    });

    let reader = tokio::spawn(async move {
        let mut buf = String::new();

        let mut result_lines = 0;

        loop {
            match tokio::time::timeout(std::time::Duration::from_millis(1), stream.next()).await {
                // TODO: Figure out why, rarely, WSS connections end in this instead of exiting normally.
                Ok(Some(Err(tungstenite::error::Error::Protocol(tungstenite::error::ProtocolError::ResetWithoutClosingHandshake)))) => break,
                Ok(Some(msg)) => if let Message::Text(x) = msg.expect("Getting a message to work") {
                    if !buf.is_empty() {buf.push('\n');}
                    buf.push_str(&x);
                    result_lines += x.lines().count();

                    if buf.len() > 65535 {
                        println!("{buf}");
                        buf.clear();
                    }
                },
                Ok(None) => {
                    if !buf.is_empty() {
                        println!("{buf}");
                    }
                    break;
                },
                Err(_) => if !buf.is_empty() {
                    println!("{buf}");
                    buf.clear();
                }
            }
        }

        result_lines
    });

    let (task_lines, result_lines) = tokio::try_join!(writer, reader).unwrap();

    assert_eq!(task_lines, result_lines, "Didn't get as many result lines as task task lines.");
}
