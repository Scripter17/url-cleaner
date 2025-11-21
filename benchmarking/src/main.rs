//! A benchmarking tool for URL Cleaner's frontends.

#![feature(unix_send_signal)]

#![allow(clippy::unwrap_used)]
#![allow(clippy::indexing_slicing)]

use std::os::unix::process::ChildExt;

use clap::Parser;

mod suite;
mod cli;
mod site;

const CARGO_CONFIG: &[&str] = &[
    "--config", "profile.release.strip=false",
    "--config", "profile.release.debug=2"
];

struct KillOnDrop(std::process::Child);

impl std::ops::Drop for KillOnDrop {
    fn drop(&mut self) {
        self.0.send_signal(2).unwrap();
        self.0.wait().unwrap();
    }
}

#[derive(Debug, Parser)]
enum Args {
    Suite(suite::Args),
    #[command(subcommand)]
    Cli(cli::Args),
    #[command(subcommand)]
    Site(site::Args)
}

impl Args {
    fn r#do(self) {
        match self {
            Args::Suite(args) => args.r#do(),
            Args::Cli(args) => {args.r#do();},
            Args::Site(args) => {args.r#do();}
        }
    }
}

fn main() {
    Args::parse().r#do();
}
