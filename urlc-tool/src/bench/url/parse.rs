//! Benchmark URL parsing.

use crate::prelude::*;

/// Benchmark URL parsing.
///
/// Reads each line of STDIN as a URL.
///
/// Column 1 is the URL's index.
///
/// Column 2 how long it takes Better URL to parse it --num times.
///
/// Column 3N, if present, is how long it takes the Nth comparison crate to parse it --num times.
///
/// Column 3N+1, if present, is this row's column 2 divided by this row's column 3N.
///
/// Column 3n+2, if present, is the sum of column 2 divided by the sum of column 3N.
#[derive(Debug, Parser)]
pub struct Args {
    /// The number of times to do each URL.
    #[arg(long)]
    pub num: usize,
    /// Compare to Servo's URL crate.
    #[arg(long)]
    pub servo: bool,
    /// Compare to Ada's URL crate.
    #[arg(long)]
    pub ada: bool,
    /// Don't make Servo and Ada create SchemeDetails and HostDetails.
    ///
    /// By default they do because they're required to provide part of Better URL's API.
    ///
    /// However, making them do this fudges the numbers in my favor.
    #[arg(long)]
    pub no_adjust: bool,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let mut me_total    = std::time::Duration::default();
        let mut servo_total = std::time::Duration::default();
        let mut ada_total   = std::time::Duration::default();

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
                    if let Ok(url) = url::Url::parse(&line) && !self.no_adjust {
                        let scheme_type = SchemeDetails::new_unchecked(url.scheme()).r#type();

                        if let Some(host) = url.host_str() {
                            let _ = HostDetails::parse(host, scheme_type);
                        }
                    }
                }

                let servo = timer.elapsed();
                servo_total += servo;

                let servo_factor       = servo      .as_secs_f64() / me      .as_secs_f64();
                let servo_total_factor = servo_total.as_secs_f64() / me_total.as_secs_f64();

                print!("\t{servo:.2?}\t{servo_factor:.2}\t{servo_total_factor:.2}");
            }



            if self.ada {
                let timer = std::time::Instant::now();

                for _ in 0..self.num {
                    if let Ok(url) = ada_url::Url::parse(&line, None) && !self.no_adjust {
                        let scheme_type = SchemeDetails::new_unchecked(url.protocol().strip_suffix(':').unwrap()).r#type();

                        let _ = HostDetails::parse(url.host(), scheme_type);
                    }
                }

                let ada = timer.elapsed();
                ada_total += ada;

                let ada_factor       = ada      .as_secs_f64() / me      .as_secs_f64();
                let ada_total_factor = ada_total.as_secs_f64() / me_total.as_secs_f64();

                print!("\t{ada:.2?}\t{ada_factor:.2}\t{ada_total_factor:.2}");
            }

            println!();
        }
    }
}
