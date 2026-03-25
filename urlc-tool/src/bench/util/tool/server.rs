//! Site tools.

use crate::prelude::*;
use super::valgrind::*;

/// The tool to measure with.
#[derive(Debug, Clone, Copy)]
pub enum ServerTool {
    /// [`ValgrindTool`].
    Valgrind(ValgrindTool)
}

impl ServerTool {
    /// Get a table entry.
    pub fn get_entry<P: AsRef<Path>>(self, path: P) -> String {
        match self {
            Self::Valgrind(x) => x.get_entry(path),
        }
    }
}

impl ValueEnum for ServerTool {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Valgrind(ValgrindTool::Massif),
            Self::Valgrind(ValgrindTool::Callgrind),
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(self.to_string().into())
    }
}

impl std::fmt::Display for ServerTool {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Valgrind(tool) => write!(formatter, "{tool}"),
        }
    }
}
