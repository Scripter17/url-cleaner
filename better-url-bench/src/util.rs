//! Utilities.

use std::time::{Instant, Duration};

use clap::Parser;

/// Parse a [`Duration`]'s [`std::fmt::Debug`] output.
pub fn parse_duration(s: &str) -> Option<Duration> {
    let secs = match s {
        s if let Some(x) = s.strip_suffix("ns") => x.parse::<f64>().ok()? / 1_000_000_000f64,
        s if let Some(x) = s.strip_suffix("µs") => x.parse::<f64>().ok()? / 1_000_000f64,
        s if let Some(x) = s.strip_suffix("ms") => x.parse::<f64>().ok()? / 1_000f64,
        s if let Some(x) = s.strip_suffix( "s") => x.parse::<f64>().ok()?,
        _ => None?
    };

    Some(Duration::from_secs_f64(secs))
}

/// A basic benchmarker thing.
#[derive(Debug, Clone, Copy, Parser)]
pub struct BasicBenchThing {
    /** The number of parsings to do for each subdatapoint.  */ #[arg(long, default_value_t = 1000)] pub num      : usize,
    /** The number of most recent subdatapoints to evaluate. */ #[arg(long, default_value_t = 8   )] pub window   : usize,
    /** The max % by which subdatapoints can differ.         */ #[arg(long, default_value_t = 0.05)] pub tolerance: f64  ,
}

impl BasicBenchThing {
    /// Repeatedly measure how long it takes to run `f` `num` times until the past `window` measurements are all within `tolerance`% of each other.
    pub fn bench<F: FnMut() -> T, T>(self, mut f: F) -> Duration {
        let mut times = Vec::with_capacity(self.window);

        loop {
            let timer = Instant::now();

            for _ in 0..self.num {
                let _ = std::hint::black_box(f());
            }

            times.push(timer.elapsed());

            if times.len() == self.window {
                if times.iter().all(|&a| times.iter().all(|&b| ((a.as_secs_f64() / b.as_secs_f64()) - 1.0).abs() < self.tolerance)) {
                    return times.iter().sum::<Duration>() / times.len() as u32;
                } else {
                    times.remove(0);
                }
            }
        }
    }

    /// [`Self::bench`] but make a [`Vec`] of [`Self::num`] `input`s and feed them into `f` one by one.
    pub fn bench_with_input<F: FnMut(T) -> U, T: Clone, U>(self, mut f: F, input: T) -> Duration {
        let mut times = Vec::with_capacity(self.window);

        loop {
            let inputs = vec![input.clone(); self.num];

            let timer = Instant::now();

            for input in inputs {
                let _ = std::hint::black_box(f(input));
            }

            times.push(timer.elapsed());

            if times.len() == self.window {
                if times.iter().all(|&a| times.iter().all(|&b| ((a.as_secs_f64() / b.as_secs_f64()) - 1.0).abs() < self.tolerance)) {
                    return times.iter().sum::<Duration>() / times.len() as u32;
                } else {
                    times.remove(0);
                }
            }
        }
    }
}
