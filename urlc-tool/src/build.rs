//! Build.

use super::prelude::*;

/// Build release with debug info and no warnings.
#[derive(Debug, Default, Parser)]
pub struct Args {
    /// Don't compile anything, just output binary paths.
    #[arg(long)]
    pub no_compile: bool,
    /// Build in debug mode.
    #[arg(long)]
    pub debug: bool,
    /// Binaries.
    #[arg(long, num_args = 1.., default_values = ["cli", "site", "site-client", "discord"])]
    pub bins: Vec<Bin>,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) -> Vec<&'static str> {
        if !self.no_compile {
            let mut cmd = Command::new("cargo");

            cmd.env("RUSTFLAGS", "-Awarnings");

            cmd.args(["build", "--config", "profile.release.strip=false", "--config", "profile.release.debug=2"]);

            if !self.debug {cmd.arg("-r");}

            for bin in &self.bins {
                cmd.args(["--bin", bin.file_name()]);
            }

            cmd.stdout(std::io::stderr());
            cmd.stderr(std::io::stderr());

            assert_eq!(cmd.spawn().unwrap().wait().unwrap().code(), Some(0));
        }

        self.bins.into_iter().map(if self.debug {Bin::debug_path} else {Bin::release_path}).collect()
    }
}
