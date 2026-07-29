//! [`FunctionArgs`].

use crate::prelude::*;

/// The arguments to a [`FunctionCall`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct FunctionArgs {
    /// The flags.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: FxHashSet<String>,
    /// The vars.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: FxHashMap<String, String>,
    /// The [`Set`]s.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub sets: FxHashMap<String, Set<String>>,
    /// The [`List`]s.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub lists: FxHashMap<String, List<String>>,
    /// The [`Map`]s.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub maps: FxHashMap<String, Map<String>>,



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
    pub string_matchers: FxHashMap<String, StringMatcher>
}
