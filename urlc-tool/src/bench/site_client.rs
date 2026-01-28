//! Site CLIent.

use super::prelude::*;

/// Site CLIent.
#[derive(Debug, Parser)]
pub struct Args {
    /// The name of the job.
    #[arg(long)]
    pub name: String,
    /// The task of the job.
    #[arg(long)]
    pub task: String,
    /// The number of tasks for the job.
    #[arg(long)]
    pub num: u64,

    /// The [`Tool`].
    #[arg(long)]
    pub tool: Tool,
    /// The [`SiteProtocol`].
    #[arg(long)]
    pub protocol: SiteProtocol
}

/// The tool to measure with.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Tool {
    /// Hyperfine.
    Hyperfine,
    /// Massif.
    Massif,
    /// Callgrind.
    Callgrind
}

impl Args {
    /// Do the command.
    pub fn r#do(self) -> String {
        let name = self.name;
        let task = self.task;
        let num  = self.num;
        let tool = format!("{:?}", self.tool).to_lowercase();

        let out = format!("urlc-tool/out/bench/cli/{tool}/{name}/{tool}.out-{name}-{num}");

        let (dir, prefix) = out.rsplit_once('/').unwrap();

        std::fs::create_dir_all(dir).unwrap();

        for entry in std::fs::read_dir(dir).unwrap().map(Result::unwrap) {
            if let Some(name) = entry.file_name().to_str() && name.starts_with(prefix) {
                std::fs::remove_file(entry.path()).unwrap();
            }
        }

        let mut server = Command::new("target/release/url-cleaner-site");

        server.args([
            "--port", "9148"
        ]);

        if matches!(self.protocol, SiteProtocol::Https | SiteProtocol::Wss) {
            server.args([
                "--key", "urlc-tool/src/bench/tls.key",
                "--cert", "urlc-tool/src/bench/tls.crt"
            ]);
        }

        server.stdout(std::process::Stdio::null());
        server.stderr(std::process::Stdio::null());

        let mut server = server.spawn().unwrap();

        while std::net::TcpStream::connect("127.0.0.1:9148").is_err() {
            if server.try_wait().unwrap().is_some() {
                panic!("Site failed to start.");
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        write_stdin(&task, num);

        let mut client = match self.tool {
            Tool::Hyperfine => {
                let mut client = Command::new("hyperfine");
                client.args([
                    "--style", "none",
                    "--command-name", &name,
                    "--export-json", &out,
                    "--input", "urlc-tool/tmp/stdin.txt",
                    &format!("target/release/url-cleaner-site-client clean {}://127.0.0.1:9148", self.protocol.scheme())
                ]);
                client
            },
            Tool::Massif => {
                let mut client = Command::new("valgrind");
                client.args([
                    "--tool=massif",
                    &format!("--massif-out-file={out}"),
                    "target/release/url-cleaner-site-client",
                    "clean",
                    &format!("{}://127.0.0.1:9148", self.protocol.scheme())
                ]);
                client.stdin(File::open("urlc-tool/tmp/stdin.txt").unwrap());
                client
            },
            Tool::Callgrind => {
                let mut client = Command::new("valgrind");
                client.args([
                    "--tool=callgrind",
                    &format!("--callgrind-out-file={out}"),
                    "--separated-threads=yes",
                    "target/release/url-cleaner-site-client",
                    "clean",
                    &format!("{}://127.0.0.1:9148", self.protocol.scheme())
                ]);
                client.stdin(File::open("urlc-tool/tmp/stdin.txt").unwrap());
                client
            }
        };

        client.stdout(std::process::Stdio::null());
        client.stderr(std::process::Stdio::null());

        assert_eq!(client.spawn().unwrap().wait().unwrap().code(), Some(0));

        server.terminate();

        std::fs::remove_file("urlc-tool/tmp/stdin.txt").unwrap();

        out
    }
}
