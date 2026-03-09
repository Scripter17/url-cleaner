//! Cleaner.

use super::prelude::*;

pub mod test;
pub mod extract;
pub mod doc;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::super::prelude::*;
}

/// Cleaner.
#[allow(missing_docs, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
pub enum Args {
    Test(test::Args),
    #[command(subcommand)]
    Extract(extract::Args),
    Doc(doc::Args),
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Self::Test   (args) => args.r#do(),
            Self::Extract(args) => args.r#do(),
            Self::Doc    (args) => args.r#do(),
        }
    }
}

