//! Query.

use clap::Parser;

mod entire;
mod segment;
mod name;
mod value;

/// Query.
#[expect(clippy::missing_docs_in_private_items, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
pub enum Args {
    Entire (entire ::Args),
    Segment(segment::Args),
    Name   (name   ::Args),
    Value  (value  ::Args),
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Self::Entire (args) => args.r#do(),
            Self::Segment(args) => args.r#do(),
            Self::Name   (args) => args.r#do(),
            Self::Value  (args) => args.r#do(),
        }
    }
}
