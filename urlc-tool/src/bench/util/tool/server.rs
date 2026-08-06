//! Site tools.

use super::prelude::*;

/// The tool to measure with.
#[derive(Debug, Clone, Copy)]
pub enum ServerTool {
    /// [`Valgrind`].
    Valgrind(Valgrind)
}

impl ServerTool {
    /// Get a table entry.
    pub fn get_entry<P: AsRef<Path>>(self, path: P) -> String {
        match self {
            Self::Valgrind(x) => x.get_entry(path),
        }
    }

    /// The name as title.
    pub fn title(self) -> &'static str {
        match self {
            Self::Valgrind(x) => x.title(),
        }
    }

    /// The name as kebab case.
    pub fn kebab(self) -> &'static str {
        match self {
            Self::Valgrind(x) => x.kebab(),
        }
    }
}

impl ValueEnum for ServerTool {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Valgrind(Valgrind::Massif),
            Self::Valgrind(Valgrind::Callgrind),
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(self.kebab().into())
    }
}
