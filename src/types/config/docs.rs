//! Allows placing documentation right into a Config.

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::util::*;

/// In-config documentation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigDocs {
    /// The basic description of the config.
    #[serde(default, skip_serializing_if = "is_default")]
    pub description: Option<String>,
    /// The descriptions of the [`Config::flags`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: HashMap<String, String>,
    /// The descriptions of the [`Config::vars`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>,
    /// The descriptions of the [`Config::sets`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub sets: HashMap<String, String>,
    /// The descriptions of the [`Config::lists`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub lists: HashMap<String, String>,
    /// The descriptions of the [`Config::maps`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub maps: HashMap<String, String>
}
