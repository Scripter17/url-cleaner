//! [`ParamsDocs`].

use crate::prelude::*;

use indexmap::IndexMap;

/// Documentation for a [`Cleaner`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct ParamsDocs {
    /// [`Params::flags`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: IndexMap<String, String>,
    /// [`Params::vars`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: IndexMap<String, VarDoc>,
    /// [`Params::sets`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub sets: IndexMap<String, String>,
    /// [`Params::lists`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub lists: IndexMap<String, String>,
    /// [`Params::maps`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub maps: IndexMap<String, String>,
    /// [`Params::partitionings`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub partitionings: IndexMap<String, String>,
}
