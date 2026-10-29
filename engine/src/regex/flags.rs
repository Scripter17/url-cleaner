//! [`RegexFlags`].

use crate::prelude::*;

/// A set of flags for a [`RegexConfig`].
///
/// The default value is unicode enabled and the rest disabled, mirroring [`regex`].
///
/// There is a simple string format that is 0 or more flags to enable, optionally followed by a - and 0 or more flags to disable.
///
/// The flags are as follows:
///
/// - `i`: bit 0, [`regex::RegexBuilder::case_insensitive`].
/// - `R`: bit 1, [`regex::RegexBuilder::crlf`].
/// - `s`: bit 2, [`regex::RegexBuilder::dot_matches_new_line`].
/// - `x`: bit 3, [`regex::RegexBuilder::ignore_whitespace`].
/// - `m`: bit 4, [`regex::RegexBuilder::multi_line`].
/// - `o`: bit 5, [`regex::RegexBuilder::octal`].
/// - `U`: bit 6, [`regex::RegexBuilder::swap_greed`].
/// - `u`: bit 7, [`regex::RegexBuilder::unicode`].
/// # Examples
/// ```
/// use url_cleaner_engine::prelude::*;
///
/// assert_eq!(RegexFlags::default(), RegexFlags(0b1000_0000));
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
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Suitability)]
pub struct RegexFlags(pub u8);

impl Default for RegexFlags {
    fn default() -> Self {
        Self(0b1000_0000)
    }
}

impl RegexFlags {
    /// Parse a flags string.
    /// # Errors
    /// If `value` is an invalid regex flags string, returns the error [`InvalidRegexFlags`].
    pub const fn parse(value: &str) -> Result<Self, InvalidRegexFlags> {
        let mut ret = Self(0b1000_0000);
        let mut bit = true;

        let value = value.as_bytes();
        let mut i = 0;

        while i < value.len() {
            match value[i] {
                b'-' if bit => bit = false,
                b'i'        => ret.set_case_insensitive    (bit),
                b'R'        => ret.set_crlf                (bit),
                b's'        => ret.set_dot_matches_new_line(bit),
                b'x'        => ret.set_ignore_whitespace   (bit),
                b'm'        => ret.set_multi_line          (bit),
                b'o'        => ret.set_octal               (bit),
                b'U'        => ret.set_swap_greed          (bit),
                b'u'        => ret.set_unicode             (bit),
                _           => return Err(InvalidRegexFlags),
            }

            i += 1;
        }

        Ok(ret)
    }

    /** Get [`regex::RegexBuilder::case_insensitive`].     */ pub const fn case_insensitive    (self) -> bool {self.0 & 0b0000_0001 != 0}
    /** Get [`regex::RegexBuilder::crlf`].                 */ pub const fn crlf                (self) -> bool {self.0 & 0b0000_0010 != 0}
    /** Get [`regex::RegexBuilder::dot_matches_new_line`]. */ pub const fn dot_matches_new_line(self) -> bool {self.0 & 0b0000_0100 != 0}
    /** Get [`regex::RegexBuilder::ignore_whitespace`].    */ pub const fn ignore_whitespace   (self) -> bool {self.0 & 0b0000_1000 != 0}
    /** Get [`regex::RegexBuilder::multi_line`].           */ pub const fn multi_line          (self) -> bool {self.0 & 0b0001_0000 != 0}
    /** Get [`regex::RegexBuilder::octal`].                */ pub const fn octal               (self) -> bool {self.0 & 0b0010_0000 != 0}
    /** Get [`regex::RegexBuilder::swap_greed`].           */ pub const fn swap_greed          (self) -> bool {self.0 & 0b0100_0000 != 0}
    /** Get [`regex::RegexBuilder::unicode`].              */ pub const fn unicode             (self) -> bool {self.0 & 0b1000_0000 != 0}

    /** Set [`regex::RegexBuilder::case_insensitive`].     */ pub const fn set_case_insensitive    (&mut self, value: bool) {self.0 = (self.0 & 0b1111_1110) | (value as u8)     ;}
    /** Set [`regex::RegexBuilder::crlf`].                 */ pub const fn set_crlf                (&mut self, value: bool) {self.0 = (self.0 & 0b1111_1101) | (value as u8) << 1;}
    /** Set [`regex::RegexBuilder::dot_matches_new_line`]. */ pub const fn set_dot_matches_new_line(&mut self, value: bool) {self.0 = (self.0 & 0b1111_1011) | (value as u8) << 2;}
    /** Set [`regex::RegexBuilder::ignore_whitespace`].    */ pub const fn set_ignore_whitespace   (&mut self, value: bool) {self.0 = (self.0 & 0b1111_0111) | (value as u8) << 3;}
    /** Set [`regex::RegexBuilder::multi_line`].           */ pub const fn set_multi_line          (&mut self, value: bool) {self.0 = (self.0 & 0b1110_1111) | (value as u8) << 4;}
    /** Set [`regex::RegexBuilder::octal`].                */ pub const fn set_octal               (&mut self, value: bool) {self.0 = (self.0 & 0b1101_1111) | (value as u8) << 5;}
    /** Set [`regex::RegexBuilder::swap_greed`].           */ pub const fn set_swap_greed          (&mut self, value: bool) {self.0 = (self.0 & 0b1011_1111) | (value as u8) << 6;}
    /** Set [`regex::RegexBuilder::unicode`].              */ pub const fn set_unicode             (&mut self, value: bool) {self.0 = (self.0 & 0b0111_1111) | (value as u8) << 7;}
}

impl FromStr for RegexFlags {
    type Err = InvalidRegexFlags;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl TryFrom<&str> for RegexFlags {
    type Error = InvalidRegexFlags;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl std::fmt::Debug for RegexFlags {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "\"{self}\"")
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
