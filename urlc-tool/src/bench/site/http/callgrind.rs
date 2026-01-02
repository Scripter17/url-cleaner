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
    pub num: usize,

    /// Enable TLS.
    #[arg(long)]
    pub tls: bool
}

impl Args {
    /// Do the command.
    pub fn r#do(self) -> String {
        let out = match self.tls {
            false => "urlc-tool/out/bench/site-http/callgrind",
            true  => "urlc-tool/out/bench/site-https/callgrind"
        };

        std::fs::create_dir_all(out).unwrap();

        let stdin = crate::bench::make_stdin(&self.task, self.num);

        let out = format!("{out}/callgrind.out-{}-{}", self.name, self.num);

        let mut server = Command::new("valgrind");

        server.args([
            "-q",
            "--tool=callgrind",
            "--separate-threads=yes",
            &format!("--callgrind-out-file={out}"),
            BINDIR.join("url-cleaner-site").to_str().unwrap(),
            "--port", "9148"
        ]);

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

        assert_eq!(Command::new("curl")
            .args([
                instance,
                "-H", "Expect:",
                "--data-binary", &format!("@{}", stdin.path())
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn().unwrap().wait().unwrap().code(), Some(0));

        out
    }
}
