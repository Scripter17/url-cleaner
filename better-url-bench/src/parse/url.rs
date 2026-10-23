//! Benchmark URL parsing.

use std::io::BufRead;
use std::time::Duration;

use clap::Parser;

use better_url::prelude::*;

use crate::util::BasicBenchThing;

/// Benchmark URL parsing.
///
/// For each line of input, the output is a TSV.
///
/// - The first column is the line number, starting at 1.
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
/// 2. Servo's URL crate plus Better URL's DomainHostDetails::parse_unchecked.
///
/// 3. Ada's URL crate (`ada_url`).
///
/// 4. Ada's URL crate plus Better URL's DomainHostDetails::parse_unchecked.
///
/// Because Better URL Bench enables Better URL's `psl` feature, comparisons 2 and 4 are going to be closer (but probably a bit above) the "actual" performance factor.
#[derive(Debug, Parser)]
pub struct Args {
    /** Compare to all parsers.                                               */ #[arg(long)] pub all       : bool,
    /** Compare to Servo's URL crate.                                         */ #[arg(long)] pub servo     : bool,
    /** Compare to Servo's URL crate plus DomainHostDetails::parse_unchecked. */ #[arg(long)] pub servo_plus: bool,
    /** Compare to Ada's URL crate.                                           */ #[arg(long)] pub ada       : bool,
    /** Compare to Ada's URL crate plus DomainHostDetails::parse_unchecked.   */ #[arg(long)] pub ada_plus  : bool,

    /** The [`BasicBenchThing`]. **/ #[command(flatten)] pub basic_bench_thing: BasicBenchThing,
}

impl Args {
    /// Do the command.
    pub fn r#do(mut self) {
        let mut burl_total       = Duration::default();
        let mut servo_total      = Duration::default();
        let mut servo_plus_total = Duration::default();
        let mut ada_total        = Duration::default();
        let mut ada_plus_total   = Duration::default();

        if self.all {
            self.servo      = true;
            self.servo_plus = true;
            self.ada        = true;
            self.ada_plus   = true;
        }

        for (i, line) in (1..).zip(std::io::stdin().lock().lines().map(Result::unwrap)) {
            print!("{i}");

            let burl = self.basic_bench_thing.bench(|| BetterUrl::new(&*line));
            burl_total += burl;
            print!("\t{burl:.2?}");



            if self.servo {
                let servo = self.basic_bench_thing.bench(|| url::Url::parse(&line));

                servo_total += servo;

                let servo_factor       = servo      .as_secs_f64() / burl      .as_secs_f64();
                let servo_total_factor = servo_total.as_secs_f64() / burl_total.as_secs_f64();

                print!("\t{servo:.2?}\t{servo_factor:.2}\t{servo_total_factor:.2}");
            }



            if self.servo_plus {
                let servo_plus = self.basic_bench_thing.bench(|| url::Url::parse(&line).map(|url| {
                    let domain_details = if url.is_special() && let Some(url::Host::Domain(domain)) = url.host() && !domain.is_empty() {
                        Some(DomainHostDetails::parse_unchecked(domain))
                    } else {
                        None
                    };

                    (url, domain_details)
                }));

                servo_plus_total += servo_plus;

                let servo_plus_factor       = servo_plus      .as_secs_f64() / burl      .as_secs_f64();
                let servo_plus_total_factor = servo_plus_total.as_secs_f64() / burl_total.as_secs_f64();

                print!("\t{servo_plus:.2?}\t{servo_plus_factor:.2}\t{servo_plus_total_factor:.2}");
            }



            if self.ada {
                let ada = self.basic_bench_thing.bench(|| ada_url::Url::parse(&line, None));

                ada_total += ada;

                let ada_factor       = ada      .as_secs_f64() / burl      .as_secs_f64();
                let ada_total_factor = ada_total.as_secs_f64() / burl_total.as_secs_f64();

                print!("\t{ada:.2?}\t{ada_factor:.2}\t{ada_total_factor:.2}");
            }



            if self.ada_plus {
                let ada_plus = self.basic_bench_thing.bench(|| ada_url::Url::parse(&line, None).map(|url| {
                    let domain_details = if url.scheme_type() != ada_url::SchemeType::NotSpecial && !url.hostname().is_empty() {
                        Some(DomainHostDetails::parse_unchecked(url.hostname()))
                    } else {
                        None
                    };

                    (url, domain_details)
                }));

                ada_plus_total += ada_plus;

                let ada_plus_factor       = ada_plus      .as_secs_f64() / burl      .as_secs_f64();
                let ada_plus_total_factor = ada_plus_total.as_secs_f64() / burl_total.as_secs_f64();

                print!("\t{ada_plus:.2?}\t{ada_plus_factor:.2}\t{ada_plus_total_factor:.2}");
            }



            println!();
        }
    }
}
