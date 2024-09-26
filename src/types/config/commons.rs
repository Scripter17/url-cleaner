//! Basically functions. Surprisingly useful.

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::types::*;
use crate::util::*;

/// Container for various things that are used in multiple spots.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
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

impl Commons {
    /// Internal method to make sure I don't accidentally commit Debug variants and other stuff unsuitable for the default config.
    pub(crate) fn is_suitable_for_release(&self, config: &Config) -> bool {
        self.rules                   .iter().all(|(_, v)| v.is_suitable_for_release(config)) &&
            self.conditions          .iter().all(|(_, v)| v.is_suitable_for_release(config)) &&
            self.mappers             .iter().all(|(_, v)| v.is_suitable_for_release(config)) &&
            self.string_sources      .iter().all(|(_, v)| v.is_suitable_for_release(config)) &&
            self.string_modifications.iter().all(|(_, v)| v.is_suitable_for_release(config)) &&
            self.string_matchers     .iter().all(|(_, v)| v.is_suitable_for_release(config))
    }
}
