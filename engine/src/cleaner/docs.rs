//! [`Docs`].

use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_with::*;

use crate::prelude::*;

/// Documentation for a [`Cleaner`].
///
/// Mainly used in [`Cleaner::docs`].
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct Docs {
    /// The name of the [`Cleaner`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub name: Option<String>,
    /// The description of the [`Cleaner`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub description: Option<Vec<String>>,
    /// The [`ParamsDocs`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub params: ParamsDocs,
    /// The [`JobContextDocs`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub job_context: JobContextDocs,
    /// The [`TaskContextDocs`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub task_context: TaskContextDocs,
    /// The documentation of the environment variables used.
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub environment_vars: HashMap<String, String>
}

/// Documentation for a [`Cleaner`].
///
/// Mainly used in [`Cleaner::docs`].
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct ParamsDocs {
    /// The documentation of the flags in [`Params::flags`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: HashMap<String, String>,
    /// The documentation of the cars in [`Params::vars`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    #[suitable(assert = "assert_required_vars_exist")]
    pub vars: HashMap<String, VarDoc>,
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
    /// The documentation of the named partitionings in [`Params::partitionings`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub partitionings: HashMap<String, String>
}

/// Documentation for the stuff used in [`JobContext`].
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct JobContextDocs {
    /// Documentation for the flags in [`JobContext::flags`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: HashMap<String, String>,
    /// Documentation for the vars in [`JobContext::vars`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>
}

/// Documentation for the stuff used in [`TaskContext`].
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct TaskContextDocs {
    /// Documentation for the flags in [`TaskContext::flags`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: HashMap<String, String>,
    /// Documentation for the vars in [`TaskContext::vars`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>
}

/// Documentation for a var.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct VarDoc {
    /// The description.
    pub desc: String,
    /// If it's required.
    pub required: bool,
    /// If [`Some`], what it means for it to be unset.
    ///
    /// Defaults to [`None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub unset: Option<String>,
    /// If [`Some`], the map of valid values to their descriptions.
    ///
    /// Defaults to [`None`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub variants: Option<HashMap<String, String>>
}

fn assert_required_vars_exist(x: &HashMap<String, VarDoc>, cleaner: &Cleaner<'_>) {
    for (name, VarDoc {required, ..}) in x {
        assert!(cleaner.params.vars.get(name).is_some() || !required, "Required var is missing: {name:?}.");
    }
}
