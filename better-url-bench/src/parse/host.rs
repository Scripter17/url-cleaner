//! Host.

use std::io::BufRead;

use clap::Parser;

use better_url::prelude::*;

/// Host.
///
/// The columns are:
///
/// 1. The line number.
///
/// 2. The time it took to parse a FileHost the specified number of times.
///
/// 3. The time it took to parse a SpecialNotFileHost the specified number of times.
///
/// 4. The time it took to parse a NonSpecialHost the specified number of times.
///
/// 5. The time it took to parse a DomainHost the specified number of times.
///
/// 6. The time it took to parse a Ipv4Host the specified number of times.
///
/// 7. The time it took to parse a Ipv6Host the specified number of times.
///
/// 8. The time it took to parse a OpaqueHost the specified number of times.
///
/// 9. The time it took to parse a EmptyHost the specified number of times.
#[derive(Debug, Parser)]
pub struct Args {
    /// The amount to do.
    #[arg(long)]
    pub num: usize
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        for (i, value) in std::io::stdin().lock().lines().map(Result::unwrap).enumerate() {
            print!("{i}");

            let timer = std::time::Instant::now();

            for _ in 0..self.num {
                let _ = FileHost::new(&*value);
            }

            print!("\t{:.2?}", timer.elapsed());

            let timer = std::time::Instant::now();

            for _ in 0..self.num {
                let _ = SpecialNotFileHost::new(&*value);
            }

            print!("\t{:.2?}", timer.elapsed());

            let timer = std::time::Instant::now();

            for _ in 0..self.num {
                let _ = NonSpecialHost::new(&*value);
            }

            print!("\t{:.2?}", timer.elapsed());




            let timer = std::time::Instant::now();

            for _ in 0..self.num {
                let _ = DomainHost::new(&*value);
            }

            print!("\t{:.2?}", timer.elapsed());

            let timer = std::time::Instant::now();

            for _ in 0..self.num {
                let _ = Ipv4Host::new(&*value);
            }

            print!("\t{:.2?}", timer.elapsed());

            let timer = std::time::Instant::now();

            for _ in 0..self.num {
                let _ = Ipv6Host::new(&*value);
            }

            print!("\t{:.2?}", timer.elapsed());

            let timer = std::time::Instant::now();

            for _ in 0..self.num {
                let _ = OpaqueHost::new(&*value);
            }

            print!("\t{:.2?}", timer.elapsed());

            let timer = std::time::Instant::now();

            for _ in 0..self.num {
                let _ = EmptyHost::new(&*value);
            }

            print!("\t{:.2?}", timer.elapsed());



            println!();
        }
    }
}
