use serde::{de::Error, Deserialize, Deserializer};

#[cfg(feature = "glob")]
pub use glob::{Pattern, MatchOptions};

#[cfg(feature = "glob")]
#[derive(Clone, Debug, Deserialize)]
pub struct Glob {
    #[serde(deserialize_with = "deserialize_pattern")]
    inner: Pattern,
    #[serde(flatten, with = "DeMatchOptions")]
    options: MatchOptions
}
#[cfg(not(feature = "glob"))]
pub struct Glob;

#[derive(Debug, Deserialize)]
struct DePattern {
    pattern: String
}

#[derive(Debug, Clone, Deserialize)]
#[serde(remote = "MatchOptions")]
struct DeMatchOptions {
    case_sensitive: bool,
    require_literal_separator: bool,
    require_literal_leading_dot: bool,
}

pub fn deserialize_pattern<'de, D>(deserializer: D) -> Result<Pattern, D::Error>
where
    D: Deserializer<'de>
{
    if cfg!(not(feature = "glob")) {
        Err(D::Error::custom("The binary was compiled without support for glob"))
    } else {
        let pattern_parts: DePattern = Deserialize::deserialize(deserializer)?;
        Pattern::new(&pattern_parts.pattern).map_err(|_| D::Error::custom(format!("Invalid glob pattern: {:?}", pattern_parts.pattern)))
    }
}

impl Glob {
    pub fn matches(&self, str: &str) -> bool {
        self.inner.matches_with(str, self.options.clone())
    }
}
