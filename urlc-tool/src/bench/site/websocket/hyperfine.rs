//! Hyperfine.

use super::prelude::*;

/// Hyperfine.
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
    pub num: usize,

    /// The numer of runs to measure..
    #[arg(long)]
    pub runs: Option<usize>,
    /// The number of warmup runs to do.
    #[arg(long)]
    pub warmup: Option<usize>
}

/// The output directory.
const OUT: &str = "urlc-tool/out/bench/site-ws/hyperfine";

impl Args {
    /// Do the command.
    pub fn r#do(self) -> String {
        std::fs::create_dir_all(OUT).unwrap();

        let stdin = crate::bench::make_stdin(&self.task, self.num);

        let out = format!("{OUT}/hyperfine.out-{}-{}.json", self.name, self.num);

        let _server = TerminateOnDrop(Command::new(BINDIR.join("url-cleaner-site"))
            .args(["--port", "9148"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn().unwrap());

        wait_for_server();

        let mut command = Command::new("hyperfine");
        command.args([
            "--style", "none",
            "--command-name", &self.name,
            "--export-json", &out
        ]);

        if let Some(runs  ) = self.runs   {command.args(["--runs"  , &runs  .to_string()]);}
        if let Some(warmup) = self.warmup {command.args(["--warmup", &warmup.to_string()]);}

        command.args([
            "--",
            &format!("{} ws://127.0.0.1:9148/clean_ws --input {} --silent", BINDIR.join("url-cleaner-site-ws-client").display(), stdin.path())
        ]);

        assert_eq!(command
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn().unwrap().wait().unwrap().code(), Some(0));

        out
    }
}
