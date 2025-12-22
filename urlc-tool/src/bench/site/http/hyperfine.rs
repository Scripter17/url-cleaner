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
const OUT: &str = "urlc-tool/out/bench/site-http/hyperfine/";

impl Args {
    /// Do the command.
    pub fn r#do(self) -> String {
        std::fs::create_dir_all(OUT).unwrap();

        let stdin = crate::bench::make_stdin(&self.task, self.num);

        let out = format!("{OUT}/hyperfine.out-{}-{}.json", self.name, self.num);

        let _server = TerminateOnDrop(Command::new(BINDIR.join("url-cleaner-site"))
            .args(["--port", "9148", "--max-payload", "1GiB"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn().unwrap());

        wait_for_server();

        let mut command = Command::new("hyperfine");
        command.args([
            "--command-name", &self.name,
            "--export-json", &out
        ]);

        if let Some(runs  ) = self.runs   {command.args(["--runs"  , &runs  .to_string()]);}
        if let Some(warmup) = self.warmup {command.args(["--warmup", &warmup.to_string()]);}

        command.args([
            "--",
            &format!("curl http://127.0.0.1:9148/clean --data-binary @{}", stdin.path())
        ]);

        assert_eq!(command
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn().unwrap().wait().unwrap().code(), Some(0));

        out
    }
}
