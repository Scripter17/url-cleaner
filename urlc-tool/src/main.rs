//! A tool to for developing URL Cleaner.

#![allow(clippy::unwrap_used       , reason = "Internal tool. I can fix it when it breads.")]
#![allow(clippy::indexing_slicing  , reason = "Internal tool. I can fix it when it breads.")]
#![allow(clippy::missing_panics_doc, reason = "Internal tool. I can fix it when it breads.")]

pub mod build;
pub mod cleaner;
pub mod tasks;
pub mod bench;
pub mod foldent;

pub mod util;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use std::path::{Path, PathBuf};
    pub use std::fs::{File, OpenOptions};
    pub use std::io::{Write, BufReader, BufRead};
    pub use std::process::Command;
    pub use std::borrow::Cow;
    pub use std::sync::{OnceLock, LazyLock};
    pub use std::str::FromStr;

    pub use regex::Regex;
    pub use num_format::{Locale, ToFormattedString};
    pub use clap::{Parser, ValueEnum, builder::PossibleValue};
    pub use serde::Deserialize;
    pub use indexmap::IndexMap;

    pub use super::util::prelude::*;

    pub use url_cleaner_engine::prelude::*;
}

use prelude::*;

/// Internal tool to develop URL Cleaner.
#[allow(missing_docs, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
pub enum Args {
    Build(build::Args),
    #[command(subcommand)]
    Cleaner(cleaner::Args),
    #[command(subcommand)]
    Tasks(tasks::Args),
    #[command(subcommand)]
    Bench(bench::Args),
    Foldent(foldent::Args),
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Self::Build  (args) => args.r#do(),
            Self::Cleaner(args) => args.r#do(),
            Self::Tasks  (args) => args.r#do(),
            Self::Bench  (args) => args.r#do(),
            Self::Foldent(args) => args.r#do(),
        }
    }
}

fn main() {
    Args::parse().r#do();
}
