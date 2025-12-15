//! Quick.

use super::prelude::*;

/// Quick.
///
/// Reads task lines from STDIN and outputs lines containing the mean (per using Hyperfine on CLI), a tab, and the task line.
#[derive(Debug, Parser)]
pub struct Args {
    /// The amount to benchmark.
    #[arg(long, default_value_t = 10000)]
    num: usize,
    /// The amount of runs.
    #[arg(long, default_value_t = 5)]
    runs: usize,
    /// The amount of warmup runs.
    #[arg(long, default_value_t = 5)]
    warmup: usize
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        for (i, line) in std::io::stdin().lock().lines().map(Result::unwrap).enumerate() {
            let data_path = crate::bench::cli::hyperfine::Args {
                name: format!("quick-{i}"),
                task: line.clone(),
                num: self.num,

                runs: Some(self.runs),
                warmup: Some(self.warmup)
            }.r#do();

            let data = serde_json::from_str::<serde_json::Value>(&std::fs::read_to_string(data_path).unwrap()).unwrap();
            let mean = data["results"][0]["mean"].as_f64().unwrap();

            println!("{:.1}\t{line}", mean * 1000.0);
        }
    }
}
