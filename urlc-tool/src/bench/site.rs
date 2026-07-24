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
    /// The protocol.
    #[arg(long)]
    pub protocol: Protocol,
    /// The tool.
    #[arg(long)]
    pub tool: ServerTool,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) -> String {
        let Self {name, task, num, protocol, tool} = self;

        let out_dir = format!("bench/site/{protocol}/{tool}/{name}/{num}");
        let out = format!("{out_dir}/{tool}.out");

        write_stdin(&task, num);
        fresh_dir(&out_dir);

        let mut cmd = match tool {
            ServerTool::Valgrind(tool) => {
                let mut cmd = Command::new("valgrind");

                cmd.arg(format!("--tool={tool}"));
                cmd.arg(format!("--{tool}-out-file={out}"));

                if matches!(tool, ValgrindTool::Callgrind) {
                    cmd.arg("--separate-threads=yes");
                }

                cmd.args(["target/release/url-cleaner-site", "run", "--port", "9148"]);

                if protocol.tls() {
                    cmd.args([
                        "--key", "keys/urlcs.key",
                        "--cert", "keys/urlcs.crt",
                    ]);
                }

                cmd
            }
        };

        cmd.stdout(std::process::Stdio::null());
        cmd.stderr(std::process::Stdio::null());

        assert_no_site();

        let mut site = TerminateOnDrop(cmd.spawn().unwrap());

        await_site(&mut site.0);

        let mut cmd = Command::new("target/release/url-cleaner-site-client");

        cmd.args(["clean", protocol.endpoint()]);

        cmd.stdin(File::open(STDIN).unwrap());
        cmd.stdout(std::process::Stdio::null());
        cmd.stderr(std::process::Stdio::null());

        assert_eq!(cmd.spawn().unwrap().wait().unwrap().code(), Some(0));

        out
    }
}
