//! Benchmark URL parsing.

use crate::prelude::*;

/// Compare Better URL and Servo URL parsing.
///
/// "big / small = more than one" = Better URL is faster.
#[derive(Debug, Parser)]
pub struct Args {
    /// The amount of parsings to do.
    #[arg(long)]
    pub num: usize,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        for line in std::io::stdin().lock().lines().map(Result::unwrap) {
            let timer = std::time::Instant::now();

            for _ in 0..self.num {
                let _ = BetterUrl::new(&*line);
            }

            let better = timer.elapsed();



            let timer = std::time::Instant::now();

            for _ in 0..self.num {
                if let Ok(url) = url::Url::parse(&line) {
                    let scheme_type = SchemeDetails::new_unchecked(url.scheme()).r#type();

                    if let Some(host) = url.host_str() {
                        let _ = HostDetails::parse(host, scheme_type);
                    }
                }
            }

            let worse = timer.elapsed();



            println!("{worse:?} / {better:?} = {}", worse.as_secs_f64() / better.as_secs_f64());
        }
    }
}
