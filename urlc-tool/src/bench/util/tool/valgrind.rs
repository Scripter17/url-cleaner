//! Valgrind.

use crate::prelude::*;

/// The valgrind tool to use.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ValgrindTool {
    /// Massif.
    Massif,
    /// Callgrind.
    Callgrind
}

impl ValgrindTool {
    /// Get a table entry.
    pub fn get_entry<P: AsRef<Path>>(self, path: P) -> String {
        match self {
            Self::Massif => {
                let mut ret = 0u64;
                for line in BufReader::new(File::open(path).unwrap()).lines() {
                    if let Some(x) = line.unwrap().strip_prefix("mem_heap_B=") {
                        ret = ret.max(x.parse().unwrap());
                    }
                }
                ret.to_formatted_string(&Locale::en)
            },
            Self::Callgrind => "...".into(),
        }
    }
}

impl std::fmt::Display for ValgrindTool {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Massif    => write!(formatter, "massif"),
            Self::Callgrind => write!(formatter, "callgrind"),
        }
    }
}
