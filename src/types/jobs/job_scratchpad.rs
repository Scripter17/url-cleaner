//! Scratchpad space for rules to store state in.

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[allow(unused_imports, reason = "Used in a doc comment.")]
use crate::types::*;

/// Similar to [`Params`] but modifiable by [`Mapper`]s.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobScratchpad {
    /// String variables used to determine behavior.
    pub vars: HashMap<String, String>
}
