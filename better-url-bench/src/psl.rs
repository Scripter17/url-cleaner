//! PSL.

use clap::Parser;
use psl::Psl;

/// PSL.
#[derive(Debug, Parser)]
pub struct Args {
    /// The numebr of times to do each input.
    #[arg(long)]
    pub num: usize
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let mut burl_total = std::time::Duration::default();
        let mut psl_total  = std::time::Duration::default();

        for (i, line) in (1..).zip(std::io::stdin().lines().map(Result::unwrap)) {
            print!("{i}");

            let timer = std::time::Instant::now();

            for _ in 0..self.num {
                let _ = std::hint::black_box(psl::List.find(better_url::util::SplitDots(Some(&line)).rev().map(str::as_bytes)));
            }

            let burl = timer.elapsed();
            burl_total += burl;

            print!("\t{burl:.2?}");



            let timer = std::time::Instant::now();

            for _ in 0..self.num {
                let _ = std::hint::black_box(psl::suffix(line.as_ref()));
            }

            let psl = timer.elapsed();
            psl_total += psl;

            let factor       = psl      .as_secs_f64() / burl      .as_secs_f64();
            let factor_total = psl_total.as_secs_f64() / burl_total.as_secs_f64();

            print!("\t{psl:.2?}\t{factor:.2?}\t{factor_total:.2?}");



            println!();
        }
    }
}
