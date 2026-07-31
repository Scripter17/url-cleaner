//! Benchmark URL stuff.

use crate::prelude::*;

pub mod parse;
pub mod compare;

/// Benchmark URL stuff.
#[expect(missing_docs, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
pub enum Args {
    Parse  (parse  ::Args),
    Compare(compare::Args),
}

impl Args {
    /// Do the stuff.
    pub fn r#do(self) {
        match self {
            Self::Parse  (args) => args.r#do(),
            Self::Compare(args) => args.r#do(),
        }
    }
}
