//! [`Docs`].

use serde::{Serialize, Deserialize};
use serde_with::*;
use indexmap::IndexMap;

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
    /// The [`EnvironmentVarsDocs`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub environment_vars: EnvironmentVarsDocs
}

/// Documentation for a [`Cleaner`].
///
/// Mainly used in [`Cleaner::docs`].
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct ParamsDocs {
    /// The documentation of the flags in [`Params::flags`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: IndexMap<String, String>,
    /// The documentation of the cars in [`Params::vars`].
    #[serde(default, skip_serializing_if = "is_default")]
    #[suitable(assert = "assert_required_vars_exist")]
    pub vars: IndexMap<String, VarDoc>,
    /// The documentation of the sets in [`Params::sets`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub sets: IndexMap<String, String>,
    /// The documentation of the lists in [`Params::lists`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub lists: IndexMap<String, String>,
    /// The documentation of the maps in [`Params::maps`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub maps: IndexMap<String, String>,
    /// The documentation of the named partitionings in [`Params::partitionings`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub partitionings: IndexMap<String, String>
}

/// Documentation for the stuff used in [`JobContext`].
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct JobContextDocs {
    /// Documentation for the flags in [`JobContext::flags`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: IndexMap<String, String>,
    /// Documentation for the vars in [`JobContext::vars`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: IndexMap<String, String>
}

/// Documentation for the stuff used in [`TaskContext`].
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct TaskContextDocs {
    /// Documentation for the flags in [`TaskContext::flags`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: IndexMap<String, String>,
    /// Documentation for the vars in [`TaskContext::vars`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: IndexMap<String, String>
}

/// Documentation for the environment variables.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(transparent)]
pub struct EnvironmentVarsDocs(pub IndexMap<String, String>);

/// Documentation for a var.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    #[serde(default, skip_serializing_if = "is_default")]
    pub variants: Option<IndexMap<String, String>>
}

/// Assert all required vars exist.
fn assert_required_vars_exist(x: &IndexMap<String, VarDoc>, cleaner: &Cleaner<'_>) {
    for (name, doc) in x {
        match (doc.required, cleaner.params.vars.get(name), &doc.variants) {
            (false, None       , _             ) => {},
            (false, Some(_)    , None          ) => {},
            (true , None       , _             ) => panic!("Missing var: {name}"),
            (true , Some(_)    , None          ) => {},
            (_    , Some(value), Some(variants)) => assert!(variants.contains_key(value)),
        }
    }
}
