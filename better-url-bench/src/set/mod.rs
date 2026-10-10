//! Set.

use clap::Parser;

mod url;

/// Set.
#[expect(missing_docs, reason = "Makes clap inherit the docs.")]
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
