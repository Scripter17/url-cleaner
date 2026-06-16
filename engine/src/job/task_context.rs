//! [`TaskContext`].

use crate::prelude::*;

/// The context of a [`Task`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskContext {
    /// The flags to use.
    ///
    /// Defaults to an empty [`HashSet`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: HashSet<String>,
    /// The vars to use.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>
}

impl TaskContext {
    /// If [`Self::flags`] and [`Self::vars`] are empty.
    pub fn is_empty(&self) -> bool {
        self.flags.is_empty() && self.vars.is_empty()
    }
}
