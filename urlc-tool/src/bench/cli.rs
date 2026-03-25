//! CLI.

use super::prelude::*;

/// CLI.
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
    pub tool: ClientTool,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) -> String {
        let Self {name, task, num, tool} = self;

        let out_dir = format!("bench/cli/{tool}/{name}/{num}");
        let out = format!("{out_dir}/{tool}.out");

        write_stdin(&task, num);
        fresh_dir(&out_dir);

        let mut cmd = match tool {
            ClientTool::Hyperfine => {
                let mut cmd = Command::new("hyperfine");

                cmd.args([
                    "--style", "none",
                    "--input", STDIN,
                    "--export-json", &out,
                    "target/release/url-cleaner"
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

                cmd.arg("target/release/url-cleaner");

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
