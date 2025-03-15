//! Scratchpad space for rules to store state in.

use std::collections::{HashSet, HashMap};

use serde::{Serialize, Deserialize};

use crate::util::*;

/// Mutable state that you can use to track data between rules outside of the URL.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobScratchpad {
    /// Boolean variables used to determine behavior.
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: HashSet<String>,
    /// String variables used to determine behavior.
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>
}
