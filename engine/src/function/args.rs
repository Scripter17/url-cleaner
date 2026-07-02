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
    pub flags: HashSet<String>,
    /// The vars.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>,
    /// The [`Set`]s.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub sets: HashMap<String, Set<String>>,
    /// The [`List`]s.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub lists: HashMap<String, List<String>>,
    /// The [`Map`]s.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub maps: HashMap<String, Map<String>>,



    /// The [`Condition`]s.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub conditions: HashMap<String, Condition>,
    /// The [`Action`]s.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub actions: HashMap<String, Action>,
    /// The [`StringSource`]s.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_sources: HashMap<String, StringSource>,
    /// The [`StringModification`]s.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_modifications: HashMap<String, StringModification>,
    /// The [`StringMatcher`]s.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_matchers: HashMap<String, StringMatcher>
}
