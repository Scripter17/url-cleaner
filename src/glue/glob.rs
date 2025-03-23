//! Glue for [`glob`].

use std::str::FromStr;
use std::path::Path;

use glob::{Pattern, MatchOptions};
use serde::{
    Serialize, Deserialize,
    ser::Serializer,
    de::{Error as _, Deserializer}
};

use crate::types::*;
use crate::util::*;

/// Wrapper around [`glob::Pattern`] and [`glob::MatchOptions`] to keep it them in one place.
/// # Examples
/// ```
/// # use ::glob::*;
/// # use url_cleaner::glue::*;
/// let glob = GlobWrapper::try_from("abc/*/def").unwrap();
/// assert!( glob.matches("abc/123/def"));
/// assert!(!glob.matches("ABC/123/ABC")); // By default, globs are case sensitive.
/// 
/// let glob = GlobWrapper {
///     pattern: Pattern::new("abc/*/def").unwrap(),
///     options: MatchOptions {
///         case_sensitive: false,
///         ..Default::default()
///     }
/// };
/// assert!(glob.matches("abc/123/def"));
/// assert!(glob.matches("ABC/123/DEF"));
/// ```
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Suitability)]
#[serde(remote= "Self")]
pub struct GlobWrapper {
    /// The [`Pattern`] to use.
    #[serde(serialize_with = "serialize_pattern", deserialize_with = "deserialize_pattern")]
    pub pattern: Pattern,
    /// The [`MatchOptions`] to use.
    #[serde(flatten, with = "SerdeMatchOptions")]
    pub options: MatchOptions
}

impl From<Pattern> for GlobWrapper {
    fn from(value: Pattern) -> Self {
        Self {
            pattern: value,
            options: Default::default()
        }
    }
}

impl FromStr for GlobWrapper {
    type Err = <Pattern as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Pattern::from_str(s).map(Into::into)
    }
}

impl TryFrom<&str> for GlobWrapper {
    type Error = <Self as FromStr>::Err;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::from_str(s)
    }
}

crate::util::string_or_struct_magic!(GlobWrapper);

/// Serde helper for deserializing [`MatchOptions`].
#[derive(Debug, Clone, Serialize, Deserialize, Suitability)]
#[serde(remote = "MatchOptions")]
struct SerdeMatchOptions {
    /// [`MatchOptions::case_sensitive`].
    ///
    /// Defaults to [`true`].
    #[serde(default = "get_true" , skip_serializing_if = "is_true" )] case_sensitive: bool,
    /// [`MatchOptions::require_literal_separator`].
    ///
    /// Defaults to [`false`].
    #[serde(default = "get_false", skip_serializing_if = "is_false")] require_literal_separator: bool,
    /// [`MatchOptions::require_literal_leading_dot`].
    ///
    /// Defaults to [`true`].
    #[serde(default = "get_true" , skip_serializing_if = "is_true" )] require_literal_leading_dot: bool,
}

/// Serde helper to deserialize [`Pattern`]s.
fn deserialize_pattern<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Pattern, D::Error> {
    let pattern: String=Deserialize::deserialize(deserializer)?;
    Pattern::new(&pattern).map_err(D::Error::custom)
}
/// Serde helper to serialize [`Pattern`]s.
fn serialize_pattern<S: Serializer>(pattern: &Pattern, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(pattern.as_str())
}

impl GlobWrapper {
    /// Returns [`true`] if [`Self::pattern`] matches `s` with [`Self::options`].
    #[must_use]
    pub fn matches(&self, s: &str) -> bool {
        self.pattern.matches_with(s, self.options)
    }

    /// Returns [`true`] if [`Self::pattern`] matches `path` with [`Self::options`].
    #[must_use]
    #[allow(dead_code, reason = "Public API.")]
    pub fn matches_path(&self, path: &Path) -> bool {
        self.pattern.matches_path_with(path, self.options)
    }
}
