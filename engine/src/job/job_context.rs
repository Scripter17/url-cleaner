//! [`JobContext`].

use std::collections::{HashSet, HashMap};

use serde::{Serialize, Deserialize};

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
