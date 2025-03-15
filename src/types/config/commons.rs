//! Basically functions. Surprisingly useful.

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::types::*;
use crate::util::*;

/// Container for various things that are used in multiple spots.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub struct Commons {
    /// [`Rule`]s that are used in multiple spots.
    #[serde(default, skip_serializing_if = "is_default")]
    pub rules: HashMap<String, Rule>,
    /// [`Condition`]s that are used in multiple spots.
    #[serde(default, skip_serializing_if = "is_default")]
    pub conditions: HashMap<String, Condition>,
    /// [`Mapper`]s that are used in multiple spots.
    #[serde(default, skip_serializing_if = "is_default")]
    pub mappers: HashMap<String, Mapper>,
    /// [`StringSource`]s that are used in multiple spots.
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_sources: HashMap<String, StringSource>,
    /// [`StringModification`]s that are used in multiple spots.
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_modifications: HashMap<String, StringModification>,
    /// [`StringMatcher`]s that are used in multiple spots.
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_matchers: HashMap<String, StringMatcher>
}
