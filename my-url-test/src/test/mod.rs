//! Tests.

use crate::prelude::*;

pub mod parser;
pub mod setters;

/// Tests.
#[derive(Debug, Parser)]
#[allow(missing_docs, reason = "Makes clap inherit the docs.")]
pub enum Args {
    Parser (parser ::Args),
    Setters(setters::Args),
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Self::Parser (args) => args.r#do(),
            Self::Setters(args) => args.r#do(),
        }
    }
}
