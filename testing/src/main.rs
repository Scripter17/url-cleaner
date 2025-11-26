//! A basic and not very good testing framework.

use std::path::PathBuf;
use std::io;

use serde::{Serialize, Deserialize};
use thiserror::Error;
use clap::Parser;

use better_url::BetterUrl;
use url_cleaner_engine::prelude::*;

mod tests;
use tests::*;
mod test_set;
use test_set::*;
mod test;
use test::*;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    cleaner: Option<PathBuf>,
    #[arg(long)]
    tests: PathBuf,
    #[arg(long)]
    assert_suitability: bool
}

fn is_default<T: Default + Eq>(x: &T) -> bool {
    x == &T::default()
}

fn get_true() -> bool {true}
fn is_true(x: &bool) -> bool {*x}

/// The enum of errors [`main`] can return.
#[derive(Debug, Error)]
pub enum TestingError {
    /// Returned when a [`GetCleanerError`] is encountered.
    #[error(transparent)] GetCleanerError(#[from] GetCleanerError),
    /// Returned when unable to load a [`Tests`] file.
    #[error(transparent)] CantLoadTests(#[from] io::Error),
    /// Returned when unable to parse a [`Tests`] file.
    #[error(transparent)] CantParseTests(#[from] serde_json::Error),
    /// Returned when a [`DoTestsError`] is encountered.
    #[error(transparent)] DoTestsError(#[from] DoTestsError)
}

fn main() -> Result<(), TestingError> {
    let args = Args::parse();

    let cleaner = Cleaner::load_or_get_bundled(args.cleaner)?;

    if args.assert_suitability {
        cleaner.assert_suitability();
    }

    serde_json::from_str::<Tests>(&std::fs::read_to_string(args.tests)?)?.r#do(&cleaner)?;

    Ok(())
}
