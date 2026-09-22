//! URls.

use crate::prelude::*;

mod extract;

/// URLs.
#[expect(missing_docs, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
pub enum Args {
    Extract(extract::Args),
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Self::Extract(args) => args.r#do(),
        }
    }
}
