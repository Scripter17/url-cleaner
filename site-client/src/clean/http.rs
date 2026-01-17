//! HTTP.

use std::io::Cursor;
use std::convert::Infallible;

use url::Url;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use hyper::{Request, body::Frame};
use http_body_util::{StreamBody, BodyExt};
use async_stream::stream;
use futures_util::StreamExt;

use hyper_util::{client::legacy::{Client, connect::HttpConnector}, rt::TokioExecutor};
use hyper_tls::HttpsConnector;

/// Do an HTTP connection.
pub async fn r#do(instance: Url) {
    let body = Box::pin(StreamBody::new(stream!(
        let mut stdin = tokio::io::stdin();

        loop {
            let mut buf = Vec::new();
            if matches!(tokio::time::timeout(std::time::Duration::from_millis(1), (&mut stdin).take(2u64.pow(20)).read_to_end(&mut buf)).await, Ok(Ok(0))) {
                break
            }
            yield Ok::<_, Infallible>(Frame::data(Cursor::new(buf)));
        }
    )));

    // Building an HttpsConnector is very expensive, and so should only be done when needed.

    let res = match instance.scheme() {
        "http"  => Client::builder(TokioExecutor::new()).build(HttpConnector ::new()).request(Request::builder().uri(instance.as_str()).method("POST").body(body).unwrap()),
        "https" => Client::builder(TokioExecutor::new()).build(HttpsConnector::new()).request(Request::builder().uri(instance.as_str()).method("POST").body(body).unwrap()),
        _ => unreachable!()
    }.await.unwrap();

    assert_eq!(res.status(), 200);

    let mut stdout = tokio::io::stdout();
    let mut data = res.into_body().into_data_stream();

    while let Some(buf) = data.next().await {
        stdout.write_all(&buf.unwrap()).await.unwrap();
        stdout.flush().await.unwrap();
    }
}
