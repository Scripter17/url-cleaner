//! Build.

use super::prelude::*;

/// Build release with debug info and no warnings.
#[derive(Debug, Default, Parser)]
pub struct Args {
    /// The binaries to build.
    #[arg(num_args = 1.., default_values = ["cli", "site", "site-client", "discord"])]
    pub bins: Vec<Bin>,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let mut cmd = Command::new("cargo");

        cmd.env("RUSTFLAGS", "-Awarnings");

        cmd.args(["build", "-r", "--config", "profile.release.strip=false", "--config", "profile.release.debug=2"]);

        for bin in self.bins {
            cmd.args(["--bin", bin.file_name()]);
        }

        assert_eq!(cmd.spawn().unwrap().wait().unwrap().code(), Some(0));
    }
}
