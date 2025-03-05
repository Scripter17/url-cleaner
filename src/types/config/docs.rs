//! Allows placing documentation right into a Config.

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::types::*;
use crate::util::*;

/// In-config documentation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigDocs {
    /// The basic description of the config.
    #[serde(default, skip_serializing_if = "is_default")]
    pub description: Option<Vec<String>>,
    /// The docs for [`Params::flags`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: HashMap<String, String>,
    /// The docs for [`Params::vars`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>,
    /// The docs for environment variables used.
    #[serde(default, skip_serializing_if = "is_default")]
    pub environment_vars: HashMap<String, String>,
    /// The docs for [`Params::sets`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub sets: HashMap<String, String>,
    /// The docs for [`Params::lists`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub lists: HashMap<String, String>,
    /// The docs for [`Params::maps`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub maps: HashMap<String, String>,
    /// The docs for [`JobContext`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub job_context: JobContextDocs,
    /// The docs for [`JobsContext`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub jobs_context: JobsContextDocs
}

/// The docs for [`JobContext`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobContextDocs {
    /// The docs for [`JobContext::vars`]
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>
}

/// The docs for [`JobContext`]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobsContextDocs {
    /// The docs for [`JobsContext::vars`]
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>
}
