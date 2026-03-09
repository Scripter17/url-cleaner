//! Get.

use super::prelude::*;

pub mod reddit;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::super::prelude::*;
}

/// Get tasks from various sources.
#[allow(missing_docs, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
pub enum Args {
    Reddit(reddit::Args)
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Args::Reddit(args) => args.r#do()
        }
    }
}
