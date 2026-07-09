//! Testing for a URL crate entirely decoupled from the main one.

#![allow(dead_code, reason = "Will be used when turned into a library.")]

mod my_url;
mod util;

/// Prelude module.
mod prelude {
    pub use std::num::NonZero;
    pub use std::borrow::Cow;
    pub use std::time::Instant;
    pub use std::path::PathBuf;
    pub use std::ops::Range;

    pub use better_url::prelude::*;
    pub use better_url::util::*;
    pub use clap::Parser;
    pub use thiserror::Error;
    pub use super::my_url::*;

    pub(crate) use super::util::*;
}

use prelude::*;

pub mod test;
pub mod benchmark;

/// Testing for a URL crate entirely decoupled from the main one.
#[allow(missing_docs, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
pub enum Args {
    #[command(subcommand)]
    Test(test::Args),
    #[command(subcommand)]
    Benchmark(benchmark::Args),
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Self::Test     (args) => args.r#do(),
            Self::Benchmark(args) => args.r#do(),
        }
    }
}

fn main() {
    Args::parse().r#do();
}
