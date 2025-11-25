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
    tests: PathBuf
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

    serde_json::from_str::<Tests>(&std::fs::read_to_string(args.tests)?)?.r#do(Cleaner::get_bundled()?)?;

    Ok(())
}
