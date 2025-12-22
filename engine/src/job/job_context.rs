//! [`JobContext`].

use std::collections::{HashSet, HashMap};
use std::io;
use std::path::Path;
use std::fs::read_to_string;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::prelude::*;

/// The context of a [`Job`].
///
/// Sometimes websites have speicifc behavior that applies to all links on them, such as adding their own tracking parameters.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JobContext {
    /// The host of the page the tasks come from.
    #[serde(default, skip_serializing_if = "is_default")]
    pub source_host: Option<BetterHost<String>>,
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

impl JobContext {
    /// Load [`Self`] from a JSON file.
    /// # Errors
    #[doc = edoc!(callerr(std::fs::read_to_string), callerr(serde_json::from_str))]
    pub fn load_from_file<T: AsRef<Path>>(path: T) -> Result<JobContext, GetJobContextError> {
        serde_json::from_str(&read_to_string(path)?).map_err(Into::into)
    }
}

/// The enum of errors that can happen when loading a [`JobContext`].
#[derive(Debug, Error)]
pub enum GetJobContextError {
    /// Returned when loading a [`JobContext`] fails.
    #[error(transparent)]
    CantLoadJobContext(#[from] io::Error),
    /// Returned when deserializing a [`JobContext`] fails.
    #[error(transparent)]
    CantParseJobContext(#[from] serde_json::Error),
}
