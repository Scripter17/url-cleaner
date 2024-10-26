//! Provides [`GlobWrapper`], a serializable/deserializable wrapper around [`Pattern`] and [`MatchOptions`].
//! 
//! Enabled by the `glob` feature flag.

use std::str::FromStr;
use std::path::Path;

use glob::{Pattern, MatchOptions};
use serde::{
    Serialize, Deserialize,
    ser::Serializer,
    de::{Error as _, Deserializer}
};

use crate::util::*;

/// A wrapper around [`glob::Pattern`] and [`glob::MatchOptions`].
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(remote= "Self")]
pub struct GlobWrapper {
    /// The pattern used to match stuff.
    #[serde(flatten, serialize_with = "serialize_pattern", deserialize_with = "deserialize_pattern")]
    pub pattern: Pattern,
    /// The options used to choose how the pattern matches stuff.
    #[serde(flatten, with = "SerdeMatchOptions")]
    pub options: MatchOptions
}

impl From<Pattern> for GlobWrapper {
    /// Creates a [`Self`] using the provided [`Pattern`] and a default [`MatchOptions`].
    fn from(value: Pattern) -> Self {
        Self {
            pattern: value,
            options: Default::default()
        }
    }
}

impl FromStr for GlobWrapper {
    type Err = <Pattern as FromStr>::Err;

    /// Simply treats the string as a glob and defaults the config.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Pattern::from_str(s).map(Into::into)
    }
}

impl TryFrom<&str> for GlobWrapper {
    type Error = <Self as FromStr>::Err;

    /// [`Self::from_str`].
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::from_str(s)
    }
}

crate::util::string_or_struct_magic!(GlobWrapper);

/// A serialization/deserialization helper for [`MatchOptions`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(remote = "MatchOptions")]
struct SerdeMatchOptions {
    /// [`MatchOptions::case_sensitive`].
    #[serde(default = "get_true" , skip_serializing_if = "is_true" )] case_sensitive: bool,
    /// [`MatchOptions::require_literal_separator`].
    #[serde(default = "get_false", skip_serializing_if = "is_false")] require_literal_separator: bool,
    /// [`MatchOptions::require_literal_leading_dot`].
    #[serde(default = "get_true" , skip_serializing_if = "is_true" )] require_literal_leading_dot: bool,
}

/// Deserializer to turn a string into a [`Pattern`].
fn deserialize_pattern<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Pattern, D::Error> {
    let pattern: String=Deserialize::deserialize(deserializer)?;
    Pattern::new(&pattern).map_err(D::Error::custom)
}

/// Serializer to turn a [`Pattern`] into a string.
fn serialize_pattern<S: Serializer>(pattern: &Pattern, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(pattern.as_str())
}

impl GlobWrapper {
    /// Wrapper for [`Pattern::matches_with`].
    #[must_use]
    pub fn matches(&self, str: &str) -> bool {
        self.pattern.matches_with(str, self.options)
    }

    /// Wrapper for [`Pattern::matches_path_with`].
    #[must_use]
    pub fn matches_path(&self, path: &Path) -> bool {
        self.pattern.matches_path_with(path, self.options)
    }
}
