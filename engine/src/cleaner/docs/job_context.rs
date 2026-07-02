//! [`JobContextDocs`].

use crate::prelude::*;

use indexmap::IndexMap;

/// Documentation for a [`Cleaner`]'s [`JobContext`] format.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct JobContextDocs {
    /// [`JobContext::flags`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: IndexMap<String, String>,
    /// [`JobContext::vars`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: IndexMap<String, VarDoc>,
}
