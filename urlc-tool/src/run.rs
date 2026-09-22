//! Build.

use super::prelude::*;

/// Build release with debug info and no warnings.
#[derive(Debug, Parser)]
pub struct Args {
    /// The binary to run.
    pub bin: Bin,
    /// The arguments for the program.
    #[arg(last = true)]
    pub args: Vec<String>,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        crate::build::Args {
            bins: vec![self.bin]
        }.r#do();

        assert_eq!(Command::new(self.bin.release_path()).args(self.args).spawn().unwrap().wait().unwrap().code(), Some(0));
    }
}

