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

    /// Enable TLS.
    #[arg(long)]
    pub tls: bool,

    /// The numer of runs to measure..
    #[arg(long)]
    pub runs: Option<usize>,
    /// The number of warmup runs to do.
    #[arg(long)]
    pub warmup: Option<usize>
}

impl Args {
    /// Do the command.
    pub fn r#do(self) -> String {
        let out = match self.tls {
            false => "urlc-tool/out/bench/site-http/hyperfine",
            true  => "urlc-tool/out/bench/site-https/hyperfine"
        };

        std::fs::create_dir_all(out).unwrap();

        let stdin = crate::bench::make_stdin(&self.task, self.num);

        let out = format!("{out}/hyperfine.out-{}-{}.json", self.name, self.num);

        let mut server = Command::new(BINDIR.join("url-cleaner-site"));

        server.args(["--port", "9148"]);

        if self.tls {
            server.args([
                "--key" , "urlc-tool/example-urlcs.key",
                "--cert", "urlc-tool/example-urlcs.crt"
            ]);
        }

        let _server = TerminateOnDrop(server
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn().unwrap());

        wait_for_server();

        let instance = match self.tls {
            false => "http://127.0.0.1:9148/clean",
            true  => "https://127.0.0.1:9148/clean"
        };

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
            &format!("curl {instance} -H Expect: --data-binary @{}", stdin.path())
        ]);

        assert_eq!(command
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn().unwrap().wait().unwrap().code(), Some(0));

        out
    }
}
