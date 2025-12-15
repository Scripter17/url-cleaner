//! Benchmark.

use super::prelude::*;

mod suite;
mod quick;
mod cli;
mod site;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::super::prelude::*;

    pub use super::make_stdin;
}

/// The path of the STDIN.
const STDIN: &str = "urlc-tool/tmp/bench/stdin.txt";

/// Make the STDIN file.
pub fn make_stdin(task: &str, num: usize) -> TmpFileHandle {
    let stdin = tmp_file(STDIN);

    for _ in 0..num {
        writeln!(stdin.file(), "{}", task).unwrap();
    }

    stdin
}

/// Benchmark.
#[allow(missing_docs, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
pub enum Args {
    Suite(suite::Args),
    Quick(quick::Args),
    #[command(subcommand)]
    Cli(cli::Args),
    #[command(subcommand)]
    Site(site::Args)
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Args::Suite(args) => args.r#do(),
            Args::Quick(args) => args.r#do(),
            Args::Cli  (args) => println!("{}", args.r#do()),
            Args::Site (args) => println!("{}", args.r#do())
        }
    }
}
