//! Benchmarking tool for Better URL.

#![allow(clippy::unwrap_used       , reason = "Shouldn't happen and if it does it's fine.")]
#![allow(clippy::missing_panics_doc, reason = "Shouldn't happen and if it does it's fine.")]

use clap::Parser;

mod parse;
mod set;

/// Benchmarking tool for Better URL.
#[expect(missing_docs, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
pub enum Args {
    #[command(subcommand)] Parse(parse::Args),
    #[command(subcommand)] Set  (set  ::Args),
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Self::Parse(args) => args.r#do(),
            Self::Set  (args) => args.r#do(),
        }
    }
}

fn main() {
    Args::parse().r#do();
}
