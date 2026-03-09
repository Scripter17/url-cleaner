//! Extract.

use super::prelude::*;

pub mod hosts;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::super::prelude::*;
}

/// Extract.
#[allow(missing_docs, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
pub enum Args {
    Hosts(hosts::Args),
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Self::Hosts(args) => args.r#do(),
        }
    }
}


