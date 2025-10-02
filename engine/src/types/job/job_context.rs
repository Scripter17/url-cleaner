//! The context of a [`Job`].

use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_with::*;

use crate::types::*;
use crate::util::*;

/// The context for a set of [`Task`]s.
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JobContext {
    /// The [`BetterHost`] of the "source" of the [`Job`].
    ///
    /// Used with [`TaskContext`] by the default cleaner and the userscript to allow for per-site optimizations and unmangling.
    #[serde(default, skip_serializing_if = "is_default")]
    pub source_host: Option<BetterHost<String>>,
    /// The vars.
    #[serde_as(as = "MapPreventDuplicates<_, _>")]
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>
}
