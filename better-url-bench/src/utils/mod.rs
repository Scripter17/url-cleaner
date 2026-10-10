//! Utils.

use clap::Parser;

mod time_stat;
mod time_secs;

/// Utils.
#[expect(missing_docs, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
pub enum Args {
    TimeStat(time_stat::Args),
    TimeSecs(time_secs::Args),
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Self::TimeStat(args) => args.r#do(),
            Self::TimeSecs(args) => args.r#do(),
        }
    }
}
