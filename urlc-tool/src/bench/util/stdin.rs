//! STDIN stuff.

use crate::prelude::*;

/// A [`OnceLock`] of a [`StdinDeleter`].
static STDIN_DELETER: OnceLock<StdinDeleter> = OnceLock::new();

/// The path used to store STDIN.
pub const STDIN: &str = "urlc-tool/tmp/bench/stdin.txt";

/// Write the job to `urlc-tool/tmp/stdin.txt`.
pub fn write_stdin(task: &str, num: u64) {
    let mut file = new_file(STDIN);

    for _ in 0..num {
        writeln!(file, "{task}").unwrap();
    }

    let _ = STDIN_DELETER.get_or_init(|| StdinDeleter);
}

/// Deletes [`STDIN`] on [`std::ops::Drop`].
#[derive(Debug)]
pub struct StdinDeleter;

impl std::ops::Drop for StdinDeleter {
    fn drop(&mut self) {
        std::fs::remove_file(STDIN).unwrap();
    }
}
