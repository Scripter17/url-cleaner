//! Set.

use clap::Parser;

mod url;

/// Set.
#[expect(clippy::missing_docs_in_private_items, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
pub enum Args {
    #[command(subcommand)]
    Url(url::Args),
    
}
impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Self::Url(args) => args.r#do(),
        }
    }
}
