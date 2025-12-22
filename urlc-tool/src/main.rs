//! A tool to for developing URL Cleaner.

#![allow(clippy::unwrap_used       , reason = "Internal tool. I can fix it when it breads.")]
#![allow(clippy::indexing_slicing  , reason = "Internal tool. I can fix it when it breads.")]
#![allow(clippy::missing_panics_doc, reason = "Internal tool. I can fix it when it breads.")]

pub mod compile;
pub mod doc;
pub mod test;
pub mod bench;
pub mod foldent;
pub mod domains;
pub mod www;
pub mod get;
pub mod filter;
pub mod fs;
pub mod command;
pub mod util;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use std::path::{Path, PathBuf};
    pub use std::fs::{File, OpenOptions};
    pub use std::io::{Write, BufReader, BufRead};
    pub use std::process::Command;
    pub use std::borrow::Cow;
    pub use std::sync::{OnceLock, LazyLock};

    pub use regex::Regex;
    pub use num_format::{Locale, ToFormattedString};
    pub use clap::{Parser, ValueEnum};

    pub use super::fs::*;
    pub use super::command::*;
    pub use super::util::*;

    pub use super::{BINDIR, DEBUG};
}

use prelude::*;

/// The directory of the binaries.
pub static BINDIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut ret = std::env::current_exe().unwrap();
    ret.pop();
    ret.pop();
    ret.push(if *DEBUG.get().unwrap() {"debug"} else {"release"});
    ret
});

/// Whether to use debug mode.
pub static DEBUG: OnceLock<bool> = OnceLock::new();

/// Internal tool to develop URL Cleaner.
///
/// Very fragile; Don't expect things to handle edge cases at all.
#[allow(clippy::missing_docs_in_private_items, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
struct Args {
    /// Use debug builds.
    #[arg(long)]
    debug: bool,
    #[command(subcommand)]
    subcommand: Subcommand
}

/// The command to do.
#[allow(clippy::missing_docs_in_private_items, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
enum Subcommand {
    Compile(compile::Args),
    Doc(doc::Args),
    Test(test::Args),
    #[command(subcommand)]
    Bench(bench::Args),
    Foldent(foldent::Args),
    Domains(domains::Args),
    Www(www::Args),
    #[command(subcommand)]
    Get(get::Args),
    Filter(filter::Args),
}

impl Args {
    /// Do the command.
    fn r#do(self) {
        DEBUG.set(self.debug).unwrap();

        match self.subcommand {
            Subcommand::Compile(args) => args.r#do(),
            Subcommand::Doc    (args) => args.r#do(),
            Subcommand::Test   (args) => args.r#do(),
            Subcommand::Bench  (args) => args.r#do(),
            Subcommand::Foldent(args) => args.r#do(),
            Subcommand::Domains(args) => args.r#do(),
            Subcommand::Www    (args) => args.r#do(),
            Subcommand::Get    (args) => args.r#do(),
            Subcommand::Filter (args) => args.r#do()
        }
    }
}

fn main() {
    Args::parse().r#do();
}
