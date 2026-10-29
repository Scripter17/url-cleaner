//! [`BidiDetail::parse`].

use clap::Parser;

use better_url::prelude::BidiDetail;

use crate::util::BasicBenchThing;

/// BidiDetail::parse.
#[derive(Debug, Parser)]
pub struct Args {
    /// The [`BasicBenchThing`].
    #[command(flatten)]
    pub basic_bench_thing: BasicBenchThing,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        for (i, line) in (1..).zip(std::io::stdin().lines().map(Result::unwrap)) {
            print!("{i}");

            print!("\t{:.2?}", self.basic_bench_thing.bench(|| BidiDetail::parse(&line)));

            println!();
        }
    }
}

