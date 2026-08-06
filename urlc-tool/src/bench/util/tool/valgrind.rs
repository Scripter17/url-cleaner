//! Valgrind.

use super::prelude::*;

/// The valgrind tool to use.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Valgrind {
    /// Massif.
    Massif,
    /// Callgrind.
    Callgrind
}

impl Valgrind {
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
                format_int(ret)
            },
            Self::Callgrind => "...".into(),
        }
    }

    /// The name as title.
    pub fn title(self) -> &'static str {
        match self {
            Self::Massif    => "Massif"   ,
            Self::Callgrind => "Callgrind",
        }
    }

    /// The name as kebab case.
    pub fn kebab(self) -> &'static str {
        match self {
            Self::Massif    => "massif"   ,
            Self::Callgrind => "callgrind",
        }
    }
}
