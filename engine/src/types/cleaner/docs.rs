//! Documentation.

use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_with::*;

#[expect(unused_imports, reason = "Used in docs.")]
use crate::types::*;
use crate::util::*;

/// Documentation stored inside of the [`Cleaner`].
///
/// Used for suitability tests to make sure I don't forget to document anything and also to generate the docs section of the README.
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct CleanerDocs {
    /// The name of the [`Cleaner`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub name: Option<String>,
    /// The description of the [`Cleaner`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub description: Option<Vec<String>>,
    /// The documentation of the flags in [`Params::flags`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: HashMap<String, String>,
    /// The documentation of the cars in [`Params::vars`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>,
    /// The documentation of the environment variables used.
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub environment_vars: HashMap<String, String>,
    /// The documentation of the sets in [`Params::sets`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub sets: HashMap<String, String>,
    /// The documentation of the lists in [`Params::lists`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub lists: HashMap<String, String>,
    /// The documentation of the maps in [`Params::maps`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub maps: HashMap<String, String>,
    /// The documentation of the named partitionings in [`Params::named_partitionings`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub named_partitionings: HashMap<String, String>,
    /// The documentation of the stuff used in [`JobContext`]s.
    #[serde(default, skip_serializing_if = "is_default")]
    pub job_context: JobContextDocs,
    /// The documentation of the stuff used in [`TaskContext`]s.
    #[serde(default, skip_serializing_if = "is_default")]
    pub task_context: TaskContextDocs
}

/// Documentation for the stuff used in [`JobContext`].
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub struct JobContextDocs {
    /// Documentation for the vars in [`JobContext::vars`]
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>
}

/// Documentation for the stuff used in [`TaskContext`].
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub struct TaskContextDocs {
    /// Documentation for the vars in [`TaskContext::vars`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>
}
