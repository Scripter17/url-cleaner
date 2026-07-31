//! Get.

use super::prelude::*;

use hyper_util::{client::legacy::{Client, connect::HttpConnector}, rt::TokioExecutor};
use hyper_tls::HttpsConnector;
use http_body_util::BodyExt;
use tokio::io::AsyncWriteExt;
use futures_util::StreamExt;

use better_url::prelude::*;

/// Get information from URL Cleaner Site.
#[derive(Debug, Parser)]
pub struct Args {
    /// The instance.
    #[arg(default_value = "http://127.0.0.1:9149")]
    pub instance: BetterUrl,
    /// The thing to get.
    pub thing: Thing,
}

/// What to get.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Thing {
    /** `/`           **/ Index     ,
    /** `/info`       **/ Info      ,
    /** `/cleaner`    **/ Cleaner   ,
    /** `/profiles`   **/ Profiles  ,
    /** `/userscript` **/ Userscript,
}

impl Args {
    /// Do the command.
    pub async fn r#do(self) {
        let mut instance = self.instance;

        instance.set_path(match self.thing {
            Thing::Index      => "/",
            Thing::Info       => "/info",
            Thing::Cleaner    => "/cleaner",
            Thing::Profiles   => "/profiles",
            Thing::Userscript => "/userscript",
        }).unwrap();

        let res = match instance.scheme_str() {
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
