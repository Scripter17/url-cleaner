//! Site.

pub use super::prelude::*;

/// Site.
#[derive(Debug, Parser)]
pub struct Args {
    /// The name of the benchmark.
    #[arg(long)]
    pub name: String,
    /// The [`JobConfig`].
    #[command(flatten)]
    pub job_config: JobConfig,

    /// The [`Tool`].
    #[arg(long)]
    pub tool: Tool,

    /// The [`Api`].
    #[arg(long)]
    pub api: Api
}

/// The API to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Api {
    /// HTTP.
    Http,
    /// HTTPS.
    Https,
    /// WebSocket.
    Ws,
    /// WebSocket Secure.
    Wss,
}

impl Api {
    /// The name of the API.
    pub fn name(self) -> &'static str {
        match self {
            Self::Http  => "HTTP",
            Self::Https => "HTTPS",
            Self::Ws    => "WebSocket",
            Self::Wss   => "WebSocket Secure"
        }
    }

    /// The protocol.
    pub fn protocol(self) -> &'static str {
        match self {
            Self::Http  => "http",
            Self::Https => "https",
            Self::Ws    => "ws",
            Self::Wss   => "wss"
        }
    }
}

impl Args {
    /// Do the command.
    pub fn r#do(self) -> String {
        let name = &self.name;
        let num = self.job_config.num;
        let api = self.api.protocol();

        let out = match self.tool {
            Tool::Hyperfine => format!("urlc-tool/out/bench/site/{api}/hyperfine/{name}/hyperfine.out-site-{api}-{name}-{num}.json"),
            Tool::Massif    => format!("urlc-tool/out/bench/site/{api}/massif/{name}/massif.out-site-{api}-{name}-{num}"),
            Tool::Callgrind => format!("urlc-tool/out/bench/site/{api}/callgrind/{name}/{num}/callgrind.out-site-{api}-{name}-{num}")
        };

        let (dir, prefix) = out.rsplit_once('/').unwrap();

        std::fs::create_dir_all(dir).unwrap();

        for entry in std::fs::read_dir(dir).unwrap().map(Result::unwrap) {
            if let Some(name) = entry.file_name().to_str() && name.starts_with(prefix) {
                std::fs::remove_file(entry.path()).unwrap();
            }
        }

        self.job_config.make();

        let mut cmd = match self.tool {
            Tool::Hyperfine => Command::new("target/release/url-cleaner-site"),
            Tool::Massif => {
                let mut cmd = Command::new("valgrind");
                cmd.args([
                    "--tool=massif",
                    &format!("--massif-out-file={out}"),
                    "target/release/url-cleaner-site"
                ]);
                cmd
            },
            Tool::Callgrind => {
                let mut cmd = Command::new("valgrind");
                cmd.args([
                    "--tool=callgrind",
                    &format!("--callgrind-out-file={out}"),
                    "--separate-threads=yes",
                    "target/release/url-cleaner-site"
                ]);
                cmd
            }
        };

        cmd.args(["--port", "9148"]);

        if matches!(self.api, Api::Https | Api::Wss) {
            cmd.args([
                "--key" , "urlc-tool/src/bench/tls.key",
                "--cert", "urlc-tool/src/bench/tls.crt"
            ]);
        }

        cmd.stdout(std::process::Stdio::null());
        cmd.stderr(std::process::Stdio::null());

        let _server_handle = TerminateOnDrop(cmd.spawn().unwrap());

        for fails in 0.. {
            match std::net::TcpStream::connect("127.0.0.1:9148") {
                Ok (_) => break,
                Err(_) => if fails == 1000 {panic!("Site failed to start.");}
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        let protocol = self.api.protocol();

        let mut cmd = match self.tool {
            Tool::Hyperfine => {
                let mut cmd = Command::new("hyperfine");

                cmd.args([
                    "--style", "none",
                    "--command-name", &self.name,
                    "--export-json", &out,
                    "--input", "urlc-tool/tmp/stdin.txt",
                    &format!("target/release/url-cleaner-site-client clean {protocol}://127.0.0.1:9148")
                ]);

                cmd
            },
            Tool::Massif | Tool::Callgrind => {
                let mut cmd = Command::new("target/release/url-cleaner-site-client");
                cmd.arg("clean");
                cmd.arg(format!("{protocol}://127.0.0.1:9148"));
                cmd.stdin(File::open("urlc-tool/tmp/stdin.txt").unwrap());
                cmd
            }
        };

        cmd.stdout(std::process::Stdio::null());
        cmd.stderr(std::process::Stdio::null());

        assert_eq!(cmd.spawn().unwrap().wait().unwrap().code(), Some(0));

        std::fs::remove_file("urlc-tool/tmp/stdin.txt").unwrap();

        out
    }
}
