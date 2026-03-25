//! Site CLIent.

use crate::prelude::*;
use super::valgrind::*;

/// The tool to measure with.
#[derive(Debug, Clone, Copy)]
pub enum ClientTool {
    /// Hyperfine.
    Hyperfine,
    /// [`ValgrindTool`].
    Valgrind(ValgrindTool)
}

impl ClientTool {
    /// Get an entry.
    pub fn get_entry<P: AsRef<Path>>(self, path: P) -> String {
        match self {
            Self::Hyperfine => format!("{:.1}", serde_json::from_str::<serde_json::Value>(&std::fs::read_to_string(path).unwrap()).unwrap()["results"][0]["mean"].as_f64().unwrap() * 1000.0),
            Self::Valgrind(x) => x.get_entry(path),
        }
    }
}

impl ValueEnum for ClientTool {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Hyperfine,
            Self::Valgrind(ValgrindTool::Massif),
            Self::Valgrind(ValgrindTool::Callgrind),
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(self.to_string().into())
    }
}

impl std::fmt::Display for ClientTool {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hyperfine => write!(formatter, "hyperfine"),
            Self::Valgrind(tool) => write!(formatter, "{tool}"),
        }
    }
}
