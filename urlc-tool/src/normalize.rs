//! Normalize.

use super::prelude::*;

use url::Url;

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

            match str::from_utf8(&buf).map(Url::parse) {
                Ok(Ok(url)) => println!("{url}"),
                Ok(Err(e))  => println!("-{e:?}"),
                Err(e)      => println!("-{e:?}")
            }

            buf.clear();
        }
    }
}
