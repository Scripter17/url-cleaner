//! Benchmark URL parsing.

use std::io::IsTerminal;

use crate::prelude::*;

/// Benchmark URL parsing.
#[derive(Debug, Parser)]
pub struct Args {
    /// The URLs.
    pub urls: Vec<String>,
    /// The num.
    #[arg(long)]
    pub num: u32,
    /// Compare to [`BetterUrl`].
    #[arg(long)]
    pub old: bool,
    /// Compare to [`url::Url`].
    #[arg(long)]
    pub raw: bool,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let a = (!std::io::stdin().is_terminal()).then(|| std::io::stdin().lines().map(|x| x.expect("???"))).into_iter().flatten();

        for url in self.urls.into_iter().chain(a) {
            println!("{url}");

            let new_start = std::time::Instant::now();
            for _ in 0..self.num {
                let _ = MyUrl::new(&url);
            }
            let new_elapsed = new_start.elapsed();
            println!("  New: {:?}", new_elapsed);

            if self.old {
                let old_start = std::time::Instant::now();
                for _ in 0..self.num {
                    let _ = BetterUrl::parse(&url);
                }
                let old_elapsed = old_start.elapsed();
                println!("  Old: {:?}: {}", old_elapsed, old_elapsed.as_secs_f64() / new_elapsed.as_secs_f64());
            }

            if self.raw {
                let raw_start = std::time::Instant::now();
                for _ in 0..self.num {
                    let _ = url::Url::parse(&url);
                }
                let raw_elapsed = raw_start.elapsed();
                println!("  Raw: {:?}: {}", raw_elapsed, raw_elapsed.as_secs_f64() / new_elapsed.as_secs_f64());
            }

            println!();
        }
    }
}
