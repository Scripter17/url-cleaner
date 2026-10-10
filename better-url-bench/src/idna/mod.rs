//! IDNA.

use clap::Parser;

mod ascii;
mod unicode;
mod bidi_detail;
mod uts46;

/// IDNA.
#[derive(Debug, Parser)]
#[expect(missing_docs, reason = "Makes clap inherit the docs.")]
pub enum Args {
    Ascii     (ascii      ::Args),
    Unicode   (unicode    ::Args),
    BidiDetail(bidi_detail::Args),
    Uts46     (uts46      ::Args),
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Self::Ascii     (args) => args.r#do(),
            Self::Unicode   (args) => args.r#do(),
            Self::BidiDetail(args) => args.r#do(),
            Self::Uts46     (args) => args.r#do(),
        }
    }
}
