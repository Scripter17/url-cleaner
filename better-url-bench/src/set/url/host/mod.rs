//! Host.

use clap::Parser;

mod entire;

/// Host.
#[expect(clippy::missing_docs_in_private_items, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
pub enum Args {
    Entire(entire::Args),
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Self::Entire(args) => args.r#do(),
        }
    }
}
