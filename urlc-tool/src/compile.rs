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
    pub site: bool,
    /// Site WebSocket Client.
    #[arg(long)]
    pub site_ws_client: bool,
    /// Discord.
    #[arg(long)]
    pub discord: bool
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let mut cmd = Command::new("cargo");

        cmd.args(["build", "--config", "profile.release.strip=false", "--config", "profile.release.debug=2"]);
        if !DEBUG.get().unwrap() {cmd.arg("-r");}
        cmd.stdout(std::io::stderr());
        cmd.stderr(std::io::stderr());

        if self.frontends.cli            {cmd.args(["--bin", "url-cleaner"               ]);}
        if self.frontends.site           {cmd.args(["--bin", "url-cleaner-site"          ]);}
        if self.frontends.site_ws_client {cmd.args(["--bin", "url-cleaner-site-ws-client"]);}
        if self.frontends.discord        {cmd.args(["--bin", "url-cleaner-discord-app"   ]);}

        assert_eq!(cmd.spawn().unwrap().wait().unwrap().code(), Some(0));
    }
}
