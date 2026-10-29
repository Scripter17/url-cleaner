//! Fragment.

use std::io::BufRead;

use clap::Parser;

use better_url::prelude::*;

use crate::util::BasicBenchThing;

/// Fragment.
///
/// The columns are:
///
/// 1. The line number starting at 1.
///
/// 2. Fragment::new.
///
/// 3. FragmentQuery::new.
///
/// 4. FragmentQuerySegment::new.
///
/// 5. FragmentQueryName::new.
///
/// 6. FragmentQueryValue::new.
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

            print!("\t{:.2?}", self.basic_bench_thing.bench(|| Fragment            ::new(&*value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| FragmentQuery       ::new(&*value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| FragmentQuerySegment::new(&*value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| FragmentQueryName   ::new(&*value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| FragmentQueryValue  ::new(&*value)));

            println!();
        }
    }
}
