//! Get.

use super::prelude::*;

use hyper_util::{client::legacy::{Client, connect::HttpConnector}, rt::TokioExecutor};
use hyper_tls::HttpsConnector;
use http_body_util::BodyExt;
use url::Url;
use tokio::io::AsyncWriteExt;
use futures_util::StreamExt;

/// Get information from URL Cleaner Site.
#[derive(Debug, Parser)]
#[allow(missing_docs, reason = "Makes clap inherit the docs.")]
pub struct Args {
    /// The instance (HTTP or HTTPS)
    #[arg(default_value = "http://127.0.0.1:9149")]
    pub instance: String,
    #[arg(long, default_value = "index")]
    pub thing: Thing
}

/// What to get.
#[derive(Debug, Default, Clone, Copy, ValueEnum)]
pub enum Thing {
    /// The Index.
    #[default]
    Index,
    /// The Info.
    Info,
    /// The Cleaner.
    Cleaner,
    /// The ProfilesConfig.
    Profiles
}

impl Args {
    /// Do the command.
    pub async fn r#do(self) {
        let mut instance = Url::parse(&self.instance).unwrap();

        instance.set_path(match self.thing {
            Thing::Index    => "/",
            Thing::Info     => "/get-info",
            Thing::Cleaner  => "/get-cleaner",
            Thing::Profiles => "/get-profiles"
        });

        let res = match instance.scheme() {
            "http"  => Client::builder(TokioExecutor::new()).build::<_, String>(HttpConnector ::new()).get(instance.as_str().parse().unwrap()),
            "https" => Client::builder(TokioExecutor::new()).build::<_, String>(HttpsConnector::new()).get(instance.as_str().parse().unwrap()),
            scheme => panic!("Invalid scheme {scheme}")
        }.await.unwrap();

        assert_eq!(res.status(), 200);

        let mut stdout = tokio::io::stdout();
        let mut data = res.into_body().into_data_stream();

        let mut append_newline = false;

        while let Some(buf) = data.next().await.map(Result::unwrap) {
            append_newline = !buf.ends_with(b"\n");
            stdout.write_all(&buf).await.unwrap();
            stdout.flush().await.unwrap();
        }

        if append_newline {
            stdout.write_all(b"\n").await.unwrap();
            stdout.flush().await.unwrap();
        }
    }
}
