//! Benchmark URL parsing.

use std::io::BufRead;

use clap::Parser;

use better_url::prelude::*;

/// Benchmark URL parsing.
///
/// Gives each line of STDIN to each requested parser and outputs a TSV of timing info and whatnot.
///
/// - The first column is the line number.
///
/// - The second column is the time it took BetterUrl::new to run the specified number of times.
///
/// The remaining columns are groups of 3 representing an alternate parser being compared to.
///
/// - A group's first column is the time it took that group's parser to run the specified number of times.
///
/// - A group's second column is the group's time divided by Better URL's time.
///
/// - A group's third column is the current sum of the group's times divided by the current sum of Better URL's times.
///
/// There are currently 4 parsers to compare with:
///
/// 1. Servo's URL crate (`url`).
///
/// 2. Servo's URL crate plus Better URL's HostDetails::parse.
///
/// 3. Ada's URL crate (`ada_url`).
///
/// 4. Ada's URL crate plus Better URL's HostDetails::parse.
///
/// HostDetails::parse does some extra work, most notably a lookup into the Public Suffix List for domain hosts, that is required to implement some of Better URL's APIs.
///
/// If you don't need those APIs then comparing without the handicap is more relevant to you, but for Servo and Ada to achieve API pairity with Better URL they'd need to add it.
#[derive(Debug, Parser)]
pub struct Args {
    /// The number of times to do each URL.
    #[arg(long)]
    pub num: usize,
    /// Compare to all parsers.
    #[arg(long)]
    pub all: bool,

    /** Compare to Servo's URL crate.                         **/ #[arg(long)] pub servo     : bool,
    /** Compare to Servo's URL crate plus HostDetails::parse. **/ #[arg(long)] pub servo_plus: bool,
    /** Compare to Ada's URL crate.                           **/ #[arg(long)] pub ada       : bool,
    /** Compare to Ada's URL crate plus HostDetails::parse.   **/ #[arg(long)] pub ada_plus  : bool,
}

impl Args {
    /// Do the command.
    pub fn r#do(mut self) {
        let mut burl_total       = std::time::Duration::default();
        let mut servo_total      = std::time::Duration::default();
        let mut servo_plus_total = std::time::Duration::default();
        let mut ada_total        = std::time::Duration::default();
        let mut ada_plus_total   = std::time::Duration::default();

        if self.all {
            self.servo      = true;
            self.servo_plus = true;
            self.ada        = true;
            self.ada_plus   = true;
        }

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

                let servo_factor       = servo      .as_secs_f64() / burl      .as_secs_f64();
                let servo_total_factor = servo_total.as_secs_f64() / burl_total.as_secs_f64();

                print!("\t{servo:.2?}\t{servo_factor:.2}\t{servo_total_factor:.2}");
            }



            if self.servo_plus {
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

                let servo_plus_factor       = servo_plus      .as_secs_f64() / burl      .as_secs_f64();
                let servo_plus_total_factor = servo_plus_total.as_secs_f64() / burl_total.as_secs_f64();

                print!("\t{servo_plus:.2?}\t{servo_plus_factor:.2}\t{servo_plus_total_factor:.2}");
            }



            if self.ada {
                let timer = std::time::Instant::now();

                for _ in 0..self.num {
                    let _ = ada_url::Url::parse(&line, None);
                }

                let ada = timer.elapsed();
                ada_total += ada;

                let ada_factor       = ada      .as_secs_f64() / burl      .as_secs_f64();
                let ada_total_factor = ada_total.as_secs_f64() / burl_total.as_secs_f64();

                print!("\t{ada:.2?}\t{ada_factor:.2}\t{ada_total_factor:.2}");
            }



            if self.ada_plus {
                let timer = std::time::Instant::now();

                for _ in 0..self.num {
                    if let Ok(url) = ada_url::Url::parse(&line, None) {
                        let scheme_type = SchemeDetails::new_unchecked(url.protocol().strip_suffix(':').unwrap()).r#type();

                        let _ = HostDetails::parse(url.hostname(), scheme_type);
                    }
                }

                let ada_plus = timer.elapsed();
                ada_plus_total += ada_plus;

                let ada_plus_factor       = ada_plus      .as_secs_f64() / burl      .as_secs_f64();
                let ada_plus_total_factor = ada_plus_total.as_secs_f64() / burl_total.as_secs_f64();

                print!("\t{ada_plus:.2?}\t{ada_plus_factor:.2}\t{ada_plus_total_factor:.2}");
            }



            println!();
        }
    }
}
