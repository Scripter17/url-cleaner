use crate::prelude::*;

mod whole;
mod host;

#[derive(Debug, Parser)]
pub enum Args {
    Whole(whole::Args),
    Host(host::Args),
}

impl Args {
    pub fn r#do(self) {
        match self {
            Self::Whole(args) => args.r#do(),
            Self::Host (args) => args.r#do(),
        }
    }
}
