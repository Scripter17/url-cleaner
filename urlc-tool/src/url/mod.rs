//! URls.

use crate::prelude::*;

mod extract;
mod to_unicode;
mod to_ascii;

/// URLs.
#[expect(missing_docs, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
pub enum Args {
    Extract(extract::Args),
    ToUnicode(to_unicode::Args),
    ToAscii(to_ascii::Args),
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Self::Extract  (args) => args.r#do(),
            Self::ToUnicode(args) => args.r#do(),
            Self::ToAscii  (args) => args.r#do(),
        }
    }
}
