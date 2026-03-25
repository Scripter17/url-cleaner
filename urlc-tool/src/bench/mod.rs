//! Benchmarking.

use super::prelude::*;

pub mod suite;
pub mod cli;
pub mod site;
pub mod site_client;

pub mod util;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::super::prelude::*;

    pub use super::{cli, site, site_client};

    pub use super::util::prelude::*;
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
