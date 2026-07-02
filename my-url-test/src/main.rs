//! Testing for a URL crate entirely decoupled from the main one.

mod my_url;

/// Prelude module.
mod prelude {
    pub use std::num::NonZero;
    pub use std::time::Instant;
    pub use std::borrow::Cow;
    pub use std::path::PathBuf;
    pub use std::ops::Range;

    pub use better_url::prelude::*;
    pub use clap::Parser;
    pub use thiserror::Error;
    pub use super::my_url::*;
}

use prelude::*;

pub mod test;
pub mod bench;

/// Testing for a URL crate entirely decoupled from the main one.
#[allow(missing_docs, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
pub enum Args {
    Test(test::Args),
    #[command(subcommand)]
    Bench(bench::Args),
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Self::Test (args) => args.r#do(),
            Self::Bench(args) => args.r#do(),
        }
    }
}

fn main() {
    Args::parse().r#do();
}
