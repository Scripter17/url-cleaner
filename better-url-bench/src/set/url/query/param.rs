//! Benchmark setting a query parameter.

use std::io::BufRead;

use clap::Parser;

use better_url::prelude::*;

/// Benchmark setting a URL's query parameter.
///
/// Parses each line of STDIN as a URL.
///
/// Please note that both creating the MaybeQueryValue and parsing the URLs are done entirely outside of timing.
///
/// - The first column is the line number.
///
/// If BetterUrl::new returns either an error, the rest of the columns are present but empty.
///
/// Otherwise:
///
/// - The second column is the time it takes BetterUrl::set_query_param to run the specified number of times.
///
/// The remaining columns are groups of 3 representing an alternate library being compared to.
///
/// - A group's first column is the time it took to set the specified query parameter the specified number of times.
///
/// - A group's second column is the group's time divided by Better URL's time.
///
/// - A group's third column is the current sum of the group's times divided by the current sum of Better URL's times.
///
/// If a particular library's URL parser returns an error, its 3 columns will be present but empty.
#[derive(Debug, Parser)]
pub struct Args {
    /** Compare to all libraries.     **/ #[arg(long)] pub all  : bool,
    /** Compare to Servo's URL crate. **/ #[arg(long)] pub servo: bool,
    /** Compare to Ada's URL crate.   **/ #[arg(long)] pub ada  : bool,

    /// The number of times to do each URL.
    #[arg(long)]
    pub num: usize,
    /// The name to set.
    #[arg(long)]
    pub name: String,
    /// The index to set.
    #[arg(long)]
    pub index: isize,
    /// The value to set it to.
    #[arg(long)]
    pub value: Option<Option<String>>,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let mut burl_total  = std::time::Duration::default();
        let mut servo_total = std::time::Duration::default();
        let mut ada_total   = std::time::Duration::default();

        let servo = self.servo || self.all;
        let ada   = self.ada   || self.all;

        let name  = &self.name;
        let index = self.index;
        let value = self.value.as_ref().map(Option::as_deref);

        for (i, line) in std::io::stdin().lock().lines().map(Result::unwrap).enumerate() {
            print!("{i}");

            match BetterUrl::new(&line) {
                Ok(url) => {
                    let r#type = url.query_type();

                    let urls = vec![url; self.num];

                    let value = value.map(|x| MaybeQueryValue::new(x, r#type));

                    let timer = std::time::Instant::now();

                    for mut url in urls {
                        let _ = url.set_query_param(name, index, value.as_ref());
                    }

                    let burl = timer.elapsed();
                    burl_total += burl;

                    print!("\t{burl:.2?}");



                    if servo {
                        match url::Url::parse(&line) {
                            Ok(url) => {
                                let urls = vec![url; self.num];

                                let timer = std::time::Instant::now();

                                for mut url in urls {
                                    let mut query = unsafe {MaybeQuery::new_unchecked(url.query(), r#type)};

                                    let _ = query.set(name, index, value.as_ref());

                                    url.set_query(query.into_owned().as_str());
                                }

                                let servo = timer.elapsed();
                                servo_total += servo;

                                let factor       = servo      .as_secs_f64() / burl      .as_secs_f64();
                                let total_factor = servo_total.as_secs_f64() / burl_total.as_secs_f64();

                                print!("\t{servo:.2?}\t{factor:.2}\t{total_factor:.2}");
                            },
                            Err(_) => print!("\t\t\t")
                        }
                    }

                    if ada {
                        match ada_url::Url::parse(&*line, None) {
                            Ok(url) => {
                                let urls = vec![url; self.num];

                                let timer = std::time::Instant::now();

                                for mut url in urls {
                                    let mut query = unsafe {MaybeQuery::new_unchecked(url.search().strip_prefix('?'), r#type)};

                                    let _ = query.set(name, index, value.as_ref());

                                    url.set_search(query.into_owned().as_str());
                                }

                                let ada = timer.elapsed();
                                ada_total += ada;

                                let factor       = ada      .as_secs_f64() / burl      .as_secs_f64();
                                let total_factor = ada_total.as_secs_f64() / burl_total.as_secs_f64();

                                print!("\t{ada:.2?}\t{factor:.2}\t{total_factor:.2}");
                            },
                            Err(_) => print!("\t\t\t")
                        }
                    }
                },
                Err(_) => {
                    print!("\t");
                    if servo {print!("\t\t\t");}
                    if ada   {print!("\t\t\t");}
                }
            }

            println!();
        }
    }
}
