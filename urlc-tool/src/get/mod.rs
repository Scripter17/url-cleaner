//! Get.

use super::prelude::*;

pub mod reddit;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::super::prelude::*;

    pub use super::Mode;
}

/// The mode.
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum Mode {
    /// Download all from reddit.
    #[default]
    Normal,
    /// Download missing from reddit.
    Continue,
    /// Download nothing from reddit and ignore missing.
    Regenerate
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
