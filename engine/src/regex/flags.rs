//! [`RegexFlags`].

use crate::prelude::*;

/// A set of flags for a [`RegexConfig`].
///
/// The default value is unicode enabld and the rest disabled, mirroring [`regex`].
///
/// The string format is of 0 or more extra flags to enabled then optionally a `-` and 0 or more flags to disable.
///
/// The flags are as follows:
///
/// - `i`: bit 0, [`Self::case_insensitive`].
/// - `R`: bit 1, [`Self::crlf`].
/// - `s`: bit 2, [`Self::dot_matches_new_line`].
/// - `x`: bit 3, [`Self::ignore_whitespace`].
/// - `m`: bit 4, [`Self::multi_line`].
/// - `o`: bit 5, [`Self::octal`].
/// - `U`: bit 6, [`Self::swap_greed`].
/// - `u`: bit 7, [`Self::unicode`].
/// # Examples
/// ```
/// use url_cleaner_engine::prelude::*;
///
/// let mut x = "".parse::<RegexFlags>().unwrap();
/// assert_eq!(x, RegexFlags(0b1000_0000));
/// assert!(x.unicode());
///
/// x.set_case_insensitive(true);
/// assert_eq!(x, RegexFlags(0b1000_0001));
/// assert!(x.case_insensitive());
///
/// x.set_unicode(false);
/// assert_eq!(x, RegexFlags(0b0000_0001));
/// assert!(!x.unicode());
///
/// assert_eq!(x, "i-u".parse().unwrap());
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Suitability)]
pub struct RegexFlags(pub u8);

impl Default for RegexFlags {
    fn default() -> Self {
        Self(0b1000_0000)
    }
}

impl RegexFlags {
    /** If [`regex::RegexBuilder::case_insensitive    `] is set. **/ pub const fn case_insensitive        (self                  ) -> bool {get_bit(     self.0, 0       )}
    /** If [`regex::RegexBuilder::crlf                `] is set. **/ pub const fn crlf                    (self                  ) -> bool {get_bit(     self.0, 1       )}
    /** If [`regex::RegexBuilder::dot_matches_new_line`] is set. **/ pub const fn dot_matches_new_line    (self                  ) -> bool {get_bit(     self.0, 2       )}
    /** If [`regex::RegexBuilder::ignore_whitespace   `] is set. **/ pub const fn ignore_whitespace       (self                  ) -> bool {get_bit(     self.0, 3       )}
    /** If [`regex::RegexBuilder::multi_line          `] is set. **/ pub const fn multi_line              (self                  ) -> bool {get_bit(     self.0, 4       )}
    /** If [`regex::RegexBuilder::octal               `] is set. **/ pub const fn octal                   (self                  ) -> bool {get_bit(     self.0, 5       )}
    /** If [`regex::RegexBuilder::swap_greed          `] is set. **/ pub const fn swap_greed              (self                  ) -> bool {get_bit(     self.0, 6       )}
    /** If [`regex::RegexBuilder::unicode             `] is set. **/ pub const fn unicode                 (self                  ) -> bool {get_bit(     self.0, 7       )}

    /** Set [`regex::RegexBuilder::case_insensitive    `].       **/ pub const fn set_case_insensitive    (&mut self, value: bool)         {set_bit(&mut self.0, 0, value)}
    /** Set [`regex::RegexBuilder::crlf                `].       **/ pub const fn set_crlf                (&mut self, value: bool)         {set_bit(&mut self.0, 1, value)}
    /** Set [`regex::RegexBuilder::dot_matches_new_line`].       **/ pub const fn set_dot_matches_new_line(&mut self, value: bool)         {set_bit(&mut self.0, 2, value)}
    /** Set [`regex::RegexBuilder::ignore_whitespace   `].       **/ pub const fn set_ignore_whitespace   (&mut self, value: bool)         {set_bit(&mut self.0, 3, value)}
    /** Set [`regex::RegexBuilder::multi_line          `].       **/ pub const fn set_multi_line          (&mut self, value: bool)         {set_bit(&mut self.0, 4, value)}
    /** Set [`regex::RegexBuilder::octal               `].       **/ pub const fn set_octal               (&mut self, value: bool)         {set_bit(&mut self.0, 5, value)}
    /** Set [`regex::RegexBuilder::swap_greed          `].       **/ pub const fn set_swap_greed          (&mut self, value: bool)         {set_bit(&mut self.0, 6, value)}
    /** Set [`regex::RegexBuilder::unicode             `].       **/ pub const fn set_unicode             (&mut self, value: bool)         {set_bit(&mut self.0, 7, value)}
}

/// Get the `i`th bit of `x`.
const fn get_bit(x: u8, i: u8) -> bool {
    x & 1 << i != 0
}

/// Set the `i`th bit of `x`.
const fn set_bit(x: &mut u8, i: u8, b: bool) {
    *x &= !(1 << i);
    *x |= (b as u8) << i;
}

impl FromStr for RegexFlags {
    type Err = InvalidRegexFlags;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret = Self::default();
        let mut x = true;

        for b in s.bytes() {
            match b {
                b'-' if x => x = false,
                b'i' => ret.set_case_insensitive    (x),
                b'R' => ret.set_crlf                (x),
                b's' => ret.set_dot_matches_new_line(x),
                b'x' => ret.set_ignore_whitespace   (x),
                b'm' => ret.set_multi_line          (x),
                b'o' => ret.set_octal               (x),
                b'U' => ret.set_swap_greed          (x),
                b'u' => ret.set_unicode             (x),
                _ => Err(InvalidRegexFlags)?
            }
        }

        Ok(ret)
    }
}

impl TryFrom<&str> for RegexFlags {
    type Error = InvalidRegexFlags;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::fmt::Debug for RegexFlags {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "\"")?;
        std::fmt::Display::fmt(self, formatter)?;
        write!(formatter, "\"")?;
        Ok(())
    }
}

impl std::fmt::Display for RegexFlags {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if  self.case_insensitive    () {write!(formatter,  "i")?;}
        if  self.crlf                () {write!(formatter,  "R")?;}
        if  self.dot_matches_new_line() {write!(formatter,  "s")?;}
        if  self.ignore_whitespace   () {write!(formatter,  "x")?;}
        if  self.multi_line          () {write!(formatter,  "m")?;}
        if  self.octal               () {write!(formatter,  "o")?;}
        if  self.swap_greed          () {write!(formatter,  "U")?;}
        if !self.unicode             () {write!(formatter, "-u")?;}

        Ok(())
    }
}

impl Serialize for RegexFlags {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for RegexFlags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <Cow<'de, str>>::deserialize(deserializer)?.parse().map_err(D::Error::custom)
    }
}
