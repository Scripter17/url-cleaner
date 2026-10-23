//! [`Hyperfine`].

use super::prelude::*;

/// Hyperfine.
#[derive(Debug, Clone, Copy)]
pub enum Hyperfine {
    /// Mean wall clock time.
    Time,
    /// Mean resident memory usage.
    Memory,
}

impl Hyperfine {
    /// Get a table entry.
    pub fn get_entry<P: AsRef<Path>>(self, path: P) -> String {
        match self {
            Self::Time   => format!   ("{:.1}", serde_json::from_str::<serde_json::Value>(&std::fs::read_to_string(path).unwrap()).unwrap()["results"][0]["summary"]["time_wall_clock"     ]["mean"].as_f64().unwrap() * 1000.0),
            Self::Memory => format_int(         serde_json::from_str::<serde_json::Value>(&std::fs::read_to_string(path).unwrap()).unwrap()["results"][0]["summary"]["memory_peak_resident"]["mean"].as_f64().unwrap() as u64  ),
        }
    }

    /// The name as title.
    pub fn title(self) -> &'static str {
        match self {
            Self::Time   => "Hyperfine Time"  ,
            Self::Memory => "Hyperfine Memory",
        }
    }

    /// The name as kebab case.
    pub fn kebab(self) -> &'static str {
        match self {
            Self::Time   => "hyperfine-time"  ,
            Self::Memory => "hyperfine-memory",
        }
    }

    /// The description.
    pub fn description(self) -> &'static str {
        match self {
            Self::Time   => "The average number of milliseconds.",
            Self::Memory => "The average peak resident memory usage.\n\nPlease note that, as far as I understand, the vast majority the base value (num=0) is shared memory, meaning it's either shared libraries that other programs can use without additional allocations or unused heap space that can be given to other programs as needed.\n\nI may be wrong about that though.",
        }
    }
}
