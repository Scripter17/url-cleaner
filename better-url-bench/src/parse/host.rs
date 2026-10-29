//! Host.

use std::io::BufRead;

use clap::Parser;

use better_url::prelude::*;

use crate::util::BasicBenchThing;

/// Host.
///
/// The columns are:
///
///  1. The line number starting at 1.
///
///  2. FileHost::new.
///
///  3. SpecialNotFileHost::new.
///
///  4. NonSpecialHost::new.
///
///  5. DomainHost::new.
///
///  6. Ipv4Host::new.
///
///  7. Ipv6Host::new.
///
///  8. OpaqueHost::new.
///
///  9. EmptyHost::new.
///
/// 10. url::Host::parse.
///
/// 11. url::Host::parse_opaque.
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

            print!("\t{:.2?}", self.basic_bench_thing.bench(|| FileHost          ::new(&*value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| SpecialNotFileHost::new(&*value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| NonSpecialHost    ::new(&*value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| DomainHost        ::new(&*value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| Ipv4Host          ::new(&*value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| Ipv6Host          ::new(&*value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| OpaqueHost        ::new(&*value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| EmptyHost         ::new(&*value)));

            print!("\t{:.2?}", self.basic_bench_thing.bench(|| url::Host::parse       (& value)));
            print!("\t{:.2?}", self.basic_bench_thing.bench(|| url::Host::parse_opaque(& value)));

            println!();
        }
    }
}
