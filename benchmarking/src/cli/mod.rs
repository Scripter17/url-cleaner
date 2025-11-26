use std::fs;
use std::process::Command;

use clap::Subcommand;

pub mod hyperfine;
pub mod massif;
pub mod callgrind;

#[derive(Debug, Subcommand)]
pub enum Args {
    Hyperfine(hyperfine::Args),
    Massif(massif::Args),
    Callgrind(callgrind::Args)
}

impl Args {
    pub fn r#do(self) -> fs::File {
        assert!(Command::new("cargo")
            .args(["+stable", "build", "-r", "--bin", "url-cleaner"])
            .args(crate::CARGO_CONFIG)
            .stdout(std::io::stderr())
            .stderr(std::io::stderr())
            .spawn().unwrap().wait().unwrap().success());

        match self {
            Args::Hyperfine(args) => args.r#do(),
            Args::Massif(args) => args.r#do(),
            Args::Callgrind(args) => args.r#do()
        }
    }
}
