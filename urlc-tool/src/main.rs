//! A tool to for developing URL Cleaner.

#![allow(clippy::unwrap_used       , reason = "Internal tool. I can fix it when it breads.")]
#![allow(clippy::indexing_slicing  , reason = "Internal tool. I can fix it when it breads.")]
#![allow(clippy::missing_panics_doc, reason = "Internal tool. I can fix it when it breads.")]

pub mod fs;
pub mod command;
pub mod util;
pub mod compile;
pub mod get;
pub mod filter;
pub mod bench;
pub mod test;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use std::path::{Path, PathBuf};
    pub use std::fs::{File, OpenOptions};
    pub use std::io::{Write, BufReader, BufRead};
    pub use std::process::Command;
    pub use std::borrow::Cow;

    pub use regex::Regex;
    pub use num_format::{Locale, ToFormattedString};
    pub use clap::{Parser, ValueEnum};

    pub use super::fs::*;
    pub use super::command::*;
    pub use super::util::*;
}

use prelude::*;

/// Internal tool to develop URL Cleaner.
///
/// Very fragile.
#[allow(clippy::missing_docs_in_private_items, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
enum Args {
    Compile(compile::Args),
    #[command(subcommand)]
    Get(get::Args),
    Filter(filter::Args),
    #[command(subcommand)]
    Bench(bench::Args),
    Test(test::Args)
}

impl Args {
    /// Do the command.
    fn r#do(self) {
        match self {
            Args::Compile(args) => args.r#do(),
            Args::Get    (args) => args.r#do(),
            Args::Filter (args) => args.r#do(),
            Args::Bench  (args) => args.r#do(),
            Args::Test   (args) => args.r#do()
        }
    }
}

fn main() {
    Args::parse().r#do();
}
