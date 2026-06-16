//! [`BetterUrl`].

use crate::prelude::*;

mod scheme;
mod userinfo;
mod host;
mod port;
mod path;
mod query;
mod fragment;

/// A [`url::Url`] with a [`UrlDetails`] and a much nicer API.
///
/// # Performance
///
/// ```
/// use better_url::prelude::*;
///
/// let mut url = BetterUrl::parse("http://example.com").unwrap();
///
/// // Here, "HTTPS" is checked for validity and corrected to "https".
/// let scheme = Scheme::new("HTTPS").unwrap();
///
/// // See?
/// assert_eq!(scheme.as_str(), "https");
///
/// // However, because [`BetterUrl::set_scheme`] calls [`url::Url::set_scheme`], that check is done twice.
/// // So this is slightly slower than it should be.
/// url.set_scheme(&scheme).unwrap();
///
/// // That said, [`url::Url::set_scheme`] is only called when the value is actually different.
/// // So this doesn't call it.
/// url.set_scheme(scheme).unwrap();
///
/// // And yes you can pass in plain strings.
/// url.set_scheme("https").unwrap();
/// ```
///
/// # Opaque paths
///
/// To compensate for a bug in [`url`]'s opaque path parser and a flaw in [`url::Url::set_path`], opaque paths ending in a space have that space (but only that space) replaced with `%20`.
///
/// See [servo/rust-url#1123](https://github.com/servo/rust-url/issues/1123) and [whatwg/url#909](https://github.com/whatwg/url/issues/909) for discussion.
///
/// ```
/// use better_url::prelude::*;
///
/// let url = url::Url::parse("a:  ?").unwrap();
/// // This is actually not spec compliant; It should already be ` %20`.
/// assert_eq!(url.path(), "  ");
///
/// assert_eq!(BetterUrl::from(url).path(), " %20");
///
///
/// let mut url = url::Url::parse("a:  ").unwrap();
/// // When parsing a URL, leading and trailing spaces are removed, which is what opens up the UB that [`url::Url::set_path`] gets wrong.
/// assert_eq!(url.path(), "");
/// // So we set it here.
/// url.set_path("  ");
/// // Technically spec undefined, but really should be corrected to ` %20`.
/// assert_eq!(url.path(), "  ");
/// // Like, it doesn't roundtip.
/// assert_ne!(url::Url::parse(url.as_str()).unwrap(), url);
///
/// assert_eq!(BetterUrl::from(url).path(), " %20");
///
/// // Side note, the base URL crate already does the "correct" thing for leading slashes.
/// let mut url = url::Url::parse("a:").unwrap();
/// url.set_path("/abc/");
/// assert_eq!(url.path(), "%2Fabc/");
/// ```
#[derive(Debug, Clone)]
pub struct BetterUrl {
    /// The [`url::Url`].
    url: url::Url,
    /// The [`Details`].
    details: UrlDetails,
}

impl BetterUrl {
    /// Parse a URL.
    /// # Errors
    /// If the call to [`url::Url::parse`] returns an error, that error is returned.
    pub fn parse(value: &str) -> Result<Self, url::ParseError> {
        value.parse()
    }

    /// The [`UrlDetails`].
    pub fn details(&self) -> &UrlDetails {
        &self.details
    }

    /// The length.
    #[allow(clippy::len_without_is_empty, reason = "Can't be empty.")]
    pub fn len(&self) -> usize {
        self.as_str().len()
    }

    /// [`SchemeDetails::is_special`].
    pub fn is_special(&self) -> bool {
        self.scheme_details().is_special()
    }

    /// [`SchemeDetails::is_special_not_file`].
    pub fn is_special_not_file(&self) -> bool {
        self.scheme_details().is_special_not_file()
    }

    /// [`SchemeDetails::is_file`].
    pub fn is_file(&self) -> bool {
        self.scheme_details().is_file()
    }

    /// [`SchemeDetails::is_non_special`].
    pub fn is_non_special(&self) -> bool {
        self.scheme_details().is_non_special()
    }
}



impl Deref for BetterUrl {
    type Target = url::Url;

    fn deref(&self) -> &Self::Target {
        &self.url
    }
}



impl FromStr for BetterUrl {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        url::Url::from_str(s).map(Into::into)
    }
}

