//! Generate a copy of URL Cleaner Site Userscript for the specified instance.

use clap::Parser;

/// Generate a copy of URL Cleaner Site Userscript for the specified instance.
#[derive(Debug, Parser)]
pub struct Args {
    /// The host.
    #[arg(long, default_value = "localhost")]
    pub host: better_url::prelude::SpecialNotFileHost<'static>,
    /// The port.
    #[arg(long, default_value = "9149")]
    pub port: u16,
    /// If TLS is used.
    #[arg(long)]
    pub tls: bool,
}

impl Args {
    /// Do the command.
    pub async fn r#do(self) {
        let host = self.host;
        let port = self.port;

        let protocol = match self.tls {
            true  => "wss",
            false => "ws" ,
        };

        let (a, b) = crate::USERSCRIPT.split_once("localhost").expect("???");
        let (b, c) = b.split_once("ws://localhost:9149").expect("???");

        print!("{a}{host}{b}{protocol}://{host}:{port}{c}");
    }
}
