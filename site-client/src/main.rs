//! URL Cleaner Site CLIent.

#![allow(clippy::unwrap_used       , reason = "It's fiiiiine.")]
#![allow(clippy::missing_panics_doc, reason = "It's fiiiiine.")]

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use clap::{Parser, ValueEnum};
}

use prelude::*;

pub mod get;
pub mod clean;

#[allow(rustdoc::bare_urls, reason = "It'd look bad in the console.")]
/// URL Cleaner Site CLIent.
/// Licensed under the Aferro GNU Public License version 3.0 or later.
/// https://github.com/Scripter17/url-cleaner
#[derive(Debug, Parser)]
#[allow(missing_docs, reason = "Makes clap inherit the docs.")]
pub enum Args {
    Get  (get  ::Args),
    Clean(clean::Args)
}

impl Args {
    /// Do the command.
    pub async fn r#do(self) {
        match self {
            Self::Get  (args) => args.r#do().await,
            Self::Clean(args) => args.r#do().await
        }
    }
}

#[tokio::main]
async fn main() {
    Args::parse().r#do().await;
}
