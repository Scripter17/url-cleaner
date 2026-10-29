//! Query.

use std::io::BufRead;

use clap::Parser;

use better_url::prelude::*;

use crate::util::BasicBenchThing;

/// Query.
///
/// The columns are:
///
///  1. The line number starting at 1.
///
///  2. SpecialQuery::new.
///
///  3. NonSpecialQuery::new.
///
///  4. SpecialQuerySegment::new.
///
///  5. NonSpecialQuerySegment::new.
///
///  6. SpecialQueryName::new.
///
///  7. NonSpecialQueryName::new.
///
///  8. SpecialQueryValue::new.
///
///  9. NonSpecialQueryValue::new.
///
/// 10. encode_query_part.
///
/// 11. decode_query_part.
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

            print!("\t{:.2?}", self.basic_bench_thing.bench(|| SpecialQuery          ::new(&*value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| NonSpecialQuery       ::new(&*value)));

            print!("\t{:.2?}", self.basic_bench_thing.bench(|| SpecialQuerySegment   ::new(&*value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| NonSpecialQuerySegment::new(&*value)));

            print!("\t{:.2?}", self.basic_bench_thing.bench(|| SpecialQueryName      ::new(&*value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| NonSpecialQueryName   ::new(&*value)));

            print!("\t{:.2?}", self.basic_bench_thing.bench(|| SpecialQueryValue     ::new(&*value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| NonSpecialQueryValue  ::new(&*value)));

            print!("\t{:.2?}", self.basic_bench_thing.bench(|| better_url::util::encode_query_part(&*value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| better_url::util::decode_query_part(&*value)));

            println!();
        }
    }
}
