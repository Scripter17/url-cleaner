//! URL.

use clap::Parser;

mod path;
mod query;

/// URL.
#[expect(clippy::missing_docs_in_private_items, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
pub enum Args {
    #[command(subcommand)]
    Path(path::Args),
    #[command(subcommand)]
    Query(query::Args),
    
}
impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Self::Path (args) => args.r#do(),
            Self::Query(args) => args.r#do(),
        }
    }
}

