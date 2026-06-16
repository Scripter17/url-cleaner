//! [`FunctionArgs`].

use crate::prelude::*;

/// The arguments to a [`FunctionCall`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct FunctionArgs {
    /// Flags.
    ///
    /// Defaults to an empty [`HashSet`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: HashSet<String>,
    /// Vars.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>,
    /// [`Set`]s.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub sets: HashMap<String, Set<String>>,
    /// Lists.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub lists: HashMap<String, Vec<String>>,
    /// [`Maps`].
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub maps: HashMap<String, Map<String>>,



    /// [`Condition`]s.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub conditions: HashMap<String, Condition>,
    /// [`Action`]s.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub actions: HashMap<String, Action>,
    /// [`StringSource`]s.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_sources: HashMap<String, StringSource>,
    /// [`StringModification`]s.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_modifications: HashMap<String, StringModification>,
    /// [`StringMatcher`]s.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_matchers: HashMap<String, StringMatcher>
}
