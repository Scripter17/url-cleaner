//! Compile.

use super::prelude::*;

/// Compile.
#[derive(Debug, Parser)]
pub struct Args {
    /// The frontends to compile.
    #[command(flatten)]
    pub frontends: Frontends
}

/// The frontends.
#[derive(Debug, Parser)]
#[group(required = true)]
pub struct Frontends {
    /// CLI.
    #[arg(long)]
    pub cli: bool,
    /// Site.
    #[arg(long)]
    pub site: bool
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let mut cmd = Command::new("cargo");

        cmd.args(["+stable", "build", "-r", "--config", "profile.release.strip=false", "--config", "profile.release.debug=2"]);
        cmd.stdout(std::io::stderr());
        cmd.stderr(std::io::stderr());

        if self.frontends.cli  {cmd.args(["--bin", "url-cleaner"     ]);}
        if self.frontends.site {cmd.args(["--bin", "url-cleaner-site"]);}

        assert_eq!(cmd.spawn().unwrap().wait().unwrap().code(), Some(0));
    }
}
