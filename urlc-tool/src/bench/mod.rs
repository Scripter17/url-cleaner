//! Benchmarking.

use super::prelude::*;

pub mod suite;
pub mod cli;
pub mod site;
pub mod site_client;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::super::prelude::*;

    pub use super::{cli, site, site_client};
    pub use super::SiteProtocol;
    pub use super::{write_stdin, get_table_entry};
}

/// Write the job to `urlc-tool/tmp/stdin.txt`.
pub fn write_stdin(task: &str, num: u64) {
    std::fs::create_dir_all("urlc-tool/tmp").unwrap();

    let mut file = std::io::BufWriter::new(
        std::fs::OpenOptions::new()
            .read(true).write(true).create(true).truncate(true)
            .open("urlc-tool/tmp/stdin.txt").unwrap()
    );

    for _ in 0..num {
        writeln!(file, "{task}").unwrap();
    }
}

/// Take an output file and get its table entry.
pub fn get_table_entry<P: AsRef<Path>>(path: P) -> String {
    match path.as_ref().file_name().unwrap().to_str().unwrap().split_once('.').unwrap().0 {
        "hyperfine" => format!("{:.1}", serde_json::from_str::<serde_json::Value>(&std::fs::read_to_string(path).unwrap()).unwrap()["results"][0]["mean"].as_f64().unwrap() * 1000.0),
        "massif" => {
            let mut ret = 0u64;
            for line in BufReader::new(File::open(path).unwrap()).lines() {
                if let Some(x) = line.unwrap().strip_prefix("mem_heap_B=") {
                    ret = ret.max(x.parse().unwrap());
                }
            }
            ret.to_formatted_string(&Locale::en)
        },
        "callgrind" => "...".into(),
        x => panic!("Unknwon result type {x}")
    }
}

/// The protocol to use.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SiteProtocol {
    /// HTTP.
    Http,
    /// HTTPS.
    Https,
    /// WebSocket.
    Ws,
    /// WebSocket Secure.
    Wss,
}

impl SiteProtocol {
    /// The name of the protocol.
    pub fn name(self) -> &'static str {
        match self {
            Self::Http  => "HTTP",
            Self::Https => "HTTPS",
            Self::Ws    => "WebSocket",
            Self::Wss   => "WebSocket Secure"
        }
    }

    /// The scheme.
    pub fn scheme(self) -> &'static str {
        match self {
            Self::Http  => "http",
            Self::Https => "https",
            Self::Ws    => "ws",
            Self::Wss   => "wss"
        }
    }
}

/// Benchmarking.
#[derive(Debug, Parser)]
#[allow(missing_docs, reason = "Makes clap inherit the docs.")]
pub enum Args {
    Suite     (suite      ::Args),
    Cli       (cli        ::Args),
    Site      (site       ::Args),
    SiteClient(site_client::Args),
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Self::Suite     (args) => args.r#do(),
            Self::Cli       (args) => println!("{}", args.r#do()),
            Self::Site      (args) => println!("{}", args.r#do()),
            Self::SiteClient(args) => println!("{}", args.r#do()),
        }
    }
}
