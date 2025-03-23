//! The context of a [`JobsSource`].

use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_with::*;

#[expect(unused_imports, reason = "Used in a doc comment.")]
use crate::types::*;
use crate::util::*;

/// The context for a set of [`Job`]s.
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobsContext {
    /// The vars.
    #[serde_as(as = "MapPreventDuplicates<_, _>")]
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>
}
