//! CLI.

use super::prelude::*;

/// CLI.
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
    pub tool: Tool
}

/// The tools to benchmark with.
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

        write_stdin(&task, num);

        let mut cmd = match self.tool {
            Tool::Hyperfine => {
                let mut cmd = Command::new("hyperfine");
                cmd.args([
                    "--style", "none",
                    "--command-name", &name,
                    "--export-json", &out,
                    "--input", "urlc-tool/tmp/stdin.txt",
                    "target/release/url-cleaner"
                ]);
                cmd
            },
            Tool::Massif => {
                let mut cmd = Command::new("valgrind");
                cmd.args([
                    "--tool=massif",
                    &format!("--massif-out-file={out}"),
                    "target/release/url-cleaner"
                ]);
                cmd.stdin(File::open("urlc-tool/tmp/stdin.txt").unwrap());
                cmd
            },
            Tool::Callgrind => {
                let mut cmd = Command::new("valgrind");
                cmd.args([
                    "--tool=callgrind",
                    &format!("--callgrind-out-file={out}"),
                    "--separate-threads=yes",
                    "target/release/url-cleaner"
                ]);
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
