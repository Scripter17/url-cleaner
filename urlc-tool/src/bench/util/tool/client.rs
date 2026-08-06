//! Site CLIent.

use super::prelude::*;

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

    /// The name as title.
    pub fn title(self) -> &'static str {
        match self {
            Self::Hyperfine(x) => x.title(),
            Self::Valgrind (x) => x.title(),
        }
    }

    /// The name as kebab case.
    pub fn kebab(self) -> &'static str {
        match self {
            Self::Hyperfine(x) => x.kebab(),
            Self::Valgrind (x) => x.kebab(),
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
        Some(self.kebab().into())
    }
}
