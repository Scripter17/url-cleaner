//! Site CLIent.

use super::prelude::*;

/// Site CLIent.
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
}

/// The tool to measure with.
#[derive(Debug, Clone, Copy, ValueEnum, Deserialize)]
pub enum Tool {
    /// Hyperfine.
    Hyperfine,
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
        let Self {name, task, num, tool, client, tls} = self;

        let out_dir = format!("urlc-tool/out/bench/site/{tool:?}/{client:?}-{tls}/{name}/{num}");

        let _ = std::fs::remove_dir_all(&out_dir);
        std::fs::create_dir_all(&out_dir).unwrap();

        let _stdin_handle = write_stdin(&task, num);

        assert_no_site();

        let mut site = Command::new("target/release/url-cleaner-site");

        site.args(["--port", "9148"]);

        if tls {
            site.args([
                "--key", "urlc-tool/src/bench/urlcs-bench.key",
                "--cert", "urlc-tool/src/bench/urlcs-bench.crt",
            ]);
        }

        site.stdout(std::process::Stdio::null());
        site.stderr(new_file(format!("{out_dir}/site-stderr.txt")));

        let mut site = TerminateOnDrop(site.spawn().unwrap());

        await_site(&mut site.0);

        let tlss = match tls {
            false => "",
            true  => "s",
        };

        let (out, mut cmd) = match tool {
            Tool::Hyperfine => {
                let out = format!("{out_dir}/hyperfine.json");

                let mut cmd = Command::new("hyperfine");

                cmd.args([
                    "--show-output",
                    "--input", STDIN,
                    "--export-json", &out,
                ]);

                cmd.arg(match client {
                    Client::SiteClientHttp => format!("target/release/url-cleaner-site-client clean http{tlss}://127.0.0.1:9148"),
                    Client::SiteClientWs   => format!("target/release/url-cleaner-site-client clean ws{tlss}://127.0.0.1:9148"),
                    Client::Curl           => format!("curl -T - http{tlss}://127.0.0.1:9148/clean"),
                    Client::Websocat       => format!("websocat ws{tlss}://127.0.0.1:9148/clean"),
                });

                (out, cmd)
            },
            Tool::Massif => {
                let out = format!("{out_dir}/massif.out");

                let mut cmd = Command::new("valgrind");

                cmd.args([
                    "--tool=massif",
                    &format!("--massif-out-file={out}"),
                ]);

                match client {
                    Client::SiteClientHttp => cmd.args(["target/release/url-cleaner-site-client", "clean", &format!("http{tlss}://127.0.0.1:9148")]),
                    Client::SiteClientWs   => cmd.args(["target/release/url-cleaner-site-client", "clean", &format!("ws{tlss}://127.0.0.1:9148")]),
                    Client::Curl           => cmd.args(["curl", "-T", "-", &format!("http{tlss}://127.0.0.1:9148/clean")]),
                    Client::Websocat       => cmd.args(["websocat", &format!("ws{tlss}://127.0.0.1:9148/clean")]),
                };

                cmd.stdin(File::open(STDIN).unwrap());

                (out, cmd)
            },
            Tool::Callgrind => {
                let out = format!("{out_dir}/callgrind.out");

                let mut cmd = Command::new("valgrind");

                cmd.args([
                    "--tool=callgrind",
                    "--separate-threads=yes",
                    &format!("--callgrind-out-file={out}"),
                ]);

                match client {
                    Client::SiteClientHttp => cmd.args(["target/release/url-cleaner-site-client", "clean", &format!("http{tlss}://127.0.0.1:9148")]),
                    Client::SiteClientWs   => cmd.args(["target/release/url-cleaner-site-client", "clean", &format!("ws{tlss}://127.0.0.1:9148")]),
                    Client::Curl           => cmd.args(["curl", "-T", "-", &format!("http{tlss}://127.0.0.1:9148/clean")]),
                    Client::Websocat       => cmd.args(["websocat", &format!("ws{tlss}://127.0.0.1:9148/clean")]),
                };

                cmd.stdin(File::open(STDIN).unwrap());

                (out, cmd)
            },
        };

        cmd.stdout(std::process::Stdio::null());
        cmd.stderr(new_file(format!("{out_dir}/client-stderr.txt")));

        assert_eq!(cmd.spawn().unwrap().wait().unwrap().code(), Some(0));

        out
    }
}
