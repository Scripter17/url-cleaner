//! [`TaskContextDocs`].

use crate::prelude::*;

use indexmap::IndexMap;

/// Documentation for a [`Cleaner`]'s [`TaskContext`] format.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct TaskContextDocs {
    /// [`TaskContext::flags`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: IndexMap<String, String>,
    /// [`TaskContext::vars`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: IndexMap<String, VarDoc>,
}
