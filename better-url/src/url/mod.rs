//! [`BetterUrl`].

use crate::prelude::*;

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
///
/// # Size
///
/// By using [`UrlString`] and [`DenseUrlDetails`] instead of [`String`] and [`UrlDetails`], [`BetterUrl`] takes, on 64 bit targets, just 64 bytes of stack space.
///
/// On 32 bit systems it takes even less but who cares.
///
/// ```
/// use better_url::prelude::*;
///
/// assert!(std::mem::size_of::<BetterUrl>() <= 64);
/// ```
///
/// # Canon APIs
///
/// The [URL spec](https://url.spec.whatwg.org/) specifies a number of APIs that do specific additional things that the normal type based APIs provided by this crate don't.
///
/// Primarily, this involves getters doing stupid things to avoid ever returning [`None`] and setters doing pointless pre-processing to compensate for javascript being awful.
///
/// Generally, you should stick to the non-canon APIs since they're just better, but if you need them the canon APIs are provided.
///
/// For example:
///
/// ```
/// use better_url::prelude::*;
///
/// let mut x = BetterUrl::new("https://example.com/abc").unwrap();
/// x.canon_set_hostname("example\t2.net/def").unwrap();
/// assert_eq!(x, "https://example2.net/abc");
///
/// let mut x = BetterUrl::new("https://example.com/abc").unwrap();
/// x.set_host("example\t3.net/def").unwrap_err();
/// ```
#[derive(Debug, Clone)]
pub struct BetterUrl {
    /** The serialization.       **/ serialization: UrlString    ,
    /** The [`DenseUrlDetails`]. **/ details      : DenseUrlDetails,
}

impl BetterUrl {
    /// Make a new [`Self`].
    /// # Errors
    /// If [`TryInto::try_into`] returns an error, that error is returned.
    pub fn new<T: TryInto<Self>>(value: T) -> Result<Self, T::Error> {
        value.try_into()
    }

    /// Make a new [`Self`] without doing any validity checks.
    /// # Safety
    /// `serialization` and `details` must be a valid output of [`Self::new`] and [`Self::into_parts`].
    pub unsafe fn from_parts(serialization: UrlString, details: DenseUrlDetails) -> Self {
        Self {serialization, details}
    }

    /// Turn into the inner [`String`] and [`DenseUrlDetails`].
    pub fn into_parts(self) -> (UrlString, DenseUrlDetails) {
        (self.serialization, self.details)
    }

    /// The [`DenseUrlDetails`].
    pub fn details(&self) -> DenseUrlDetails {
        self.details
    }

    /// The URL as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.serialization
    }

    /// If it cannot be a base.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// assert!(!BetterUrl::new("https://example.com/").unwrap().cannot_be_a_base());
    /// assert!(!BetterUrl::new("aaaaa://example.com/").unwrap().cannot_be_a_base());
    /// assert!(!BetterUrl::new("aaaaa://example.com" ).unwrap().cannot_be_a_base());
    /// assert!(!BetterUrl::new("aaaaa:/123"          ).unwrap().cannot_be_a_base());
    /// assert!( BetterUrl::new("aaaaa:123"           ).unwrap().cannot_be_a_base());
    /// assert!( BetterUrl::new("aaaaa:"              ).unwrap().cannot_be_a_base());
    /// ```
    pub fn cannot_be_a_base(&self) -> bool {
        !self.has_host() && unsafe {!self.as_str().get_unchecked(self.details.path_start as usize ..).starts_with('/')}
    }

    /// If it can be a base.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// assert!( BetterUrl::new("https://example.com/").unwrap().can_be_a_base());
    /// assert!( BetterUrl::new("aaaaa://example.com/").unwrap().can_be_a_base());
    /// assert!( BetterUrl::new("aaaaa://example.com" ).unwrap().can_be_a_base());
    /// assert!( BetterUrl::new("aaaaa:/123"          ).unwrap().can_be_a_base());
    /// assert!(!BetterUrl::new("aaaaa:123"           ).unwrap().can_be_a_base());
    /// assert!(!BetterUrl::new("aaaaa:"              ).unwrap().can_be_a_base());
    /// ```
    pub fn can_be_a_base(&self) -> bool {
        self.has_host() || unsafe {self.as_str().get_unchecked(self.details.path_start as usize ..).starts_with('/')}
    }

    /// The length.
    #[expect(clippy::len_without_is_empty, reason = "Can't be empty.")]
    pub fn len(&self) -> usize {
        self.serialization.len() as usize
    }
}



impl TryFrom<Cow<'_, [u8]>> for BetterUrl {
    type Error = InvalidUrl;

    fn try_from(value: Cow<'_, [u8]>) -> Result<Self, Self::Error> {
        let (_, value) = canonize_parser_input_bytes(value);

        let i = value.memchr(b':').ok_or(InvalidUrl::MissingScheme)?;

        let scheme = unsafe {value.get_unchecked(.. i       )};
        let rest   = unsafe {value.get_unchecked(   i + 1 ..)};

        Self::after_scheme(Scheme::new(scheme)?, rest)
    }
}

impl FromStr for BetterUrl {
    type Err = InvalidUrl;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.try_into()
    }
}

