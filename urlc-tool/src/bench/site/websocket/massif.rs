//! Massif.

use super::prelude::*;

/// Massif.
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
const OUT: &str = "urlc-tool/out/bench/site-ws/massif/";

impl Args {
    /// Do the command.
    pub fn r#do(self) -> String {
        std::fs::create_dir_all(OUT).unwrap();

        let stdin = crate::bench::make_stdin(&self.task, self.num);

        let out = format!("{OUT}/massif.out-{}-{}", self.name, self.num);

        let _server = TerminateOnDrop(Command::new("valgrind")
            .args([
                "-q",
                "--tool=massif",
                &format!("--massif-out-file={out}"),
                "target/release/url-cleaner-site",
                "--port", "9148"
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn().unwrap());

        wait_for_server();

        assert_eq!(Command::new("websocat")
            .arg(format!("readfile:{}", stdin.path()))
            .arg("ws://127.0.0.1:9148/clean_ws")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn().unwrap().wait().unwrap().code(), Some(0));

        out
    }
}
