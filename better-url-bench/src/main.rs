//! Benchmarking tool for Better URL.
//!
//! Licensed under the Affero GNU Public License version 3.0 or later.
//!
//! <https://www.gnu.org/licenses/agpl-3.0.html>
//!
//! <https://github.com/Scripter17/url-cleaner>

#![allow(clippy::unwrap_used       , reason = "Shouldn't happen and if it does it's fine.")]
#![allow(clippy::missing_panics_doc, reason = "Shouldn't happen and if it does it's fine.")]

use clap::Parser;

pub mod parse;
pub mod set;
pub mod idna;
pub mod psl;
pub mod monitor;
pub mod util;

#[expect(rustdoc::bare_urls, reason = "It'd look bad in the console.")]
/// Benchmarking tool for Better URL.
///
/// Licensed under the Affero GNU Public License version 3.0 or later.
///
/// https://www.gnu.org/licenses/agpl-3.0.html
///
/// https://github.com/Scripter17/url-cleaner
#[expect(missing_docs, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
pub enum Args {
    #[command(subcommand)] Parse(parse::Args),
    #[command(subcommand)] Set  (set  ::Args),
    #[command(subcommand)] Idna (idna ::Args),
    Psl(psl::Args),
    Monitor(monitor::Args),
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Self::Parse  (args) => args.r#do(),
            Self::Set    (args) => args.r#do(),
            Self::Idna   (args) => args.r#do(),
            Self::Psl    (args) => args.r#do(),
            Self::Monitor(args) => args.r#do(),
        }
    }
}

fn main() {
    Args::parse().r#do();
}
