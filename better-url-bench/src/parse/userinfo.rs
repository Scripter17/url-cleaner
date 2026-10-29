//! Userinfo.

use std::io::BufRead;

use clap::Parser;

use better_url::prelude::*;

use crate::util::BasicBenchThing;

/// Userinfo.
///
/// The columns are:
///
/// 1. The line number starting at 1.
///
/// 2. Userinfo::new.
///
/// 3. Username::new.
///
/// 4. Password::new.
#[derive(Debug, Parser)]
pub struct Args {
    /// The [`BasicBenchThing`].
    #[command(flatten)]
    pub basic_bench_thing: BasicBenchThing,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        for (i, value) in (1..).zip(std::io::stdin().lock().lines().map(Result::unwrap)) {
            print!("{i}");

            print!("\t{:.2?}", self.basic_bench_thing.bench(|| Userinfo::new(&*value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| Username::new(&*value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| Password::new(&*value)));

            println!();
        }
    }
}
