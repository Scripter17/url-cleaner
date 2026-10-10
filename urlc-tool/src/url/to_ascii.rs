//! [`better_url::util::domain_host_to_ascii`].

use crate::prelude::*;

/// better_url::util::domain_host_to_ascii.
#[derive(Debug, Parser)]
pub struct Args {
    /// Be strict.
    #[arg(long)]
    pub strict: bool,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        for line in std::io::stdin().lines().map(Result::unwrap) {
            match better_url::util::domain_host_to_ascii(line, self.strict) {
                Ok((raw, ascii)) => println!("{}", ascii.map_or(raw, Into::into)),
                Err(_) => println!(),
            }
        }
    }
}

