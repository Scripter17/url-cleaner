//! Normalize.

use super::prelude::*;

use url::Url;

/// Parse each line of STDIN as a URL and print it.
///
/// STDOUT lines starting with - represent errors.
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

            match str::from_utf8(&buf) {
                Ok(x) => match Url::parse(x) {
                    Ok(url) => println!("{url}"),
                    Err(e) => println!("-{e:?}"),
                },
                Err(e) => println!("-{e:?}")
            }

            buf.clear();
        }
    }
}