impl TryFrom<&str> for BetterUrl {
    type Error = url::ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl From<url::Url> for BetterUrl {
    fn from(mut value: url::Url) -> Self {
        if let Some(x) = value.path().strip_suffix(' ') {
            value.set_path(&format!("{x}%20"));
        }

        Self {
            details: UrlDetails::from_url(&value),
            url: value
        }
    }
}



impl From<BetterUrl> for url::Url {fn from(value: BetterUrl) -> Self {value.url}}
impl From<BetterUrl> for String   {fn from(value: BetterUrl) -> Self {value.url.into()}}

impl PartialEq<str         > for BetterUrl {fn eq(&self, other: &str         ) -> bool {self.as_str() ==  other}}
impl PartialEq<&str        > for BetterUrl {fn eq(&self, other: &&str        ) -> bool {self.as_str() == *other}}
impl PartialEq<String      > for BetterUrl {fn eq(&self, other: &String      ) -> bool {self.as_str() ==  other}}
impl PartialEq<Cow<'_, str>> for BetterUrl {fn eq(&self, other: &Cow<'_, str>) -> bool {self.as_str() ==  other}}
impl PartialEq<url::Url    > for BetterUrl {fn eq(&self, other: &url::Url    ) -> bool {self.url      == *other}}

impl PartialEq<BetterUrl> for str          {fn eq(&self, other: &BetterUrl) -> bool {other == self}}
impl PartialEq<BetterUrl> for &str         {fn eq(&self, other: &BetterUrl) -> bool {other == self}}
impl PartialEq<BetterUrl> for String       {fn eq(&self, other: &BetterUrl) -> bool {other == self}}
impl PartialEq<BetterUrl> for Cow<'_, str> {fn eq(&self, other: &BetterUrl) -> bool {other == self}}
impl PartialEq<BetterUrl> for url::Url     {fn eq(&self, other: &BetterUrl) -> bool {other == self}}

impl PartialEq for BetterUrl {fn eq(&self, other: &Self) -> bool {self.url == other.url}}
impl Eq for BetterUrl {}



impl PartialOrd<str         > for BetterUrl {fn partial_cmp(&self, other: &str         ) -> Option<Ordering> {self.as_str().partial_cmp(   other)}}
impl PartialOrd<&str        > for BetterUrl {fn partial_cmp(&self, other: &&str        ) -> Option<Ordering> {self.as_str().partial_cmp(  *other)}}
impl PartialOrd<String      > for BetterUrl {fn partial_cmp(&self, other: &String      ) -> Option<Ordering> {self.as_str().partial_cmp(&**other)}}
impl PartialOrd<Cow<'_, str>> for BetterUrl {fn partial_cmp(&self, other: &Cow<'_, str>) -> Option<Ordering> {self.as_str().partial_cmp(&**other)}}
impl PartialOrd<url::Url    > for BetterUrl {fn partial_cmp(&self, other: &url::Url    ) -> Option<Ordering> {self.url     .partial_cmp(   other)}}

impl PartialOrd<BetterUrl> for str          {fn partial_cmp(&self, other: &BetterUrl) -> Option<Ordering> {   self .partial_cmp( other.as_str())}}
impl PartialOrd<BetterUrl> for &str         {fn partial_cmp(&self, other: &BetterUrl) -> Option<Ordering> {( *self).partial_cmp( other.as_str())}}
impl PartialOrd<BetterUrl> for String       {fn partial_cmp(&self, other: &BetterUrl) -> Option<Ordering> {(**self).partial_cmp( other.as_str())}}
impl PartialOrd<BetterUrl> for Cow<'_, str> {fn partial_cmp(&self, other: &BetterUrl) -> Option<Ordering> {(**self).partial_cmp( other.as_str())}}
impl PartialOrd<BetterUrl> for url::Url     {fn partial_cmp(&self, other: &BetterUrl) -> Option<Ordering> {   self .partial_cmp(&other.url     )}}

impl PartialOrd for BetterUrl {fn partial_cmp(&self, other: &Self) -> Option<Ordering> {Some(self.cmp(other))}}
impl Ord        for BetterUrl {fn cmp        (&self, other: &Self) ->        Ordering  {self.url.cmp(&other.url)}}



impl AsRef <str> for BetterUrl {fn as_ref(&self) -> &str {self.as_str()}}
impl Borrow<str> for BetterUrl {fn borrow(&self) -> &str {self.as_str()}}

impl AsRef <url::Url> for BetterUrl {fn as_ref(&self) -> &url::Url {&self.url}}
impl Borrow<url::Url> for BetterUrl {fn borrow(&self) -> &url::Url {&self.url}}



impl Hash for BetterUrl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state)
    }
}



impl Display for BetterUrl {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.url)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for BetterUrl {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        url::Url::deserialize(deserializer).map(Into::into)
    }
}

#[cfg(feature = "serde")]
impl Serialize for BetterUrl {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.url.serialize(serializer)
    }
}
