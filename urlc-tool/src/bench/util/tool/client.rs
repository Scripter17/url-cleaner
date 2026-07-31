//! Site CLIent.

use crate::prelude::*;

use super::*;

/// The tool to measure with.
#[derive(Debug, Clone, Copy)]
pub enum ClientTool {
    /// [`Hyperfine`].
    Hyperfine(Hyperfine),
    /// [`Valgrind`].
    Valgrind(Valgrind)
}

impl ClientTool {
    /// Get an entry.
    pub fn get_entry<P: AsRef<Path>>(self, path: P) -> String {
        match self {
            Self::Hyperfine(x) => x.get_entry(path),
            Self::Valgrind (x) => x.get_entry(path),
        }
    }
}

impl ValueEnum for ClientTool {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Hyperfine(Hyperfine),
            Self::Valgrind (Valgrind::Massif),
            Self::Valgrind (Valgrind::Callgrind),
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(self.to_string().into())
    }
}

impl std::fmt::Display for ClientTool {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hyperfine(x) => write!(formatter, "{x}"),
            Self::Valgrind (x) => write!(formatter, "{x}"),
        }
    }
}
