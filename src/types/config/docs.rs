//! Documentation.

use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_with::*;

use crate::types::*;
use crate::util::*;

/// Documentation stored inside of the [`Config`].
///
/// Used for suitability tests to make sure I don't forget to document anything and also to generate the docs section of the README.
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub struct ConfigDocs {
    /// The title of the [`Config`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub title: Option<String>,
    /// The description of the [`Config`].
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
    /// The documentation of the stuff used in [`JobsContext`]s.
    #[serde(default, skip_serializing_if = "is_default")]
    pub jobs_context: JobsContextDocs,
    /// The documentation of the stuff used in [`JobContext`]s.
    #[serde(default, skip_serializing_if = "is_default")]
    pub job_context: JobContextDocs
}

/// Documentation for the stuff used in [`JobsContext`].
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub struct JobsContextDocs {
    /// Documentation for the vars in [`JobsContext::vars`]
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>
}

/// Documentation for the stuff used in [`JobContext`].
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub struct JobContextDocs {
    /// Documentation for the vars in [`JobContext::vars`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>
}
