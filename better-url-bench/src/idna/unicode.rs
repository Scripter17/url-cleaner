//! ToUnicode.

use clap::Parser;

/// ToUnicode.
#[derive(Debug, Parser)]
pub struct Args {
    /** Compare to all parsers.       **/ #[arg(long)] pub all  : bool,
    /** Compare to Servo's URL crate. **/ #[arg(long)] pub servo: bool,
    /** Compare to Ada's URL crate.   **/ #[arg(long)] pub ada  : bool,

    /// The number of times to do each input.
    #[arg(long)]
    pub num: usize,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let mut burl_total  = std::time::Duration::default();
        let mut servo_total = std::time::Duration::default();
        let mut ada_total   = std::time::Duration::default();

        let servo = self.servo || self.all;
        let ada   = self.ada   || self.all;

        for (i, line) in (1..).zip(std::io::stdin().lines().map(Result::unwrap)) {
            print!("{i}");

            let timer = std::time::Instant::now();

            for _ in 0..self.num {
                let _ = std::hint::black_box(better_url::util::domain_host_to_unicode(&*line));
            }

            let burl = timer.elapsed();
            burl_total += burl;

            print!("\t{burl:.2?}");



            if servo {
                let timer = std::time::Instant::now();

                for _ in 0..self.num {
                    let _ = std::hint::black_box(idna::domain_to_unicode(&line));
                }

                let servo = timer.elapsed();
                servo_total += servo;

                let factor       = servo      .as_secs_f64() / burl      .as_secs_f64();
                let factor_total = servo_total.as_secs_f64() / burl_total.as_secs_f64();

                print!("\t{servo:.2?}\t{factor:.2?}\t{factor_total:.2?}");
            }



            if ada {
                let timer = std::time::Instant::now();

                for _ in 0..self.num {
                    let _ = std::hint::black_box(ada_url::Idna::unicode(&line));
                }

                let ada = timer.elapsed();
                ada_total += ada;

                let factor       = ada      .as_secs_f64() / burl      .as_secs_f64();
                let factor_total = ada_total.as_secs_f64() / burl_total.as_secs_f64();

                print!("\t{ada:.2?}\t{factor:.2?}\t{factor_total:.2?}");
            }



            println!();
        }
    }
}
