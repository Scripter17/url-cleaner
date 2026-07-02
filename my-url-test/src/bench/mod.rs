//! Benchmarking.

use crate::prelude::*;

mod whole;
mod host;

/// Benchmarking.
#[allow(missing_docs, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
pub enum Args {
    Whole(whole::Args),
    Host(host::Args),
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Self::Whole(args) => args.r#do(),
            Self::Host (args) => args.r#do(),
        }
    }
}
