//! Build.

use super::prelude::*;

/// Build release with debug info and no warnings.
#[derive(Debug, Parser)]
pub struct Args {
    /// The binary to run.
    pub bin: Bin,
    /// Don't compile.
    #[arg(long)]
    pub no_compile: bool,
    /// Run under a wrapper.
    #[arg(long)]
    pub wrapper: Option<Wrapper>,
    /// The arguments for the program.
    #[arg(last = true)]
    pub args: Vec<String>,
}

/// The wrapper to use.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Wrapper {
    /// Callgrind.
    Callgrind,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        if !self.no_compile {
            crate::build::Args {
                bins: vec![self.bin]
            }.r#do();
        }

        let mut command = match self.wrapper {
            Some(Wrapper::Callgrind) => {let mut command = Command::new("valgrind"             ); command.arg("--tool=callgrind").arg(self.bin.release_path()).args(self.args); command},
            None                     => {let mut command = Command::new(self.bin.release_path()); command.args(self.args)                                                     ; command},
        };

        assert_eq!(command.spawn().unwrap().wait().unwrap().code(), Some(0));
    }
}

