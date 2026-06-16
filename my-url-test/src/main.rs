mod my_url;

mod prelude {
    pub use std::num::NonZero;
    pub use std::time::Instant;

    pub use better_url::prelude::*;
    pub use clap::Parser;
    pub use super::my_url::*;
}

use prelude::*;

mod test;
mod bench;
mod idna;

#[derive(Debug, Parser)]
pub enum Args {
    Test(test::Args),
    #[command(subcommand)]
    Bench(bench::Args),
    Idna(idna::Args),
}

impl Args {
    pub fn r#do(self) {
        match self {
            Self::Test (args) => args.r#do(),
            Self::Bench(args) => args.r#do(),
            Self::Idna (args) => args.r#do(),
        }
    }
}

fn main() {
    Args::parse().r#do();
}
