//! CLI.

pub use super::prelude::*;

/// CLI.
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
    pub tool: Tool
}

impl Args {
    /// Do the command.
    pub fn r#do(self) -> String {
        let name = &self.name;
        let num = self.job_config.num;

        let out = match self.tool {
            Tool::Hyperfine => format!("urlc-tool/out/bench/cli/hyperfine/{name}/hyperfine.out-cli-{name}-{num}.json"),
            Tool::Massif    => format!("urlc-tool/out/bench/cli/massif/{name}/massif.out-cli-{name}-{num}"),
            Tool::Callgrind => format!("urlc-tool/out/bench/cli/callgrind/{name}/{num}/callgrind.out-cli-{name}-{num}")
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
            Tool::Hyperfine => {
                let mut cmd = Command::new("hyperfine");

                cmd.args([
                    "--style", "none",
                    "--command-name", &self.name,
                    "--export-json", &out,
                    "--input", "urlc-tool/tmp/stdin.txt",
                    "target/release/url-cleaner"
                ]);

                cmd
            },
            tool @ (Tool::Massif | Tool::Callgrind) => {
                let mut cmd = Command::new("valgrind");

                cmd.args([
                    &format!("--tool={tool}"),
                    &format!("--{tool}-out-file={out}"),
                ]);

                if tool == Tool::Callgrind {
                    cmd.arg("--separate-threads=yes");
                }

                cmd.arg("target/release/url-cleaner");

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
