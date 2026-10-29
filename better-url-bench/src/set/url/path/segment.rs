//! Benchmark setting a path.

use std::io::BufRead;

use clap::Parser;

use better_url::prelude::*;

use crate::util::BasicBenchThing;

/// Benchmark setting a URL's path segment.
///
/// Parses each line of STDIN as a URL.
///
/// Please note that both creating the PathSegment and parsing the URLs are done entirely outside of timing.
///
/// - The first column is the line number.
///
/// If BetterUrl::new returns either an error or BetterUrl::segmented_path_type returns None, the rest of the columns are present but empty.
///
/// Otherwise:
///
/// - The second column is the time it takes BetterUrl::set_path_segment to run the specified number of times.
///
/// The remaining columns are groups of 3 representing an alternate library being compared to.
///
/// - A group's first column is the time it took to set the specified path segment the specified number of times.
///
/// - A group's second column is the group's time divided by Better URL's time.
///
/// - A group's third column is the current sum of the group's times divided by the current sum of Better URL's times.
///
/// If a particular library's URL parser returns an error, its 3 columns will be present but empty.
#[derive(Debug, Parser)]
pub struct Args {
    /** Compare to all libraries.     */ #[arg(long)] pub all  : bool,
    /** Compare to Servo's URL crate. */ #[arg(long)] pub servo: bool,
    /** Compare to Ada's URL crate.   */ #[arg(long)] pub ada  : bool,

    /// The index to set.
    #[arg(long)]
    pub index: isize,
    /// The value to set it to.
    #[arg(long)]
    pub value: Option<String>,

    /** The [`BasicBenchThing`]. */ #[command(flatten)] pub basic_bench_thing: BasicBenchThing,
}

impl Args {
    /// Do the command.
    pub fn r#do(mut self) {
        let mut burl_total  = std::time::Duration::default();
        let mut servo_total = std::time::Duration::default();
        let mut ada_total   = std::time::Duration::default();

        if self.all {
            self.servo = true;
            self.ada   = true;
        }

        for (i, line) in (1..).zip(std::io::stdin().lock().lines().map(Result::unwrap)) {
            print!("{i}");

            match BetterUrl::new(&line) {
                Ok(url) if let Some(r#type) = url.segmented_path_type() => {
                    let value = self.value.as_deref().map(|x| PathSegment::new(x, r#type));

                    let burl = self.basic_bench_thing.bench_with_input(|mut url| url.set_path_segment(self.index, value.as_ref()), url);
                    burl_total += burl;

                    print!("\t{burl:.2?}");



                    if self.servo {
                        match url::Url::parse(&line) {
                            Ok(url) => {
                                let servo = self.basic_bench_thing.bench_with_input(|mut url| {
                                    let mut path = unsafe {SegmentedPath::new_unchecked(url.path(), r#type)};

                                    if let Ok(true) = path.set(self.index, value.as_ref()) {
                                        url.set_path(path.into_owned().as_str());
                                    }
                                }, url);
                                servo_total += servo;

                                let factor       = servo      .as_secs_f64() / burl      .as_secs_f64();
                                let total_factor = servo_total.as_secs_f64() / burl_total.as_secs_f64();

                                print!("\t{servo:.2?}\t{factor:.2}\t{total_factor:.2}");
                            },
                            Err(_) => print!("\t\t\t"),
                        }
                    }



                    if self.ada {
                        match ada_url::Url::parse(&line, None) {
                            Ok(url) => {
                                let ada = self.basic_bench_thing.bench_with_input(|mut url| {
                                    let mut path = unsafe {SegmentedPath::new_unchecked(url.pathname(), r#type)};

                                    if let Ok(true) = path.set(self.index, value.as_ref()) {
                                        let _ = url.set_pathname(Some(path.into_owned().as_str()));
                                    }
                                }, url);
                                ada_total += ada;

                                let factor       = ada      .as_secs_f64() / burl      .as_secs_f64();
                                let total_factor = ada_total.as_secs_f64() / burl_total.as_secs_f64();

                                print!("\t{ada:.2?}\t{factor:.2}\t{total_factor:.2}");
                            },
                            Err(_) => print!("\t\t\t"),
                        }
                    }
                },
                _ => {
                    print!("\t\t");
                    if self.servo {print!("\t\t\t");}
                    if self.ada   {print!("\t\t\t");}
                },
            }

            println!();
        }
    }
}
