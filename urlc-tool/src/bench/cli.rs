//! CLI.

use super::prelude::*;

/// CLI.
#[derive(Debug, Parser)]
pub struct Args {
    /** The name.       **/ #[arg(long)] pub name       : String,
    /** The task.       **/ #[arg(long)] pub task       : String,
    /** The num.        **/ #[arg(long)] pub num        : u64,
    /** The ParamsDiff. **/ #[arg(long)] pub params_diff: Option<String>,
    /** The tool.       **/ #[arg(long)] pub tool       : ClientTool,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) -> String {
        let Self {name, task, num, params_diff, tool} = self;

        let out_dir = format!("bench/cli/{}/{name}/{num}", tool.kebab());
        let out = format!("{out_dir}/{}.out", tool.kebab());

        write_stdin(&task, num);
        if let Some(params_diff) = params_diff.as_ref() {
            write_params_diff(params_diff);
        }
        fresh_dir(&out_dir);

        let mut cmd = match tool {
            ClientTool::Hyperfine(Hyperfine) => {
                let mut cmd = Command::new("hyperfine");

                cmd.args([
                    "--style", "none",
                    "--input", STDIN,
                    "--export-json", &out,
                    match params_diff.is_some() {
                        true  => "target/release/url-cleaner --params-diff urlc-tool/tmp/bench/params_diff.json",
                        false => "target/release/url-cleaner"
                    }
                ]);

                cmd
            },
            ClientTool::Valgrind(tool) => {
                let mut cmd = Command::new("valgrind");

                cmd.arg(format!("--tool={}", tool.kebab()));
                cmd.arg(format!("--{}-out-file={out}", tool.kebab()));

                if matches!(tool, Valgrind::Callgrind) {
                    cmd.arg("--separate-threads=yes");
                }

                cmd.arg("target/release/url-cleaner");

                if params_diff.is_some() {
                    cmd.args(["--params-diff", PARAMS_DIFF]);
                }

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
