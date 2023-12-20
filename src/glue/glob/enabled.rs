use serde::{de::Error, Deserialize, Deserializer};
pub use glob::{Pattern, MatchOptions};

#[derive(Clone, Debug, Deserialize)]
pub struct Glob {
    #[serde(flatten, deserialize_with = "deserialize_pattern")]
    inner: Pattern,
    #[serde(flatten, with = "DeMatchOptions")]
    options: MatchOptions
}

#[derive(Debug, Deserialize)]
struct PatternParts {
    pattern: String
}

#[cfg(feature = "glob")]
#[derive(Debug, Clone, Deserialize)]
#[serde(remote = "MatchOptions")]
struct DeMatchOptions {
    #[serde(default = "get_true")]
    case_sensitive: bool,
    #[serde(default = "get_false")]
    require_literal_separator: bool,
    #[serde(default = "get_true")]
    require_literal_leading_dot: bool,
}

fn get_true() -> bool {true}
fn get_false() -> bool {false}

pub fn deserialize_pattern<'de, D>(deserializer: D) -> Result<Pattern, D::Error>
where
    D: Deserializer<'de>
{
    if cfg!(not(feature = "glob")) {
        Err(D::Error::custom("Url-cleaner was compiled without support for glob"))
    } else {
        let pattern_parts: PatternParts = Deserialize::deserialize(deserializer)?;
        Pattern::new(&pattern_parts.pattern).map_err(|_| D::Error::custom(format!("Invalid glob pattern: {:?}", pattern_parts.pattern)))
    }
}

impl Glob {
    pub fn matches(&self, str: &str) -> bool {
        self.inner.matches_with(str, self.options.clone())
    }
}