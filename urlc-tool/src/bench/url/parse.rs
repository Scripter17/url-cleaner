//! Benchmark URL parsing.

use crate::prelude::*;

/// Benchmark URL parsing.
#[derive(Debug, Parser)]
pub struct Args {
    /// The amount of parsings to do.
    #[arg(long)]
    pub num: usize,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        for line in std::io::stdin().lock().lines().map(Result::unwrap) {
            let a = std::time::Instant::now();

            for _ in 0..self.num {
                let _ = BetterUrl::new(&*line);
            }

            println!("{:?}", a.elapsed());
        }
    }
}
