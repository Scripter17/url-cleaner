//! Path.

use std::io::BufRead;

use clap::Parser;

use better_url::prelude::*;

use crate::util::BasicBenchThing;

/// Path.
///
/// The columns are:
///
///  1. The line number starting at 1.
///
///  2. FilePath::new.
///
///  3. SpecialNotFilePath::new.
///
///  4. NonSpecialPath::new.
///
///  5. OpaquePath::new.
///
///  6. The line number.
///
///  7. FilePathSegments::new.
///
///  8. SpecialNotFilePathSegments::new.
///
///  9. NonSpecialPathSegments::new.
///
/// 10. FilePathSegment::new.
///
/// 11. SpecialNotFilePathSegment::new.
///
/// 12. NonSpecialPathSegment::new.
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

            print!("\t{:.2?}", self.basic_bench_thing.bench(|| FilePath          ::new(&*value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| SpecialNotFilePath::new(&*value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| NonSpecialPath    ::new(&*value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| OpaquePath        ::new(&*value)));

            print!("\t{:.2?}", self.basic_bench_thing.bench(|| FilePathSegments          ::new(&*value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| SpecialNotFilePathSegments::new(&*value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| NonSpecialPathSegments    ::new(&*value)));

            print!("\t{:.2?}", self.basic_bench_thing.bench(|| FilePathSegment          ::new(&*value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| SpecialNotFilePathSegment::new(&*value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| NonSpecialPathSegment    ::new(&*value)));

            println!();
        }
    }
}
