//! A URL Cleaner Site WebSocket client.

#![allow(clippy::unwrap_used       , reason = "It's fiiiiine.")]
#![allow(clippy::missing_panics_doc, reason = "It's fiiiiine.")]

use std::path::PathBuf;

use url::Url;
use clap::Parser;
use tokio_tungstenite::tungstenite;
use tungstenite::Message;
use futures_util::{StreamExt, SinkExt};
use tokio::io::{AsyncRead, AsyncBufReadExt, BufReader};

/// A URL Cleaner Site WebSocket client.
#[derive(Debug, Parser)]
pub struct Args {
    /// The instance.
    pub instance: String,
    /// The JobConfig.
    #[arg(long)]
    pub config: Option<String>,
    /// The input.
    #[arg(long)]
    pub input: Option<PathBuf>,
    /// The size after which to send the buffer.
    #[arg(long, default_value_t = 65536)]
    pub buffer: usize,
    /// Disable output.
    #[arg(long)]
    pub silent: bool
}

impl Args {
    /// Do the command.
    pub async fn r#do(self) {
        let mut instance = Url::parse(&self.instance).unwrap();

        if let Some(config) = self.config {
            instance.query_pairs_mut().append_pair("config", &config);
        }

        let (mut sink, mut stream) = tokio_tungstenite::connect_async(instance).await.unwrap().0.split();

        let writer = tokio::spawn(async move {
            let mut input: BufReader<Box<dyn AsyncRead + Unpin + Send>> = BufReader::new(match self.input {
                Some(file) => Box::new(tokio::fs::File::open(file).await.unwrap()),
                None       => Box::new(tokio::io::stdin())
            });

            let mut buf = Vec::new();

            while input.read_until(b'\n', &mut buf).await.unwrap() > 0 {
                while buf.len() < self.buffer {
                    tokio::select! {
                        x = input.read_until(b'\n', &mut buf) => if x.unwrap() == 0 {break;},
                        _ = tokio::time::sleep(std::time::Duration::from_nanos(10)) => {break;}
                    };
                }

                sink.send(Message::binary(buf)).await.unwrap();

                buf = Vec::new();
            }

            sink.send(Message::Close(None)).await.unwrap();
        });

        let reader = tokio::spawn(async move {
            while let Some(message) = stream.next().await {
                if let Ok(mut x) = message.unwrap().to_text() && !self.silent && !x.is_empty() {
                    if let Some(y) = x.strip_suffix("\n") {
                        x = y;
                        if let Some(y) = x.strip_suffix("\r") {
                            x = y;
                        }
                    }
                    println!("{x}");
                }
            }
        });

        let (writer_result, reader_result) = tokio::join!(writer, reader);
        writer_result.unwrap();
        reader_result.unwrap();
    }
}

#[tokio::main]
async fn main() {
    Args::parse().r#do().await;
}
