//! Benchmarking tool for Better URL.

#![allow(clippy::unwrap_used       , reason = "Shouldn't happen and if it does it's fine.")]
#![allow(clippy::missing_panics_doc, reason = "Shouldn't happen and if it does it's fine.")]

use clap::Parser;

pub mod parse;
pub mod set;
pub mod idna;
pub mod psl;
pub mod utils;
pub mod util;

/// Benchmarking tool for Better URL.
#[expect(missing_docs, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
pub enum Args {
    #[command(subcommand)] Parse(parse::Args),
    #[command(subcommand)] Set  (set  ::Args),
    #[command(subcommand)] Idna (idna ::Args),
    Psl(psl::Args),
    #[command(subcommand)] Utils(utils::Args),
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Self::Parse(args) => args.r#do(),
            Self::Set  (args) => args.r#do(),
            Self::Idna (args) => args.r#do(),
            Self::Psl  (args) => args.r#do(),
            Self::Utils(args) => args.r#do(),
        }
    }
}

fn main() {
    Args::parse().r#do();
}
