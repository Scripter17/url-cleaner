//! Benchmark URL parsing.

use crate::prelude::*;

/// Benchmark URL parsing.
#[derive(Debug, Parser)]
pub struct Args {
    /// The URLs.
    pub urls: Vec<String>,
    /// The num.
    #[arg(long)]
    pub num: u32,
    /// Test [`MyUrl`].
    #[arg(long)]
    pub new: bool,
    /// Test [`BetterUrl`].
    #[arg(long)]
    pub old: bool,
    /// Test [`url::Url`].
    #[arg(long)]
    pub raw: bool,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        for url in self.urls.into_iter().chain(std::io::stdin().lines().map(|x| x.expect("???"))) {
            println!("{url}");

            if self.new {
                let start = std::time::Instant::now();

                for _ in 0..self.num {
                    MyUrl::new(&url).expect("To parse the URL.");
                }

                println!("  New: {:?}", start.elapsed());
            }

            if self.old {
                let start = std::time::Instant::now();

                for _ in 0..self.num {
                    BetterUrl::parse(&url).expect("To parse the URL.");
                }

                println!("  Old: {:?}", start.elapsed());
            }

            if self.raw {
                let start = std::time::Instant::now();

                for _ in 0..self.num {
                    url::Url::parse(&url).expect("To parse the URL.");
                }

                println!("  Raw: {:?}", start.elapsed());
            }

            println!();
        }
    }
}
