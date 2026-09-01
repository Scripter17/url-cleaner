//! Site CLIent.

use super::prelude::*;

/// Site CLIent.
#[derive(Debug, Parser)]
pub struct Args {
    /** The name.       **/ #[arg(long)] pub name       : String,
    /** The task.       **/ #[arg(long)] pub task       : String,
    /** The num.        **/ #[arg(long)] pub num        : u64,
    /** The ParamsDiff. **/ #[arg(long)] pub params_diff: Option<String>,
    /** The protocol.   **/ #[arg(long)] pub protocol   : Protocol,
    /** The tool.       **/ #[arg(long)] pub tool       : ClientTool,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) -> String {
        let Self {name, task, num, params_diff, protocol, tool} = self;

        let out_dir = format!("urlc-tool/out/bench/site-client/{}/{}/{name}/{num}", protocol.kebab(), tool.kebab());
        let out = format!("{out_dir}/{}.out", tool.kebab());

        write_stdin(&task, num);
        fresh_dir(&out_dir);

        let _site = start_site(protocol.is_tls());

        let mut cmd = match tool {
            ClientTool::Hyperfine(Hyperfine) => {
                let mut cmd = Command::new("hyperfine");

                cmd.args([
                    "--style", "none",
                    "--input", STDIN,
                    "--export-json", &out,
                    &match params_diff {
                        Some(params_diff) => format!("target/release/url-cleaner-site-client clean {} --params-diff '{params_diff}'", protocol.endpoint()),
                        None              => format!("target/release/url-cleaner-site-client clean {}"                              , protocol.endpoint()),
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

                cmd.args(["target/release/url-cleaner-site-client", "clean", protocol.endpoint()]);

                if let Some(params_diff) = params_diff {
                    cmd.args(["--params-diff", &params_diff]);
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
