//! Segment.

use std::io::BufRead;

use clap::Parser;

use better_url::prelude::*;

/// Segment.
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
                    let _ = QuerySegment::new(&*value, r#type);
                }

                print!("\t{:.2?}", timer.elapsed());
            }

            println!();
        }
    }
}
