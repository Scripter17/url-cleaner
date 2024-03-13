mod regex_parts;
pub use regex_parts::*;

use std::str::FromStr;
use std::sync::OnceLock;
use std::borrow::Cow;

use serde::{Serialize, Deserialize};

use regex_syntax::Error as RegexSyntaxError;
use regex::{Regex, Replacer, Match, Captures};

/// A wrapper around both a [`OnceLock`] of a [`Regex`] and a [`RegexParts`].
/// This is because converting a [`Regex`] into a [`RegexParts`] is extremely complicated and because it allows lazy compilation of regexes.
/// Because the contained regex and regex parts have to always be in sync, the fields of this struct are unfortunately private.
/// In place of public fields, various [`Into`]'s and getters are defined for this type.
/// This does not implement [`std::ops::Deref`] or [`std::convert::AsRef`]`<`[`Regex`]`>` because [`Self::get_regex`] can panic, which is disallowed in [`std::ops::Deref::deref`] and [`std::convert::AsRef::as_ref`].
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(from = "RegexParts", into = "RegexParts")]
pub struct RegexWrapper {
    regex: OnceLock<Regex>,
    parts: RegexParts
}

impl From<RegexParts> for RegexWrapper {
    fn from(parts: RegexParts) -> Self {
        Self {
            regex: OnceLock::new(),
            parts
        }
    }
}

impl From<RegexWrapper> for (OnceLock<Regex>, RegexParts) {
    fn from(value: RegexWrapper) -> Self {
        (value.regex, value.parts)
    }
}

impl FromStr for RegexWrapper {
    type Err = Box<RegexSyntaxError>;

    /// [`RegexParts::from_str`].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RegexParts::from_str(s).map(Into::into)
    }
}

impl PartialEq for RegexWrapper {
    /// [`RegexParts::eq`].
    fn eq(&self, other: &Self) -> bool {
        self.parts.eq(&other.parts)
    }
}
impl Eq for RegexWrapper {}

impl From<RegexWrapper> for RegexParts {
    fn from(value: RegexWrapper) -> Self {
        value.parts
    }
}

impl AsRef<RegexParts> for RegexWrapper {
    fn as_ref(&self) -> &RegexParts {
        &self.parts
    }
}

impl RegexWrapper {
    /// Gets the cached compiled regex and compiles it first if it's not already cached.
    /// # Panics
    /// Although the regex is guaranteed to be syntactically valid, it is possible it will exceed the default DFA size limit. In that case, this method will panic.
    /// For the sake of API design, I consider that a niche enough case that this warning is sufficient.
    pub fn get_regex(&self) -> &Regex {
        self.regex.get_or_init(|| self.parts.build()
            .expect("The regex to not exceed the DFA size limit."))
    }

    /// Gets the contained [`RegexParts`].
    pub fn get_regex_parts(&self) -> &RegexParts {
        &self.parts
    }

    /// A convenience wrapper around [`Regex::find`].
    /// # Panics
    /// Panics whenever [`Self::get_regex`] would as it calls that method.
    pub fn find<'h>(&self, haystack: &'h str) -> Option<Match<'h>> {
        self.get_regex().find(haystack)
    }

    /// A convenience wrapper around [`Regex::captures`].
    /// # Panics
    /// Panics whenever [`Self::get_regex`] would as it calls that method.
    pub fn captures<'h>(&self, haystack: &'h str) -> Option<Captures<'h>> {
        self.get_regex().captures(haystack)
    }

    /// A convenience wrapper around [`Regex::is_match`].
    /// # Panics
    /// Panics whenever [`Self::get_regex`] would as it calls that method.
    pub fn is_match(&self, haystack: &str) -> bool {
        self.get_regex().is_match(haystack)
    }

    /// A convenience wrapper around [`Regex::replace`].
    /// # Panics
    /// Panics whenever [`Self::get_regex`] would as it calls that method.
    pub fn replace<'h, R: Replacer>(&self, haystack: &'h str, rep: R) -> Cow<'h, str> {
        self.get_regex().replace(haystack, rep)
    }

    /// A convenience wrapper around [`Regex::replace_all`].
    /// # Panics
    /// Panics whenever [`Self::get_regex`] would as it calls that method.
    pub fn replace_all<'h, R: Replacer>(&self, haystack: &'h str, rep: R) -> Cow<'h, str> {
        self.get_regex().replace_all(haystack, rep)
    }

    /// A convenience wrapper around [`Regex::replacen`].
    /// # Panics
    /// Panics whenever [`Self::get_regex`] would as it calls that method.
    pub fn replacen<'h, R: Replacer>(&self, haystack: &'h str, limit: usize, rep: R) -> Cow<'h, str> {
        self.get_regex().replacen(haystack, limit, rep)
    }
}
