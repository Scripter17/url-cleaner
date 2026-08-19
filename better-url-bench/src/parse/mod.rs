//! Parsing.

use clap::Parser;

mod url;
mod path;
mod query;

/// Parsing.
#[expect(clippy::missing_docs_in_private_items, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
pub enum Args {
    Url(url::Args),
    #[command(subcommand)]
    Path(path::Args),
    #[command(subcommand)]
    Query(query::Args),
    
}
impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Self::Url  (args) => args.r#do(),
            Self::Path (args) => args.r#do(),
            Self::Query(args) => args.r#do(),
        }
    }
}

