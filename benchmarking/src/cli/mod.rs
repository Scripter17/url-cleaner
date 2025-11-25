use std::fs;

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
        match self {
            Args::Hyperfine(args) => args.r#do(),
            Args::Massif(args) => args.r#do(),
            Args::Callgrind(args) => args.r#do()
        }
    }
}
