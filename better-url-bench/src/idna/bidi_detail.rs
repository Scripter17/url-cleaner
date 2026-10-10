//! [`BidiDetail::parse`].

use clap::Parser;

use better_url::prelude::BidiDetail;

/// BidiDetail::parse.
#[derive(Debug, Parser)]
pub struct Args {
    /// The number of times to do each input.
    #[arg(long)]
    pub num: usize,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let mut burl_total  = std::time::Duration::default();

        for (i, line) in (1..).zip(std::io::stdin().lines().map(Result::unwrap)) {
            print!("{i}");

            let timer = std::time::Instant::now();

            for _ in 0..self.num {
                let _ = std::hint::black_box(BidiDetail::parse(&line));
            }

            let burl = timer.elapsed();
            burl_total += burl;

            print!("\t{burl:.2?}");



            println!();
        }
    }
}