impl                 TryFrom<Cow<'_, str>> for BetterUrl {type Error = InvalidUrl; fn try_from(value: Cow<'_, str>) -> Result<Self, Self::Error> {cow_str_to_bytes(value).try_into()}}
impl                 TryFrom<&str        > for BetterUrl {type Error = InvalidUrl; fn try_from(value: &str        ) -> Result<Self, Self::Error> {Cow::Borrowed     (   value           ).try_into()}}
impl                 TryFrom< String     > for BetterUrl {type Error = InvalidUrl; fn try_from(value: String      ) -> Result<Self, Self::Error> {Cow::<str>::Owned (   value           ).try_into()}}
impl                 TryFrom<&String     > for BetterUrl {type Error = InvalidUrl; fn try_from(value: &String     ) -> Result<Self, Self::Error> {Cow::Borrowed     (&**value           ).try_into()}}
impl                 TryFrom<&[u8   ]    > for BetterUrl {type Error = InvalidUrl; fn try_from(value: &[u8   ]    ) -> Result<Self, Self::Error> {Cow::Borrowed     (   value           ).try_into()}}
impl<const N: usize> TryFrom<&[u8; N]    > for BetterUrl {type Error = InvalidUrl; fn try_from(value: &[u8; N]    ) -> Result<Self, Self::Error> {Cow::Borrowed     (   value.as_slice()).try_into()}}
impl                 TryFrom< Vec<u8>    > for BetterUrl {type Error = InvalidUrl; fn try_from(value: Vec<u8>     ) -> Result<Self, Self::Error> {Cow::<[u8]>::Owned(   value           ).try_into()}}
impl                 TryFrom<&Vec<u8>    > for BetterUrl {type Error = InvalidUrl; fn try_from(value: &Vec<u8>    ) -> Result<Self, Self::Error> {Cow::Borrowed     (&**value           ).try_into()}}



impl From<BetterUrl> for String            {fn from(value: BetterUrl) -> Self {           value.serialization.into() }}
impl From<BetterUrl> for Cow<'static, str> {fn from(value: BetterUrl) -> Self {Cow::Owned(value.serialization.into())}}

impl AsRef <str> for BetterUrl {fn as_ref(&self) -> &str {self.as_str()}}
impl Borrow<str> for BetterUrl {fn borrow(&self) -> &str {self.as_str()}}

impl AsRef <UrlString> for BetterUrl {fn as_ref(&self) -> &UrlString {&self.serialization}}
impl Borrow<UrlString> for BetterUrl {fn borrow(&self) -> &UrlString {&self.serialization}}



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



impl PartialOrd for BetterUrl {fn partial_cmp(&self, other: &Self) -> Option<Ordering> {Some(self.cmp(other))}}
impl Ord        for BetterUrl {fn cmp        (&self, other: &Self) ->        Ordering  {self.as_str().cmp(other.as_str())}}

impl PartialOrd< str         > for BetterUrl {fn partial_cmp(&self, other: & str         ) -> Option<Ordering> {self.as_str().partial_cmp(    other)}}
impl PartialOrd< String      > for BetterUrl {fn partial_cmp(&self, other: & String      ) -> Option<Ordering> {self.as_str().partial_cmp(& **other)}}
impl PartialOrd< Cow<'_, str>> for BetterUrl {fn partial_cmp(&self, other: & Cow<'_, str>) -> Option<Ordering> {self.as_str().partial_cmp(& **other)}}
impl PartialOrd<&str         > for BetterUrl {fn partial_cmp(&self, other: &&str         ) -> Option<Ordering> {self.as_str().partial_cmp(   *other)}}
impl PartialOrd<&String      > for BetterUrl {fn partial_cmp(&self, other: &&String      ) -> Option<Ordering> {self.as_str().partial_cmp(&***other)}}
impl PartialOrd<&Cow<'_, str>> for BetterUrl {fn partial_cmp(&self, other: &&Cow<'_, str>) -> Option<Ordering> {self.as_str().partial_cmp(&***other)}}

impl PartialOrd<BetterUrl> for  str          {fn partial_cmp(&self, other: &BetterUrl    ) -> Option<Ordering> {other.partial_cmp(self)}}
impl PartialOrd<BetterUrl> for  String       {fn partial_cmp(&self, other: &BetterUrl    ) -> Option<Ordering> {other.partial_cmp(self)}}
impl PartialOrd<BetterUrl> for  Cow<'_, str> {fn partial_cmp(&self, other: &BetterUrl    ) -> Option<Ordering> {other.partial_cmp(self)}}
impl PartialOrd<BetterUrl> for &str          {fn partial_cmp(&self, other: &BetterUrl    ) -> Option<Ordering> {other.partial_cmp(self)}}
impl PartialOrd<BetterUrl> for &String       {fn partial_cmp(&self, other: &BetterUrl    ) -> Option<Ordering> {other.partial_cmp(self)}}
impl PartialOrd<BetterUrl> for &Cow<'_, str> {fn partial_cmp(&self, other: &BetterUrl    ) -> Option<Ordering> {other.partial_cmp(self)}}



impl Hash for BetterUrl {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.as_str().hash(hasher)
    }
}

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
