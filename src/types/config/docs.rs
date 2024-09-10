//! Allows placing documentation right into a Config.

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[allow(unused_imports, reason = "Used in a doc comment.")]
use crate::types::*;
use crate::util::*;

/// In-config documentation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigDocs {
    /// The basic description of the config.
    #[serde(default, skip_serializing_if = "is_default")]
    pub description: Option<String>,
    /// The descriptions of the [`Params::flags`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: HashMap<String, String>,
    /// The descriptions of the [`Params::vars`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>,
    /// The descriptions of the environment variables used.
    #[serde(default, skip_serializing_if = "is_default")]
    pub environment_vars: HashMap<String, String>,
    /// The descriptions of the [`Params::sets`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub sets: HashMap<String, String>,
    /// The descriptions of the [`Params::lists`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub lists: HashMap<String, String>,
    /// The descriptions of the [`Params::maps`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub maps: HashMap<String, String>
}
