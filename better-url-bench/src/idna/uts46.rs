//! UTS-46 mapping and normalizing.

use clap::Parser;

use crate::util::BasicBenchThing;

/// UTS-46 mapping and normalizing.
#[derive(Debug, Parser)]
pub struct Args {
    /** Compare to all alternatives. */ #[arg(long)] pub all  : bool,
    /** Compare to ICU4X.            */ #[arg(long)] pub icu4x: bool,

    /// The [`BasicBenchThing`].
    #[command(flatten)]
    pub basic_bench_thing: BasicBenchThing,
}

impl Args {
    /// Do the command.
    pub fn r#do(mut self) {
        let mut burl_total  = std::time::Duration::default();
        let mut icu4x_total = std::time::Duration::default();

        let icu_normalizer = icu_normalizer::uts46::Uts46MapperBorrowed::new();

        if self.all {
            self.icu4x = true;
        }

        for (i, line) in (1..).zip(std::io::stdin().lines().map(Result::unwrap)) {
            print!("{i}");

            let burl = self.basic_bench_thing.bench(|| better_url::util::uts46_map_normalize(&*line));
            burl_total += burl;

            print!("\t{burl:.2?}");


            if self.icu4x {
                let icu4x = self.basic_bench_thing.bench(|| icu_normalizer.map_normalize(line.chars()).collect::<String>());
                icu4x_total += icu4x;

                let factor       = icu4x      .as_secs_f64() / burl      .as_secs_f64();
                let factor_total = icu4x_total.as_secs_f64() / burl_total.as_secs_f64();

                print!("\t{icu4x:.2?}\t{factor:.2?}\t{factor_total:.2?}");
            }



            println!();
        }
    }
}
