//! Run.

use super::prelude::*;

/// Compile and run a binary.
#[derive(Debug, Parser)]
pub struct Args {
    /// The bin to run.
    pub bin: Bin,
    /// No compile.
    #[arg(long)]
    pub no_compile: bool,
    /// Debug.
    #[arg(long)]
    pub debug: bool,
    /// The args to give to the bin.
    #[arg(last = true)]
    pub extra: Vec<String>
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let bin = crate::build::Args {
            bins: vec![self.bin],
            no_compile: self.no_compile,
            debug: self.debug
        }.r#do().pop().unwrap();

        assert_eq!(Command::new(bin).args(self.extra).spawn().unwrap().wait().unwrap().code(), Some(0));
    }
}
