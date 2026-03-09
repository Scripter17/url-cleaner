//! Benchmarking.

use super::prelude::*;

pub mod suite;
pub mod cli;
pub mod site;
pub mod site_client;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::super::prelude::*;

    pub use super::{cli, site, site_client};

    pub use super::{STDIN, write_stdin};
}

/// The path used to store STDIN.
pub const STDIN: &str = "urlc-tool/tmp/bench/stdin.txt";

/// Write the job to `urlc-tool/tmp/stdin.txt`.
pub fn write_stdin(task: &str, num: u64) -> StdinHandle {
    let mut file = new_file(STDIN);

    for _ in 0..num {
        writeln!(file, "{task}").unwrap();
    }

    StdinHandle
}

/// Deletes [`STDIN`] on [`std::ops::Drop`].
#[derive(Debug)]
pub struct StdinHandle;

impl std::ops::Drop for StdinHandle {
    fn drop(&mut self) {
        std::fs::remove_file(STDIN).unwrap();
    }
}

/// Benchmarking.
#[derive(Debug, Parser)]
#[allow(missing_docs, reason = "Makes clap inherit the docs.")]
pub enum Args {
    Suite     (suite      ::Args),
    Cli       (cli        ::Args),
    Site      (site       ::Args),
    SiteClient(site_client::Args),
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Self::Suite     (args) => args.r#do(),
            Self::Cli       (args) => println!("{}", args.r#do()),
            Self::Site      (args) => println!("{}", args.r#do()),
            Self::SiteClient(args) => println!("{}", args.r#do()),
        }
    }
}
