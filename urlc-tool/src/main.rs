//! A tool to for developing URL Cleaner.

#![allow(clippy::unwrap_used       , reason = "Internal tool. I can fix it when it breads.")]
#![allow(clippy::indexing_slicing  , reason = "Internal tool. I can fix it when it breads.")]
#![allow(clippy::missing_panics_doc, reason = "Internal tool. I can fix it when it breads.")]

pub mod build;
pub mod doc;
pub mod normalize;
pub mod test;
pub mod bench;
pub mod foldent;
pub mod domains;
pub mod www;
pub mod get;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use std::path::{Path, PathBuf};
    pub use std::fs::{File, OpenOptions};
    pub use std::io::{Write, BufReader, BufRead};
    pub use std::process::{Command, Child};
    pub use std::borrow::Cow;
    pub use std::sync::{OnceLock, LazyLock};
    pub use std::str::FromStr;

    pub use regex::Regex;
    pub use num_format::{Locale, ToFormattedString};
    pub use clap::{Parser, ValueEnum, builder::PossibleValue};
    pub use thiserror::Error;

    pub use super::build::Bin;

    pub use super::TerminateChild;
    pub use super::new_file;
}

use prelude::*;

/// Make a new writable [`File`] and its directory if needed.
pub fn new_file<P: AsRef<Path>>(path: P) -> File {
    if let Some(dir) = path.as_ref().parent() {
        std::fs::create_dir_all(dir).unwrap();
    }

    std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(true).open(path).unwrap()
}

/// Terminate a [`Child`].
pub trait TerminateChild {
    /// Terminate a [`Child`].
    fn terminate(self);
}

impl TerminateChild for Child {
    fn terminate(mut self) {
        if self.try_wait().unwrap().is_none() {
            unsafe {
                libc::kill(self.id() as _, libc::SIGTERM);
            }
            assert_eq!(self.wait().unwrap().code(), None);
        }
    }
}

/// Internal tool to develop URL Cleaner.
///
/// Very fragile; Don't expect things to handle edge cases at all.
///
/// Always assumes you're running in the root of the URL Cleaner git repository.
#[allow(clippy::missing_docs_in_private_items, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
enum Args {
    Build(build::Args),
    Doc(doc::Args),
    Normalize(normalize::Args),
    Test(test::Args),
    #[command(subcommand)]
    Bench(bench::Args),
    Foldent(foldent::Args),
    Domains(domains::Args),
    Www(www::Args),
    #[command(subcommand)]
    Get(get::Args)
}

impl Args {
    /// Do the command.
    fn r#do(self) {
        match self {
            Self::Build    (args) => args.r#do(),
            Self::Doc      (args) => args.r#do(),
            Self::Normalize(args) => args.r#do(),
            Self::Test     (args) => args.r#do(),
            Self::Bench    (args) => args.r#do(),
            Self::Foldent  (args) => args.r#do(),
            Self::Domains  (args) => args.r#do(),
            Self::Www      (args) => args.r#do(),
            Self::Get      (args) => args.r#do()
        }
    }
}

fn main() {
    Args::parse().r#do();
}
