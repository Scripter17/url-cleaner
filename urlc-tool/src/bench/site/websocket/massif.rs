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
    pub num: usize,

    /// Enable TLS.
    #[arg(long)]
    pub tls: bool
}

impl Args {
    /// Do the command.
    pub fn r#do(self) -> String {
        let out = match self.tls {
            false => "urlc-tool/out/bench/site-ws/massif",
            true  => "urlc-tool/out/bench/site-wss/massif"
        };

        std::fs::create_dir_all(out).unwrap();

        let stdin = crate::bench::make_stdin(&self.task, self.num);

        let out = format!("{out}/massif.out-{}-{}", self.name, self.num);

        let mut server = Command::new("valgrind");

        server.args([
            "-q",
            "--tool=massif",
            &format!("--massif-out-file={out}"),
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
            false => "ws://127.0.0.1:9148/clean_ws",
            true  => "wss://127.0.0.1:9148/clean_ws"
        };

        assert_eq!(Command::new(BINDIR.join("url-cleaner-site-ws-client"))
            .args([
                instance,
                "--input", stdin.path()
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn().unwrap().wait().unwrap().code(), Some(0));

        out
    }
}
