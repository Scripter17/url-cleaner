//! Normalize.

use super::prelude::*;

/// Parse each line of STDIN as a URL and print it.
///
/// Output lines starting with - represent errors.
#[derive(Debug, Parser)]
pub struct Args {}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        for line in std::io::stdin().lock().lines().map(Result::unwrap) {
            if line.is_empty() {
                continue;
            }

            match Task::new(&*line) {
                Ok (task) => println!("{task}"),
                Err(e   ) => println!("-{e:?}"),
            }
        }
    }
}
