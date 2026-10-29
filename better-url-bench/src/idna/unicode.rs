//! ToUnicode.

use clap::Parser;

use crate::util::BasicBenchThing;

/// ToUnicode.
#[derive(Debug, Parser)]
pub struct Args {
    /** Compare to all parsers.       */ #[arg(long)] pub all  : bool,
    /** Compare to Servo's URL crate. */ #[arg(long)] pub servo: bool,
    /** Compare to Ada's URL crate.   */ #[arg(long)] pub ada  : bool,

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

        for (i, line) in (1..).zip(std::io::stdin().lines().map(Result::unwrap)) {
            print!("{i}");

            let burl = self.basic_bench_thing.bench_with_input(better_url::util::domain_host_to_unicode, &line);
            burl_total += burl;

            print!("\t{burl:.2?}");



            if self.servo {
                let servo = self.basic_bench_thing.bench_with_input(idna::domain_to_unicode, &line);
                servo_total += servo;

                let factor       = servo      .as_secs_f64() / burl      .as_secs_f64();
                let factor_total = servo_total.as_secs_f64() / burl_total.as_secs_f64();

                print!("\t{servo:.2?}\t{factor:.2?}\t{factor_total:.2?}");
            }



            if self.ada {
                let ada = self.basic_bench_thing.bench_with_input(ada_url::Idna::unicode, &line);
                ada_total += ada;

                let factor       = ada      .as_secs_f64() / burl      .as_secs_f64();
                let factor_total = ada_total.as_secs_f64() / burl_total.as_secs_f64();

                print!("\t{ada:.2?}\t{factor:.2?}\t{factor_total:.2?}");
            }



            println!();
        }
    }
}
