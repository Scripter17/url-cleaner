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
        let mut stdin = std::io::stdin().lock();
        let mut buf = Vec::new();

        while stdin.read_until(b'\n', &mut buf).unwrap() > 0 {
            if buf.ends_with(b"\n") {
                buf.pop();
                if buf.ends_with(b"\r") {
                    buf.pop();
                }
            }

            if buf.is_empty() {
                continue;
            }

            match (&*buf).make_task() {
                Ok(task) => if is_default(&task.context) {
                    println!("{}", task.url);
                } else {
                    println!("{}", serde_json::to_string(&task).unwrap());
                },
                Err(e) => println!("-{e:?}")
            }

            buf.clear();
        }
    }
}
