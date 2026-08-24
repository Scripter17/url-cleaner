//! Benchmark URL parsing.

use std::io::BufRead;

use clap::Parser;

use better_url::prelude::*;

/// Benchmark URL parsing.
///
/// Compares against Servo's and Ada's URL crates, both with and without them calculating a SchemeDetails and HostDetails
///
/// If you care about Better URL's domain part APIs like BetterUrl::domain_prefix, columns 6-8 and 12-14 are probably more relevant to you than columns 3-5 and 9-11
///
/// If you don't, columns 3-5 and 9-11 are probably more relevant to you than columns 6-8 and 12-14
///
/// The columns are:
///
/// 1. The line number
///
/// 2. The time it takes BetterUrl::new to complete
///
/// If --servo
///
/// 3. The time it takes url::Url::parse to complete
///
/// 4. Column 3 divided by column 2
///
/// 5. The current sum of column 3 divided by the current sum of column 2
///
/// 6. The time it takes url::Url::parse, SchemeDetails::new_unchecked, and HostDetails::parse to complete
///
/// 7. Column 6 divided by column 2
///
/// 8. The current sum of column 6 divided by the current sum of column 2
///
/// If --ada
///
/// 9. The time it takes ada_url::Url::parse to complete
///
/// 10. Column 9 divided by column 2
///
/// 11. The current sum of column 9 divided by the current sum of column 2
///
/// 12. The time it takes ada_url::Url::parse, SchemeDetails::new_unchecked, and HostDetails::parse to complete
///
/// 13. Column 12 divided by column 2
///
/// 14. The current sum of column 12 divided by the current sum of column 2
#[derive(Debug, Parser)]
pub struct Args {
    /// Compare to Servo's URL crate.
    #[arg(long)]
    pub servo: bool,
    /// Compare to Ada's URL crate.
    #[arg(long)]
    pub ada: bool,
    /// The number of times to do each URL.
    #[arg(long)]
    pub num: usize,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let mut burl_total       = std::time::Duration::default();
        let mut servo_total      = std::time::Duration::default();
        let mut servo_plus_total = std::time::Duration::default();
        let mut ada_total        = std::time::Duration::default();
        let mut ada_plus_total   = std::time::Duration::default();

        for (i, line) in std::io::stdin().lock().lines().map(Result::unwrap).enumerate() {
            print!("{i}");

            let timer = std::time::Instant::now();

            for _ in 0..self.num {
                let _ = BetterUrl::new(&*line);
            }

            let burl = timer.elapsed();
            burl_total += burl;

            print!("\t{burl:.2?}");



            if self.servo {
                let timer = std::time::Instant::now();

                for _ in 0..self.num {
                    let _ = url::Url::parse(&line);
                }

                let servo = timer.elapsed();
                servo_total += servo;

                let timer = std::time::Instant::now();

                for _ in 0..self.num {
                    if let Ok(url) = url::Url::parse(&line) {
                        let scheme_type = SchemeDetails::new_unchecked(url.scheme()).r#type();

                        if let Some(host) = url.host_str() {
                            let _ = HostDetails::parse(host, scheme_type);
                        }
                    }
                }

                let servo_plus = timer.elapsed();
                servo_plus_total += servo_plus;

                let servo_factor            = servo           .as_secs_f64() / burl      .as_secs_f64();
                let servo_total_factor      = servo_total     .as_secs_f64() / burl_total.as_secs_f64();
                let servo_plus_factor       = servo_plus      .as_secs_f64() / burl      .as_secs_f64();
                let servo_plus_total_factor = servo_plus_total.as_secs_f64() / burl_total.as_secs_f64();

                print!("\t{servo:.2?}\t{servo_factor:.2}\t{servo_total_factor:.2}");
                print!("\t{servo_plus:.2?}\t{servo_plus_factor:.2}\t{servo_plus_total_factor:.2}");
            }



            if self.ada {
                let timer = std::time::Instant::now();

                for _ in 0..self.num {
                    let _ = ada_url::Url::parse(&line, None);
                }

                let ada = timer.elapsed();
                ada_total += ada;

                let timer = std::time::Instant::now();

                for _ in 0..self.num {
                    if let Ok(url) = ada_url::Url::parse(&line, None) {
                        let scheme_type = SchemeDetails::new_unchecked(url.protocol().strip_suffix(':').unwrap()).r#type();

                        let _ = HostDetails::parse(url.hostname(), scheme_type);
                    }
                }

                let ada_plus = timer.elapsed();
                ada_plus_total += ada_plus;

                let ada_factor            = ada           .as_secs_f64() / burl      .as_secs_f64();
                let ada_total_factor      = ada_total     .as_secs_f64() / burl_total.as_secs_f64();
                let ada_plus_factor       = ada_plus      .as_secs_f64() / burl      .as_secs_f64();
                let ada_plus_total_factor = ada_plus_total.as_secs_f64() / burl_total.as_secs_f64();

                print!("\t{ada:.2?}\t{ada_factor:.2}\t{ada_total_factor:.2}");
                print!("\t{ada_plus:.2?}\t{ada_plus_factor:.2}\t{ada_plus_total_factor:.2}");
            }



            println!();
        }
    }
}
