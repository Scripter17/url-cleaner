use std::process::{Command, Stdio};
use std::fs;

use clap::Subcommand;

pub mod hyperfine;
pub mod massif;

#[derive(Debug, Subcommand)]
pub enum Args {
    Hyperfine(hyperfine::Args),
    Massif(massif::Args)
}

impl Args {
    pub fn r#do(self) -> fs::File {
        Command::new("cargo")
            .args(["build", "-r", "--bin", "url-cleaner"])
            .args(crate::CARGO_CONFIG)
            .stdout(std::io::stderr())
            .stderr(std::io::stderr())
            .spawn().unwrap().wait().unwrap();

        match self {
            Args::Hyperfine(args) => args.r#do(),
            Args::Massif(args) => args.r#do()
        }
    }
}
