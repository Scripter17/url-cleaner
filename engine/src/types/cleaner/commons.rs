//! Frequently used snippets that can be called like functions.

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::types::*;
use crate::util::*;

/// Common snippets of various tools that can be invoked like functions.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct Commons {
    /// Common [`Condition`]s.
    #[serde(default, skip_serializing_if = "is_default")]
    pub conditions: HashMap<String, Condition>,
    /// Common [`Action`]s.
    #[serde(default, skip_serializing_if = "is_default")]
    pub actions: HashMap<String, Action>,
    /// Common [`StringSource`]s.
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_sources: HashMap<String, StringSource>,
    /// Common [`StringModification`]s.
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_modifications: HashMap<String, StringModification>,
    /// Common [`StringMatcher`]s.
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_matchers: HashMap<String, StringMatcher>
}
