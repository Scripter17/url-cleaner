//! Benchmark setting a query parameter.

use std::io::BufRead;

use clap::Parser;

use better_url::prelude::*;

/// Benchmark setting a query parameter.
///
/// The columns are:
///
/// 1. The line number
///
/// 2. The time it takes to BetterUrl::set_query_patam with a precomputed MaybeQueryValue
///
/// 3. The time it takes to BetterUrl::set_query_patam with --value as-is
///
/// 4. Column 3 divided by column 4
///
/// 5. The curent sum of column 3 divided by the current sum of column 4
///
/// If --servo
///
/// 6. The time it takes to Query::new_unchecked, Query::set, and servo::Url::set_query with --value as-is.
///
/// 7. Column 6 divided by column 2
///
/// 8. The current sum of column 6 divided by the current sum of column 2
///
/// 9. Column 6 divided by column 3
///
/// 10. The current sum of column 6 divided by the current sum of column 3
///
/// If --ada
///
/// 11. The time it takes to Query::new_unchecked, Query::set, and ada::Url::set_search with --value as-is.
///
/// 12. Column 11 divided by column 2
///
/// 13. The current sum of column 11 divided by the current sum of column 2
///
/// 14. Column 11 divided by column 3
///
/// 15. The current sum of column 11 divided by the current sum of column 3
///
/// If BetterUrl::new returns an error, columns 2 through 15 are present but empty
///
/// If --servo and servo::Url::parse returns an error, columns 6 through 10 are present but empty
///
/// If --ada and ada::Url::parse returns an error, columns 11 through 15 are present but empty
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
        let mut burl_t_total = std::time::Duration::default();
        let mut burl_s_total = std::time::Duration::default();
        let mut servo_total  = std::time::Duration::default();
        let mut ada_total    = std::time::Duration::default();

        let name  = &self.name;
        let index = self.index;
        let value = self.value.as_ref().map(Option::as_deref);

        for (i, line) in std::io::stdin().lock().lines().map(Result::unwrap).enumerate() {
            print!("{i}");

            match BetterUrl::new(&*line) {
                Ok(url) => {
                    let r#type = url.query_type();

                    let urls = (0..self.num).map(|_| url.clone()).collect::<Vec<_>>();

                    let typed = value.map(|x| MaybeQueryValue::new(x, r#type));

                    let timer = std::time::Instant::now();

                    for mut url in urls {
                        let _ = url.set_query_param(name, index, typed.as_ref());
                    }

                    let burl_t = timer.elapsed();
                    burl_t_total += burl_t;

                    print!("\t{burl_t:.2?}");



                    let urls = (0..self.num).map(|_| url.clone()).collect::<Vec<_>>();

                    let timer = std::time::Instant::now();

                    for mut url in urls {
                        let _ = url.set_query_param(name, index, value.as_ref());
                    }

                    let burl_s = timer.elapsed();
                    burl_s_total += burl_s;

                    let f  = burl_s      .as_secs_f64() / burl_t      .as_secs_f64();
                    let tf = burl_s_total.as_secs_f64() / burl_t_total.as_secs_f64();

                    print!("\t{burl_s:.2?}\t{f:.2}\t{tf:.2}");



                    if self.servo {
                        match url::Url::parse(&line) {
                            Ok(url) => {
                                let urls = (0..self.num).map(|_| url.clone()).collect::<Vec<_>>();

                                let timer = std::time::Instant::now();

                                for mut url in urls {
                                    let mut query = unsafe {MaybeQuery::new_unchecked(url.query(), r#type)};

                                    let _ = query.set(name, index, value);

                                    url.set_query(query.into_owned().as_str());
                                }

                                let servo = timer.elapsed();
                                servo_total += servo;

                                let tf  = servo      .as_secs_f64() / burl_t      .as_secs_f64();
                                let ttf = servo_total.as_secs_f64() / burl_t_total.as_secs_f64();
                                let sf  = servo      .as_secs_f64() / burl_s      .as_secs_f64();
                                let stf = servo_total.as_secs_f64() / burl_s_total.as_secs_f64();

                                print!("\t{servo:.2?}\t{tf:.2}\t{ttf:.2}\t{sf:.2}\t{stf:.2}");
                            },
                            Err(_) => print!("\t\t\t\t\t")
                        }
                    }

                    if self.ada {
                        match ada_url::Url::parse(&*line, None) {
                            Ok(url) => {
                                let urls = (0..self.num).map(|_| url.clone()).collect::<Vec<_>>();

                                let timer = std::time::Instant::now();

                                for mut url in urls {
                                    let mut query = unsafe {MaybeQuery::new_unchecked(url.search().strip_prefix('?'), r#type)};

                                    let _ = query.set(name, index, value);

                                    url.set_search(query.into_owned().as_str());
                                }

                                let ada = timer.elapsed();
                                ada_total += ada;

                                let tf  = ada      .as_secs_f64() / burl_t      .as_secs_f64();
                                let ttf = ada_total.as_secs_f64() / burl_t_total.as_secs_f64();
                                let sf  = ada      .as_secs_f64() / burl_s      .as_secs_f64();
                                let stf = ada_total.as_secs_f64() / burl_s_total.as_secs_f64();

                                print!("\t{ada:.2?}\t{tf:.2}\t{ttf:.2}\t{sf:.2}\t{stf:.2}");
                            },
                            Err(_) => print!("\t\t\t\t\t")
                        }
                    }
                },
                Err(_) => {
                    print!("\t");
                    if self.servo {print!("\t\t\t\t\t");}
                    if self.ada   {print!("\t\t\t\t\t");}
                }
            }

            println!();
        }
    }
}
