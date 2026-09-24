//! Value.

use std::io::BufRead;

use clap::Parser;

use better_url::prelude::*;

/// Value.
///
/// The columns are:
///
/// 1. The line number.
///
/// 2. Time time it took to parse a QueryValue of type QueryType::Special the specified number of times.
///
/// 3. Time time it took to parse a QueryValue of type QueryType::NonSpecial the specified number of times.
#[derive(Debug, Parser)]
pub struct Args {
    /// The amount to do.
    #[arg(long)]
    pub num: u64,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        for (i, value) in std::io::stdin().lock().lines().map(Result::unwrap).enumerate() {
            print!("{i}");

            for r#type in [QueryType::Special, QueryType::NonSpecial] {
                let timer = std::time::Instant::now();

                for _ in 0..self.num {
                    let _ = QueryValue::new(&*value, r#type);
                }

                print!("\t{:.2?}", timer.elapsed());
            }

            println!();
        }
    }
}

