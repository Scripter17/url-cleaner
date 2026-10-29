//! PSL.

use clap::Parser;
use psl::Psl;

use crate::util::BasicBenchThing;

/// PSL.
///
/// The columns are:
///
/// 1. The line number starting at 1.
///
/// 2. better_url::util::get_domain_suffix.
///
/// 3. psl::suffix.
///
/// 4. The current row's column 3 divided by the current row's column 2.
///
/// 5. The current sum of column 3 divided by the current sum of column 2.
///
/// 6. publicsuffix::List::suffix.
///
/// 7. The current row's column 6 divided by the current row's column 2.
///
/// 8. The current sum of column 6 divided by the current sum of column 2.
#[derive(Debug, Parser)]
pub struct Args {
    /// The [`BasicBenchThing`].
    #[command(flatten)]
    pub basic_bench_thing: BasicBenchThing,
}

/// The current Public Suffix List data.
///
/// Should be kept up to date with the ./update.sh at the crate root.
const PSL_DATA: &str = include_str!("public_suffix_list.dat");

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let dyn_list = publicsuffix::List::from_bytes(PSL_DATA.as_bytes()).unwrap();

        let mut burl_total          = std::time::Duration::default();
        let mut psl_total           = std::time::Duration::default();
        let mut public_suffix_total = std::time::Duration::default();

        for (i, line) in (1..).zip(std::io::stdin().lines().map(Result::unwrap)) {
            print!("{i}");



            let burl = self.basic_bench_thing.bench(|| better_url::util::get_domain_suffix(&line));
            burl_total += burl;

            print!("\t{burl:.2?}");



            let psl = self.basic_bench_thing.bench(|| psl::suffix(line.as_bytes()));
            psl_total += psl;

            let factor       = psl      .as_secs_f64() / burl      .as_secs_f64();
            let factor_total = psl_total.as_secs_f64() / burl_total.as_secs_f64();

            print!("\t{psl:.2?}\t{factor:.2?}\t{factor_total:.2?}");



            let public_suffix = self.basic_bench_thing.bench(|| dyn_list.suffix(line.as_bytes()));
            public_suffix_total += public_suffix;

            let factor       = public_suffix      .as_secs_f64() / burl      .as_secs_f64();
            let factor_total = public_suffix_total.as_secs_f64() / burl_total.as_secs_f64();

            print!("\t{public_suffix:.2?}\t{factor:.2?}\t{factor_total:.2?}");



            println!();
        }
    }
}
