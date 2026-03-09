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
    pub tool: Tool,
}

/// The tool.
#[derive(Debug, Clone, Copy, ValueEnum, Deserialize)]
pub enum Tool {
    /// Hyperfine.
    Hyperfine,
    /// Massif.
    Massif,
    /// Callgrind.
    Callgrind,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) -> String {
        let Self {name, task, num, tool} = self;

        let out_dir = format!("urlc-tool/out/bench/cli/{tool:?}/{name}/{num}");

        let _ = std::fs::remove_dir_all(&out_dir);
        std::fs::create_dir_all(&out_dir).unwrap();

        let _stdin_handle = write_stdin(&task, num);

        let (out, mut cmd) = match tool {
            Tool::Hyperfine => {
                let out = format!("{out_dir}/hyperfine.json");

                let mut cmd = Command::new("hyperfine");

                cmd.args([
                    "--show-output",
                    "--input", STDIN,
                    "--export-json", &out,
                    "target/release/url-cleaner"
                ]);

                (out, cmd)
            },
            Tool::Massif => {
                let out = format!("{out_dir}/massif.out");

                let mut cmd = Command::new("valgrind");

                cmd.args(["--tool=massif"]);
                cmd.arg(format!("--massif-out-file={out}"));
                cmd.arg("target/release/url-cleaner");

                cmd.stdin(File::open(STDIN).unwrap());

                (out, cmd)
            },
            Tool::Callgrind => {
                let out = format!("{out_dir}/callgrind.out");

                let mut cmd = Command::new("valgrind");

                cmd.args(["--tool=callgrind", "--separate-threads=yes"]);
                cmd.arg(format!("--callgrind-out-file={out}"));
                cmd.arg("target/release/url-cleaner");

                cmd.stdin(File::open(STDIN).unwrap());

                (out, cmd)
            },
        };

        cmd.stdout(std::process::Stdio::null());
        cmd.stderr(new_file(format!("{out_dir}/stderr.txt")));

        assert_eq!(cmd.spawn().unwrap().wait().unwrap().code(), Some(0));

        out
    }
}
