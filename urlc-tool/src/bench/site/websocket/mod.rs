//! WebSocket.

use super::prelude::*;

pub mod hyperfine;
pub mod massif;
pub mod callgrind;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::super::prelude::*;
}

/// WebSocket.
#[allow(clippy::missing_docs_in_private_items, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
pub enum Args {
    Hyperfine(hyperfine::Args),
    Massif(massif::Args),
    Callgrind(callgrind::Args)
}

impl Args {
    /// Do the command.
    pub fn r#do(self) -> String {
        match self {
            Args::Hyperfine(args) => args.r#do(),
            Args::Massif   (args) => args.r#do(),
            Args::Callgrind(args) => args.r#do()
        }
    }
}
