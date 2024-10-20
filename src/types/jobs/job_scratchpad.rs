//! Scratchpad space for rules to store state in.

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[allow(unused_imports, reason = "Used in a doc comment.")]
use crate::types::*;
use crate::util::*;

/// Mutable state that you can use to track data between rules outside of the URL.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobScratchpad {
    /// String variables used to determine behavior.
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>
}
