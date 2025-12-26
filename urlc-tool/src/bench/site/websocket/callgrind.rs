//! Callgrind.

use super::prelude::*;

/// Callgrind.
#[derive(Debug, Parser)]
pub struct Args {
    /// The name of the benchmark.
    #[arg(long)]
    pub name: String,
    /// The task line.
    #[arg(long)]
    pub task: String,
    /// The number to clean per run.
    #[arg(long)]
    pub num: usize
}

/// The output directory.
const OUT: &str = "urlc-tool/out/bench/site-ws/callgrind";

impl Args {
    /// Do the command.
    pub fn r#do(self) -> String {
        std::fs::create_dir_all(OUT).unwrap();

        let stdin = crate::bench::make_stdin(&self.task, self.num);

        let out = format!("{OUT}/callgrind.out-{}-{}", self.name, self.num);

        let _server = TerminateOnDrop(Command::new("valgrind")
            .args([
                "-q",
                "--tool=callgrind",
                "--separate-threads=yes",
                &format!("--callgrind-out-file={out}"),
                BINDIR.join("url-cleaner-site").to_str().unwrap(),
                "--port", "9148"
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn().unwrap());

        wait_for_server();

        assert_eq!(Command::new(BINDIR.join("url-cleaner-site-ws-client"))
            .args([
                "ws://127.0.0.1:9148/clean_ws",
                "--input", stdin.path()
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn().unwrap().wait().unwrap().code(), Some(0));

        out
    }
}
