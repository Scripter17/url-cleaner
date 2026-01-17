//! Benchmarking.

use super::prelude::*;

pub mod suite;
pub mod cli;
pub mod site;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::super::prelude::*;

    pub use super::{JobConfig, Tool, Target};
}

/// The config to make a job.
#[derive(Debug, Parser)]
pub struct JobConfig {
    /// The task to use.
    #[arg(long)]
    pub task: String,
    /// The number of [`Self::task`] to use.
    #[arg(long)]
    pub num: u64
}

impl JobConfig {
    /// Write the job to `urlc-tool/tmp/stdin.txt`.
    pub fn make(&self) {
        std::fs::create_dir_all("urlc-tool/tmp").unwrap();

        let mut file = std::io::BufWriter::new(
            std::fs::OpenOptions::new()
                .read(true).write(true).create(true).truncate(true)
                .open("urlc-tool/tmp/stdin.txt").unwrap()
        );

        for _ in 0..self.num {
            writeln!(file, "{}", self.task).unwrap();
        }
    }
}

/// The tools to benchmark with/
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Tool {
    /// Hyperfine.
    Hyperfine,
    /// Massif.
    Massif,
    /// Callgrind.
    Callgrind
}

impl Tool {
    /// Parse an output file and get its [`suite`] table entry.
    pub fn get_entry<P: AsRef<Path>>(self, path: &P) -> String {
        match self {
            Self::Hyperfine => format!("`{:.1}`", serde_json::from_str::<serde_json::Value>(&std::fs::read_to_string(path).unwrap()).unwrap()["results"][0]["mean"].as_f64().unwrap() * 1000.0),
            Self::Massif => {
                let mut ret = 0u64;
                for line in BufReader::new(File::open(path).unwrap()).lines() {
                    if let Some(x) = line.unwrap().strip_prefix("mem_heap_B=") {
                        ret = ret.max(x.parse().unwrap());
                    }
                }
                format!("`{}`", ret.to_formatted_string(&Locale::en))
            }
            Self::Callgrind => "`...`".to_string()
        }
    }
}

impl std::fmt::Display for Tool {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", match self {
            Self::Hyperfine => "hyperfine",
            Self::Massif    => "massif",
            Self::Callgrind => "callgrind"
        })
    }
}

/// The target to benchmark.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    /// CLI.
    Cli,
    /// Site.
    Site(site::Api)
}

impl ValueEnum for Target {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Cli,
            Self::Site(site::Api::Http ),
            Self::Site(site::Api::Https),
            Self::Site(site::Api::Ws   ),
            Self::Site(site::Api::Wss  ),
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Self::Cli                    => Some(PossibleValue::new("cli"  )),
            Self::Site(site::Api::Http ) => Some(PossibleValue::new("http" )),
            Self::Site(site::Api::Https) => Some(PossibleValue::new("https")),
            Self::Site(site::Api::Ws   ) => Some(PossibleValue::new("ws"   )),
            Self::Site(site::Api::Wss  ) => Some(PossibleValue::new("wss"  )),
        }
    }
}

/// Benchmarking.
#[derive(Debug, Parser)]
#[allow(missing_docs, reason = "Makes clap inherit the docs.")]
pub enum Args {
    Suite(suite::Args),
    Cli  (cli  ::Args),
    Site (site ::Args)
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Self::Suite(args) => args.r#do(),
            Self::Cli  (args) => println!("{}", args.r#do()),
            Self::Site (args) => println!("{}", args.r#do())
        }
    }
}
