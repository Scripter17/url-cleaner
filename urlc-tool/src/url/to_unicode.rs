//! [`better_url::util::domain_host_to_unicode`].

use crate::prelude::*;

/// better_url::util::domain_host_to_unicode.
#[derive(Debug, Parser)]
pub struct Args {
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        for line in std::io::stdin().lines().map(Result::unwrap) {
            match better_url::util::domain_host_to_unicode(line) {
                Ok((raw, unicode, _)) | Err((raw, unicode)) => println!("{}", unicode.map_or(raw, Into::into)),
            }
        }
    }
}
