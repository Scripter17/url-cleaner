use std::process::{Command, Stdio};
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
        Command::new("cargo")
            .args(["build", "-r", "--bin", "url-cleaner-site"])
            .args(crate::CARGO_CONFIG)
            .stdout(std::io::stderr())
            .stderr(std::io::stderr())
            .spawn().unwrap().wait().unwrap();

        match self {
            Args::Http(args) => args.r#do(),
            Args::Websocket(args) => args.r#do()
        }
    }
}

