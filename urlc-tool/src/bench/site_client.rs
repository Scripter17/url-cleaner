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
    /// The protocol.
    #[arg(long)]
    pub protocol: Protocol,
    /// The tool.
    #[arg(long)]
    pub tool: ClientTool,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) -> String {
        let Self {name, task, num, protocol, tool} = self;

        let out_dir = format!("bench/site/{protocol}/{tool}/{name}/{num}");
        let out = format!("{out_dir}/{tool}.out");

        write_stdin(&task, num);
        fresh_dir(&out_dir);

        let mut cmd = Command::new("target/release/url-cleaner-site");

        cmd.args(["--port", "9148"]);

        if protocol.tls() {
            cmd.args([
                "--key", "urlc-tool/src/bench/urlcs-bench.key",
                "--cert", "urlc-tool/src/bench/urlcs-bench.crt",
            ]);
        }

        cmd.stdout(std::process::Stdio::null());
        cmd.stderr(std::process::Stdio::null());

        assert_no_site();

        let mut site = TerminateOnDrop(cmd.spawn().unwrap());

        await_site(&mut site.0);

        let mut cmd = match tool {
            ClientTool::Hyperfine => {
                let mut cmd = Command::new("hyperfine");

                cmd.args([
                    "--style", "none",
                    "--input", STDIN,
                    "--export-json", &out,
                    &format!("target/release/url-cleaner-site-client clean {}", protocol.endpoint())
                ]);

                cmd
            },
            ClientTool::Valgrind(tool) => {
                let mut cmd = Command::new("valgrind");

                cmd.arg(format!("--tool={tool}"));
                cmd.arg(format!("--{tool}-out-file={out}"));

                if matches!(tool, ValgrindTool::Callgrind) {
                    cmd.arg("--separate-threads=yes");
                }

                cmd.args(["target/release/url-cleaner-site-client", "clean", protocol.endpoint()]);

                cmd.stdin(File::open(STDIN).unwrap());

                cmd
            },
        };

        cmd.stdout(std::process::Stdio::null());
        cmd.stderr(std::process::Stdio::null());

        assert_eq!(cmd.spawn().unwrap().wait().unwrap().code(), Some(0));

        out
    }
}
