//! [`BetterUrl`].

use crate::prelude::*;

#[cfg(test)]
mod tests;
mod new;
mod join;

mod canon;
mod scheme;
mod userinfo;
mod host;
mod port;
mod path;
mod query;
mod fragment;

/// A URL.
#[derive(Debug, Clone)]
pub struct BetterUrl {
    /** The serialization.                            **/ serialization : String,
    /** The `:` marking the scheme.                   **/ scheme_mark   : u32,
    /** The end of the username.                      **/ username_after: Option<NonZero<u32>>,
    /** The start of the host.                        **/ host_start    : Option<NonZero<u32>>,
    /** The `:` marking the port.                     **/ port_mark     : Option<NonZero<u32>>,
    /** If [`Self::port_mark`] is [`Some`], the port. **/ port          : u16,
    /** The start of the path.                        **/ path_start    : u32,
    /** The `?` marking the query.                    **/ query_mark    : Option<NonZero<u32>>,
    /** The `#` marking the fragment.                 **/ fragment_mark : Option<NonZero<u32>>,
    /** The [`UrlDetails`].                           **/ details       : UrlDetails,
}

impl BetterUrl {
    /// The URL as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.serialization
    }

    /// If the URL cannot be a base.
    pub fn cannot_be_a_base(&self) -> bool {
        !self.serialization[self.scheme_mark as usize..].starts_with(":/")
    }

    /// The [`UrlDetails`].
    pub fn details(&self) -> UrlDetails {
        self.details
    }

    /// The length.
    #[expect(clippy::len_without_is_empty, reason = "Can't be empty.")]
    pub fn len(&self) -> usize {
        self.serialization.len()
    }
}



impl TryFrom<Cow<'_, str>> for BetterUrl {
    type Error = InvalidUrl;

    fn try_from(mut value: Cow<'_, str>) -> Result<Self, Self::Error> {
        let start = value.bytes(). position(|b| b > 0x20 && b != 0x7F).unwrap_or(0);
        let end   = value.bytes().rposition(|b| b > 0x20 && b != 0x7F).map_or(0, |x| x + 1);

        value.retain_range(start..end);

        if value.bytes().any(|b| b == b'\t' || b == b'\n' || b == b'\r') {
            value.to_mut().retain(|c| c != '\t' && c != '\n' && c != '\r');
        }

        let i = value.bytes().position(|b| b == b':').ok_or(InvalidUrl::MissingScheme)?;

        let (scheme, rest) = unsafe {(value.get_unchecked(..i), value.get_unchecked(i+1..))};

        Self::after_scheme(Scheme::new(scheme)?, rest)
    }
}

impl FromStr for BetterUrl {
    type Err = InvalidUrl;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.try_into()
    }
}

impl TryFrom<&str> for BetterUrl {
    type Error = InvalidUrl;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Cow::from(value).try_into()
    }
}

impl TryFrom<String> for BetterUrl {
    type Error = InvalidUrl;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Cow::from(value).try_into()
    }
}

impl TryFrom<&String> for BetterUrl {
    type Error = InvalidUrl;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Cow::from(value).try_into()
    }
}



impl From<BetterUrl> for String            {fn from(value: BetterUrl) -> Self {value.serialization       }}
impl From<BetterUrl> for Cow<'static, str> {fn from(value: BetterUrl) -> Self {value.serialization.into()}}

impl AsRef <str> for BetterUrl {fn as_ref(&self) -> &str {self.as_str()}}
impl Borrow<str> for BetterUrl {fn borrow(&self) -> &str {self.as_str()}}

impl AsRef <String> for BetterUrl {fn as_ref(&self) -> &String {&self.serialization}}
impl Borrow<String> for BetterUrl {fn borrow(&self) -> &String {&self.serialization}}



impl PartialEq for BetterUrl {fn eq(&self, other: &Self) -> bool {self.as_str() == other.as_str()}}
impl Eq for BetterUrl {}

impl PartialEq< str         > for BetterUrl {fn eq(&self, other: & str         ) -> bool {self.as_str() ==  other}}
impl PartialEq< String      > for BetterUrl {fn eq(&self, other: & String      ) -> bool {self.as_str() ==  other}}
impl PartialEq< Cow<'_, str>> for BetterUrl {fn eq(&self, other: & Cow<'_, str>) -> bool {self.as_str() ==  other}}
impl PartialEq<&str         > for BetterUrl {fn eq(&self, other: &&str         ) -> bool {self.as_str() == *other}}
impl PartialEq<&String      > for BetterUrl {fn eq(&self, other: &&String      ) -> bool {self.as_str() == *other}}
impl PartialEq<&Cow<'_, str>> for BetterUrl {fn eq(&self, other: &&Cow<'_, str>) -> bool {self.as_str() == *other}}

impl PartialEq<BetterUrl> for  str          {fn eq(&self, other: &BetterUrl) -> bool {other == self}}
impl PartialEq<BetterUrl> for  String       {fn eq(&self, other: &BetterUrl) -> bool {other == self}}
impl PartialEq<BetterUrl> for  Cow<'_, str> {fn eq(&self, other: &BetterUrl) -> bool {other == self}}
impl PartialEq<BetterUrl> for &str          {fn eq(&self, other: &BetterUrl) -> bool {other == self}}
impl PartialEq<BetterUrl> for &String       {fn eq(&self, other: &BetterUrl) -> bool {other == self}}
impl PartialEq<BetterUrl> for &Cow<'_, str> {fn eq(&self, other: &BetterUrl) -> bool {other == self}}

impl Hash for BetterUrl {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.as_str().hash(hasher)
    }
}



impl PartialOrd for BetterUrl {fn partial_cmp(&self, other: &Self) -> Option<Ordering> {Some(self.cmp(other))}}
impl Ord        for BetterUrl {fn cmp        (&self, other: &Self) ->        Ordering  {self.as_str().cmp(other.as_str())}}



impl std::fmt::Display for BetterUrl {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}

#[cfg(feature = "serde")]
impl Serialize for BetterUrl {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_str().serialize(serializer)
    }
}

#[cfg(any(feature = "serde", test))]
impl<'de> Deserialize<'de> for BetterUrl {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::new(<Cow<'de, str>>::deserialize(deserializer)?).map_err(D::Error::custom)
    }
}
