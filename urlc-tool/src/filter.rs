//! Filter.

use super::prelude::*;

use std::io::pipe;
use std::io::{BufRead, BufReader};

/// Filter failing tasks.
///
/// Reads lines from STDIN.
///
/// Prints passing lines to STDOUT.
///
/// Prints failing lines to STDERR.
#[derive(Debug, Parser)]
pub struct Args {
    /// Don't compile CLI.
    #[arg(long)]
    pub no_compile: bool,
    /// Compile in debug mode.
    #[arg(long)]
    pub debug: bool,
    /// Extra arguments to give URL Cleaner CLI.
    #[arg(last = true)]
    pub last: Vec<String>
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let bin = crate::build::Args {
            bins: vec![Bin::Cli],
            no_compile: self.no_compile,
            debug: self.debug
        }.r#do()[0];

        let (in_read , mut in_write ) = pipe().unwrap();
        let (out_read,     out_write) = pipe().unwrap();

        let mut out_read = BufReader::new(out_read);

        let _child = TerminateOnDrop(Command::new(bin)
            .args(self.last)
            .stdin(in_read)
            .stdout(out_write)
            .spawn().unwrap());

        let mut task   = Vec::new();
        let mut result = Vec::new();

        let mut stdin  = std::io::stdin ().lock();
        let mut stdout = std::io::stdout().lock();
        let mut stderr = std::io::stderr().lock();

        while stdin.read_until(b'\n', &mut task).unwrap() > 0 {
            in_write.write_all(&task).unwrap();
            in_write.flush().unwrap();

            if task.ends_with(b"\n") {
                task.pop();
                if task.ends_with(b"\r") {
                    task.pop();
                }
            }
            task.push(b'\n');

            assert_ne!(out_read.read_until(b'\n', &mut result).unwrap(), 0);

            if result.starts_with(b"-") {
                stderr.write_all(&task).unwrap();
            } else {
                stdout.write_all(&task).unwrap();
            }

            task.clear();
            result.clear();
        }
    }
}
