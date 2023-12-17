use serde::{de::Error, Deserialize, Deserializer};

#[cfg(feature = "regex")]
pub use regex::{Regex, RegexBuilder};

#[cfg(not(feature = "regex"))]
pub struct Regex;

#[derive(Debug, Deserialize)]
struct RegexParts {
    pattern: String,
    dot_all: bool,
    ignore_whitespace: bool,
    line_terminator: u8,
    multi_line: bool,
    octal: bool,
    swap_greed: bool,
    unicode: bool
}

pub fn deserialize_regex<'de, D>(deserializer: D) -> Result<Regex, D::Error>
where
    D: Deserializer<'de>
{
    if cfg!(not(feature = "regex")) {
        Err(D::Error::custom("The binary was compiled without support for regex"))
    } else {
        let regex_parts: RegexParts = Deserialize::deserialize(deserializer)?;
        RegexBuilder::new(&regex_parts.pattern)
            .dot_matches_new_line(regex_parts.dot_all)
            .ignore_whitespace(regex_parts.ignore_whitespace)
            .line_terminator(regex_parts.line_terminator)
            .multi_line(regex_parts.multi_line)
            .octal(regex_parts.octal)
            .swap_greed(regex_parts.swap_greed)
            .unicode(regex_parts.unicode)
            .build()
            .map_err(|_| D::Error::custom(format!("Invalid regex pattern: {:?}", regex_parts.pattern)))
    }
}
