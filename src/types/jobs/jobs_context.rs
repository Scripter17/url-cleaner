//! The context of an entire [`Jobs`].

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[expect(unused_imports, reason = "Used in a doc comment.")]
use crate::types::*;
use crate::util::*;

/// The context of an entire [`Jobs`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobsContext {
    /// String variables.
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>
}
