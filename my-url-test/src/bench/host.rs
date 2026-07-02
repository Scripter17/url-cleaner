//! Benchmark host parsing.

use crate::prelude::*;

/// Benchmark host parsing.
#[derive(Debug, Parser)]
pub struct Args {
    /// The hosts.
    pub hosts: Vec<String>,
    /// The num.
    #[arg(long, default_value_t = 10_000)]
    pub num: usize,
    /// Test [`Host`].
    #[arg(long)]
    pub new: bool,
    /// Yest [`url::Host`].
    #[arg(long)]
    pub raw: bool,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        for host in self.hosts {
            println!("{host}");

            if self.new {
                let start = Instant::now();

                for _ in 0..self.num {
                    Host::new(&*host).expect("To parse the host.");
                }

                println!("  New: {:?}", start.elapsed());
            }

            if self.raw {
                let start = Instant::now();

                for _ in 0..self.num {
                    url::Host::parse(&host).expect("To parse the host.");
                }

                println!("  Raw: {:?}", start.elapsed());
            }

            println!();
        }
    }
}
