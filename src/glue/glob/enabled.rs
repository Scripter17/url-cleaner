pub use glob::{Pattern, MatchOptions};

use serde::{
    Serialize, Deserialize,
    ser::Serializer,
    de::{Error as _, Deserializer}
};

/// The enabled form of the wrapper around [`glob::Pattern`] and [`glob::MatchOptions`].
/// Only the necessary methods are exposed for the sake of simplicity.
/// Note that if the `glob` feature is disabled, this struct is empty and all provided functions will always panic.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GlobWrapper {
    /// The pattern used to match stuff.
    #[serde(flatten, serialize_with = "serialize_pattern", deserialize_with = "deserialize_pattern")]
    pub pattern: Pattern,
    /// The options used to chooose how the pattern matches stuff.
    #[serde(flatten, with = "SerdeMatchOptions")]
    pub options: MatchOptions
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(remote = "MatchOptions")]
struct SerdeMatchOptions {
    #[serde(default = "get_true" )] case_sensitive: bool,
    #[serde(default = "get_false")] require_literal_separator: bool,
    #[serde(default = "get_true" )] require_literal_leading_dot: bool,
}

/// Serde doesn't have an equivalent to Clap's `default_value_t`
const fn get_true() -> bool {true}
const fn get_false() -> bool {false}

fn deserialize_pattern<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Pattern, D::Error> {
    let pattern: String=Deserialize::deserialize(deserializer)?;
    Pattern::new(&pattern).map_err(|_| D::Error::custom(format!("Invalid glob pattern: {pattern:?}.")))
}

fn serialize_pattern<S: Serializer>(pattern: &Pattern, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(pattern.as_str())
}

impl GlobWrapper {
    /// Wrapper for `glob::Pattern::matches`.
    pub fn matches(&self, str: &str) -> bool {
        self.pattern.matches_with(str, self.options)
    }
}
