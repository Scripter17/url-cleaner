//! Benchmark URL parsing.

use std::io::BufRead;

use clap::Parser;

use better_url::prelude::*;

/// Benchmark URL parsing.
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
/// 6. Column 3 plus the time it takes SchemeDetails::new_unchecked and HostDetails::parse to complete once per run
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
/// 12. Column 9 plus the time it takes SchemeDetails::new_unchecked and HostDetails::parse to complete once per run
///
/// 13. Column 12 divided by column 2
///
/// 14. The current sum of column 12 divided by the current sum of column 2
///
/// If you care about Better URL's domain part APIs (BetterUrl::domain_prefix, ::set_domain_prefix, etc.) then columns 6-8 and 12-14 are more more relevant.
///
/// If you don't then columns 3-5 and 9-11 are more relevant to you.
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
        let mut me_total             = std::time::Duration::default();
        let mut servo_raw_total      = std::time::Duration::default();
        let mut servo_detailed_total = std::time::Duration::default();
        let mut ada_raw_total        = std::time::Duration::default();
        let mut ada_detailed_total   = std::time::Duration::default();

        for (i, line) in std::io::stdin().lock().lines().map(Result::unwrap).enumerate() {
            print!("{i}");

            let timer = std::time::Instant::now();

            for _ in 0..self.num {
                let _ = BetterUrl::new(&*line);
            }

            let me = timer.elapsed();
            me_total += me;

            print!("\t{me:.2?}");



            if self.servo {
                let timer = std::time::Instant::now();

                for _ in 0..self.num {
                    let _ = url::Url::parse(&line);
                }

                let servo_raw = timer.elapsed();
                servo_raw_total += servo_raw;

                let url = url::Url::parse(&line);

                let timer = std::time::Instant::now();

                if let Ok(url) = url {
                    for _ in 0..self.num {
                        let scheme_type = SchemeDetails::new_unchecked(url.scheme()).r#type();

                        if let Some(host) = url.host_str() {
                            let _ = HostDetails::parse(host, scheme_type);
                        }
                    }
                }

                let servo_detailed = servo_raw + timer.elapsed();
                servo_detailed_total += servo_detailed;

                let servo_raw_factor            = servo_raw           .as_secs_f64() / me      .as_secs_f64();
                let servo_raw_total_factor      = servo_raw_total     .as_secs_f64() / me_total.as_secs_f64();
                let servo_detailed_factor       = servo_detailed      .as_secs_f64() / me      .as_secs_f64();
                let servo_detailed_total_factor = servo_detailed_total.as_secs_f64() / me_total.as_secs_f64();

                print!("\t{servo_raw:.2?}\t{servo_raw_factor:.2}\t{servo_raw_total_factor:.2}");
                print!("\t{servo_detailed:.2?}\t{servo_detailed_factor:.2}\t{servo_detailed_total_factor:.2}");
            }



            if self.ada {
                let timer = std::time::Instant::now();

                for _ in 0..self.num {
                    let _ = ada_url::Url::parse(&line, None);
                }

                let ada_raw = timer.elapsed();
                ada_raw_total += ada_raw;

                let url = ada_url::Url::parse(&line, None);

                let timer = std::time::Instant::now();

                if let Ok(url) = url {
                    for _ in 0..self.num {
                        let scheme_type = SchemeDetails::new_unchecked(url.protocol().strip_suffix(':').unwrap()).r#type();

                        let _ = HostDetails::parse(url.hostname(), scheme_type);
                    }
                }

                let ada_detailed = ada_raw + timer.elapsed();
                ada_detailed_total += ada_detailed;

                let ada_raw_factor            = ada_raw           .as_secs_f64() / me      .as_secs_f64();
                let ada_raw_total_factor      = ada_raw_total     .as_secs_f64() / me_total.as_secs_f64();
                let ada_detailed_factor       = ada_detailed      .as_secs_f64() / me      .as_secs_f64();
                let ada_detailed_total_factor = ada_detailed_total.as_secs_f64() / me_total.as_secs_f64();

                print!("\t{ada_raw:.2?}\t{ada_raw_factor:.2}\t{ada_raw_total_factor:.2}");
                print!("\t{ada_detailed:.2?}\t{ada_detailed_factor:.2}\t{ada_detailed_total_factor:.2}");
            }



            println!();
        }
    }
}
