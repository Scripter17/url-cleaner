//! [`Functions`]

use crate::prelude::*;

/// Functions to be called at multiple points in a [`Cleaner`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct Functions {
    /// The [`Condition`]s.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub conditions: FxHashMap<String, Condition>,
    /// The [`Action`]s.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub actions: FxHashMap<String, Action>,
    /// The [`StringSource`]s.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_sources: FxHashMap<String, StringSource>,
    /// The [`StringModification`]s.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_modifications: FxHashMap<String, StringModification>,
    /// The [`StringMatcher`]s.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_matchers: FxHashMap<String, StringMatcher>,
}

impl Functions {
    /// If every field is empty.
    pub fn is_empty(&self) -> bool {
        self.conditions                 .is_empty()
            && self.actions             .is_empty()
            && self.string_sources      .is_empty()
            && self.string_modifications.is_empty()
            && self.string_matchers     .is_empty()
    }
}
