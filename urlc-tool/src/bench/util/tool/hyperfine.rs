//! [`Hyperfine`].

use crate::prelude::*;

/// Hyperfine.
#[derive(Debug, Clone, Copy)]
pub struct Hyperfine;

impl Hyperfine {
    /// Get a table entry.
    pub fn get_entry<P: AsRef<Path>>(self, path: P) -> String {
        format!("{:.1}", serde_json::from_str::<serde_json::Value>(&std::fs::read_to_string(path).unwrap()).unwrap()["results"][0]["mean"].as_f64().unwrap() * 1000.0)
    }
}

impl std::fmt::Display for Hyperfine {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "hyperfine")
    }
}
