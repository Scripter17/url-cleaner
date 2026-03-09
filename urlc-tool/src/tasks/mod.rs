//! Tasks.

use super::prelude::*;

pub mod www;
pub mod get;
pub mod normalize;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::super::prelude::*;
}

/// Tasks.
#[allow(missing_docs, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
pub enum Args {
    Www(www::Args),
    #[command(subcommand)]
    Get(get::Args),
    Normalize(normalize::Args),
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Args::Www      (args) => args.r#do(),
            Self::Get      (args) => args.r#do(),
            Self::Normalize(args) => args.r#do(),
        }
    }
}
