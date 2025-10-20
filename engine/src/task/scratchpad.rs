//! [`Scratchpad`].

use std::collections::{HashSet, HashMap};

use serde::{Serialize, Deserialize};
use serde_with::*;

use crate::prelude::*;

/// A scratchpad to allow storing data during a cleaning.
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scratchpad {
    /// The flags.
    #[serde_as(as = "SetPreventDuplicates<_>")]
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: HashSet<String>,
    /// The vars.
    #[serde_as(as = "MapPreventDuplicates<_, _>")]
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>
}
