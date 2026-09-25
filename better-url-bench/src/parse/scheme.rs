//! Scheme.

use std::io::BufRead;

use clap::Parser;

use better_url::prelude::*;

/// Scheme.
///
/// The columns are:
///
/// 1. The line number.
///
/// 2. The time it took to parse a Scheme the specified number of times.
#[derive(Debug, Parser)]
pub struct Args {
    /// The amount to do.
    #[arg(long)]
    pub num: usize
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        for (i, value) in std::io::stdin().lock().lines().map(Result::unwrap).enumerate() {
            print!("{i}");

            let timer = std::time::Instant::now();

            for _ in 0..self.num {
                let _ = Scheme::new(&*value);
            }

            print!("\t{:.2?}", timer.elapsed());

            println!();
        }
    }
}
