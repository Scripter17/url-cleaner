//! [`TaskContext`].

use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_with::*;

use crate::prelude::*;

/// The context for a [`TaskConfig`], such as a link's text.
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskContext {
    /// The vars.
    #[serde_as(as = "MapPreventDuplicates<_, _>")]
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>
}
