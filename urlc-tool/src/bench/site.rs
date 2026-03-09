//! Site HTTP.

use super::prelude::*;

/// Site.
#[derive(Debug, Parser)]
pub struct Args {
    /// The name
    #[arg(long)]
    pub name: String,
    /// The task.
    #[arg(long)]
    pub task: String,
    /// The num.
    #[arg(long)]
    pub num: u64,
    /// The tool.
    #[arg(long)]
    pub tool: Tool,
    /// The client.
    #[arg(long)]
    pub client: Client,
    /// TLS.
    #[arg(long)]
    pub tls: bool,
    /// The number of clients to use at once.
    #[arg(long, default_value_t = 1)]
    pub parallel: u64,
}

/// The tool to measure with.
#[derive(Debug, Clone, Copy, ValueEnum, Deserialize)]
pub enum Tool {
    /// Massif.
    Massif,
    /// Callgrind.
    Callgrind,
}

/// The client to use.
#[derive(Debug, Clone, Copy, ValueEnum, Deserialize)]
pub enum Client {
    /// Site CLIent (HTTP).
    SiteClientHttp,
    /// Site CLIent (WebSocket).
    SiteClientWs,
    /// Curl.
    Curl,
    /// Websocat.
    Websocat,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) -> String {
        let Self {name, task, num, tool, client, tls, parallel} = self;

        let out_dir = format!("urlc-tool/out/bench/site/{tool:?}/{client:?}-{tls}/{name}/{num}");

        let _ = std::fs::remove_dir_all(&out_dir);
        std::fs::create_dir_all(&out_dir).unwrap();

        let _stdin_handle = write_stdin(&task, num);

        assert_no_site();

        let mut cmd = Command::new("valgrind");

        let out = match tool {
            Tool::Massif => {
                let out = format!("{out_dir}/massif.out");

                cmd.args(["--tool=massif", &format!("--massif-out-file={out}")]);

                out
            },
            Tool::Callgrind => {
                let out = format!("{out_dir}/callgrind.out");

                cmd.args(["--tool=callgrind", "--separate-threads=yes", &format!("--callgrind-out-file={out}")]);

                out
            },
        };

        cmd.args(["target/release/url-cleaner-site", "--port", "9148"]);

        if tls {
            cmd.args([
                "--key", "urlc-tool/src/bench/urlcs-bench.key",
                "--cert", "urlc-tool/src/bench/urlcs-bench.crt",
            ]);
        }

        cmd.stdout(std::process::Stdio::null());
        cmd.stderr(new_file(format!("{out_dir}/site-stderr.txt")));

        let mut child = TerminateOnDrop(cmd.spawn().unwrap());

        await_site(&mut child.0);

        let tlss = match tls {
            false => "",
            true  => "s",
        };

        let mut cmd = match client {
            Client::SiteClientHttp => {
                let mut cmd = Command::new("target/release/url-cleaner-site-client");

                cmd.arg("clean");
                cmd.arg(format!("http{tlss}://127.0.0.1:9148"));

                cmd
            },
            Client::SiteClientWs => {
                let mut cmd = Command::new("target/release/url-cleaner-site-client");

                cmd.arg("clean");
                cmd.arg(format!("ws{tlss}://127.0.0.1:9148"));

                cmd
            },
            Client::Curl => {
                let mut cmd = Command::new("curl");

                cmd.args(["-T", "-"]);
                cmd.arg(format!("http{tlss}://127.0.0.1:9148/clean"));

                cmd
            },
            Client::Websocat => {
                let mut cmd = Command::new("websocat");

                cmd.arg(format!("ws{tlss}://127.0.0.1:9148/clean"));

                cmd
            },
        };

        cmd.stdin(File::open(STDIN).unwrap());

        let mut clients = Vec::new();

        for i in 0..parallel {
            cmd.stdout(std::process::Stdio::null());
            cmd.stderr(new_file(format!("{out_dir}/client-{i}-stderr.txt")));
            clients.push(TerminateOnDrop(cmd.spawn().unwrap()));
        }

        for mut client in clients {
            assert_eq!(client.0.wait().unwrap().code(), Some(0));
        }

        out
    }
}
