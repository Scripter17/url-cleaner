use std::process::Command;
use std::fs;

use clap::Parser;

pub mod http;
pub mod websocket;

#[derive(Debug, Parser)]
pub enum Args {
    #[command(subcommand)]
    Http(http::Args),
    #[command(subcommand)]
    Websocket(websocket::Args)
}

impl Args {
    pub fn r#do(self) -> fs::File {
        assert!(Command::new("cargo")
            .args(["+stable", "build", "-r", "--bin", "url-cleaner-site"])
            .args(crate::CARGO_CONFIG)
            .stdout(std::io::stderr())
            .stderr(std::io::stderr())
            .spawn().unwrap().wait().unwrap().success());

        match self {
            Args::Http(args) => args.r#do(),
            Args::Websocket(args) => args.r#do()
        }
    }
}
