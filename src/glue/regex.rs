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

impl FromStr for RegexWrapper {
    type Err = Box<RegexSyntaxError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RegexParts::new(s).map(Into::into)
    }
}

impl PartialEq for RegexWrapper {
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

impl RegexWrapper {
    /// Gets the cached compiled regex and compiles it first if it's not already cached.
    /// # Panics
    /// Although the regex is guaranteed to be syntactically valid, it is possible to exceed the default DFA size limit. In that case, this method will panic.
    /// For the sake of API design, I consider that a niche enough case that this warning is sufficient.
    pub fn get_regex(&self) -> &Regex {
        self.regex.get_or_init(|| self.parts.build()
            .expect("The regex to have been validated during `RegexParts::new` and the regex to not exceed configured limits."))
    }

    /// Mutably gets the cached compiled regex and compiles it first if it's not already cached.
    /// # Panics
    /// Panics whenever [`Self::get_regex`] would as it calls that function.
    /// # Safety
    /// [`RegexWrapper`] assumes the contained [`RegexParts`] and [`Regex`] are always the same.
    /// The exact behaviour of a desync is unspecified but should be fairly limited in practice.
    pub unsafe fn get_regex_mut(&mut self) -> &mut Regex {
        self.get_regex();
        self.regex.get_mut().expect("The regex to have just been set.") // No [`OnceLock::get_mut_or_init`] as of 1.75.
    }

    /// Mutably gets the [`OnceLock`] containging the cached [`Regex`].
    /// # Safety
    /// Because [`OnceLock`] has interior mutability, this is effectively as unsafe as [`Self::get_regex_container_mut`].
    /// [`RegexWrapper`] assumes the contained [`RegexParts`] and [`Regex`] are always the same.
    /// The exact behaviour of a desync is unspecified but should be fairly limited in practice.
    pub unsafe fn get_regex_container(&self) -> &OnceLock<Regex> {
        &self.regex
    }

    /// Mutably gets the [`OnceLock`] containging the cached [`Regex`].
    /// # Safety
    /// [`RegexWrapper`] assumes the contained [`RegexParts`] and [`Regex`] are always the same.
    /// The exact behaviour of a desync is unspecified but should be fairly limited in practice.
    pub unsafe fn get_regex_container_mut(&mut self) -> &mut OnceLock<Regex> {
        &mut self.regex
    }

    /// Gets the contained [`RegexParts`].
    pub fn get_regex_parts(&self) -> &RegexParts {
        &self.parts
    }

    /// Mutably gets the contained [`RegexParts`].
    /// # Safety
    /// [`RegexWrapper`] assumes the contained [`RegexParts`] and [`Regex`] are always the same.
    /// The exact behaviour of a desync is unspecified but should be fairly limited in practice.
    /// If the regex hasn't been compiled and cached yet (or the [`OnceLock`] it's stored in has been cleared via [`OnceLock::take`] or by being replaced) then mutating this *should* be fine.
    /// Though in that case you should probably just [`Into::into`] this back into a [`RegexParts`], mutate that, then makea a new [`RegexWrapper`].
    /// I'm not the boss of you. That's why I'm providing these functions.
    pub unsafe fn get_regex_parts_mut(&mut self) -> &mut RegexParts {
        &mut self.parts
    }

    /// A convenience wrapper around [`Regex::find`].
    /// # Panics
    /// Panics whenever [`Self::get_regex`] would as it calls that function.
    pub fn find<'h>(&self, haystack: &'h str) -> Option<Match<'h>> {
        self.get_regex().find(haystack)
    }

    /// A convenience wrapper around [`Regex::captures`].
    /// # Panics
    /// Panics whenever [`Self::get_regex`] would as it calls that function.
    pub fn captures<'h>(&self, haystack: &'h str) -> Option<Captures<'h>> {
        self.get_regex().captures(haystack)
    }

    /// A convenience wrapper around [`Regex::is_match`].
    /// # Panics
    /// Panics whenever [`Self::get_regex`] would as it calls that function.
    pub fn is_match(&self, haystack: &str) -> bool {
        self.get_regex().is_match(haystack)
    }

    /// A convenience wrapper around [`Regex::replace`].
    /// # Panics
    /// Panics whenever [`Self::get_regex`] would as it calls that function.
    pub fn replace<'h, R: Replacer>(&self, haystack: &'h str, rep: R) -> Cow<'h, str> {
        self.get_regex().replace(haystack, rep)
    }

    /// A convenience wrapper around [`Regex::replace_all`].
    /// # Panics
    /// Panics whenever [`Self::get_regex`] would as it calls that function.
    pub fn replace_all<'h, R: Replacer>(&self, haystack: &'h str, rep: R) -> Cow<'h, str> {
        self.get_regex().replace_all(haystack, rep)
    }

    /// A convenience wrapper around [`Regex::replacen`].
    /// # Panics
    /// Panics whenever [`Self::get_regex`] would as it calls that function.
    pub fn replacen<'h, R: Replacer>(&self, haystack: &'h str, limit: usize, rep: R) -> Cow<'h, str> {
        self.get_regex().replacen(haystack, limit, rep)
    }
}
