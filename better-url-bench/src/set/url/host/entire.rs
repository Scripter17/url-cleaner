//! Entire.

use std::io::BufRead;

use clap::Parser;

use better_url::prelude::*;

/// Benchmark setting a URL's host.
///
/// Parses each line of STDIN as a URL and processes it --num times for each datapoint.
///
/// Outputs a TSV whose columns are:
///
/// 1. The first column is the line number.
///
/// If BetterUrl::new returns an error, the rest of the columns are present but empty.
///
/// Otherwise:
///
/// 2. The second column is the time it takes BetterUrl::set_host to run using the precomputed Host.
///
/// 3. The third column is the time it takes BetterUrl::set_host to run using the raw --value.
///
/// 4. Column 3 divided by column 2.
///
/// 5. The current sum of column 3 divided by the current sum of column 2.
///
/// The remaining columns are groups of 6 representing an alternate library being compared to.
///
/// For each group, the columns are:
///
/// 1. The time it took to set the host using the precomputed Host.
///
/// 2. The group's column 1 divided by column 2.
///
/// 3. The current sum of the group's column 1 divided by the current sum of column 2.
///
/// 4. The time it took to set the host using the raw --value.
///
/// 5. The group's column 4 divided by column 3.
///
/// 6. The current sum of the group's column 4 divided by the current sum of column 3.
///
/// If a particular library's URL parser returns an error, its columns will be present but empty.
#[derive(Debug, Parser)]
pub struct Args {
    /** Compare to all libraries.     **/ #[arg(long)] pub all  : bool,
    /** Compare to Servo's URL crate. **/ #[arg(long)] pub servo: bool,
    /** Compare to Ada's URL crate.   **/ #[arg(long)] pub ada  : bool,

    /// The number of times to do each URL.
    #[arg(long)]
    pub num: usize,
    /// The value to set it to.
    #[arg(long)]
    pub value: String,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let mut burl_total          = std::time::Duration::default();
        let mut burl_untyped_total  = std::time::Duration::default();
        let mut servo_total         = std::time::Duration::default();
        let mut servo_untyped_total = std::time::Duration::default();
        let mut ada_total           = std::time::Duration::default();
        let mut ada_untyped_total   = std::time::Duration::default();

        let servo = self.servo || self.all;
        let ada   = self.ada   || self.all;

        for (i, line) in std::io::stdin().lock().lines().map(Result::unwrap).enumerate() {
            print!("{i}");

            match BetterUrl::new(&line) {
                Ok(url) if let Ok(value) = Host::new(&self.value, url.scheme_type()) => {
                    let temp = url.clone();
                    let urls = vec![temp; self.num];

                    let timer = std::time::Instant::now();

                    for mut url in urls {
                        let _ = url.set_host(value.borrowed());
                    }

                    let burl = timer.elapsed();
                    burl_total += burl;

                    print!("\t{burl:.2?}");



                    let urls = vec![url; self.num];

                    let timer = std::time::Instant::now();

                    for mut url in urls {
                        let _ = url.set_host(&self.value);
                    }

                    let burl_untyped = timer.elapsed();
                    burl_untyped_total += burl_untyped;

                    let factor       = burl_untyped      .as_secs_f64() / burl      .as_secs_f64();
                    let total_factor = burl_untyped_total.as_secs_f64() / burl_total.as_secs_f64();

                    print!("\t{burl_untyped:.2?}\t{factor:.2}\t{total_factor:.2}");



                    if servo {
                        match url::Url::parse(&line) {
                            Ok(url) => {
                                let temp = url.clone();
                                let urls = vec![temp; self.num];

                                let timer = std::time::Instant::now();

                                for mut url in urls {
                                    let _ = url.set_host(Some(value.as_str()));
                                }

                                let servo = timer.elapsed();
                                servo_total += servo;

                                let factor       = servo      .as_secs_f64() / burl      .as_secs_f64();
                                let total_factor = servo_total.as_secs_f64() / burl_total.as_secs_f64();

                                print!("\t{servo:.2?}\t{factor:.2}\t{total_factor:.2}");



                                let urls = vec![url; self.num];

                                let timer = std::time::Instant::now();

                                for mut url in urls {
                                    let _ = url.set_host(Some(&self.value));
                                }

                                let servo_untyped = timer.elapsed();
                                servo_untyped_total += servo_untyped;

                                let factor       = servo_untyped      .as_secs_f64() / burl_untyped      .as_secs_f64();
                                let total_factor = servo_untyped_total.as_secs_f64() / burl_untyped_total.as_secs_f64();

                                print!("\t{servo:.2?}\t{factor:.2}\t{total_factor:.2}");
                            },
                            Err(_) => print!("\t\t\t\t\t\t")
                        }
                    }



                    if ada {
                        match ada_url::Url::parse(&line, None) {
                            Ok(url) => {
                                let temp = url.clone();
                                let urls = vec![temp; self.num];

                                let timer = std::time::Instant::now();

                                for mut url in urls {
                                    let _ = url.set_hostname(Some(value.as_str()));
                                }

                                let ada = timer.elapsed();
                                ada_total += ada;

                                let factor       = ada      .as_secs_f64() / burl      .as_secs_f64();
                                let total_factor = ada_total.as_secs_f64() / burl_total.as_secs_f64();

                                print!("\t{ada:.2?}\t{factor:.2}\t{total_factor:.2}");



                                let urls = vec![url; self.num];

                                let timer = std::time::Instant::now();

                                for mut url in urls {
                                    let _ = url.set_hostname(Some(&self.value));
                                }

                                let ada_untyped = timer.elapsed();
                                ada_untyped_total += ada_untyped;

                                let factor       = ada_untyped      .as_secs_f64() / burl_untyped      .as_secs_f64();
                                let total_factor = ada_untyped_total.as_secs_f64() / burl_untyped_total.as_secs_f64();

                                print!("\t{ada:.2?}\t{factor:.2}\t{total_factor:.2}");
                            },
                            Err(_) => print!("\t\t\t\t\t\t")
                        }
                    }
                },
                _ => {
                    print!("\t\t\t\t");
                    if servo {print!("\t\t\t\t\t\t");}
                    if ada   {print!("\t\t\t\t\t\t");}
                }
            }

            println!();
        }
    }
}
