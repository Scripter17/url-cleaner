//! [`PercentEncodeSet`].

use crate::prelude::*;

/// A [`ByteSet`] with all non-ASCII bytes always set.
///
/// Mainly for use in [`percent_encode`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PercentEncodeSet(ByteSet);

impl PercentEncodeSet {
    /// Make a new [`Self`].
    pub const fn new() -> Self {
        Self(ByteSet::new().add_non_ascii())
    }

    /// If it contains `b`.
    pub const fn contains(self, b: u8) -> bool {
        self.0.contains(b)
    }



    /// Add `b` to the set.
    pub const fn add(self, b: u8) -> Self {
        Self(self.0.add(b))
    }

    /// [`Self::add`] each byte.
    pub const fn add_many(self, bs: &[u8]) -> Self {
        Self(self.0.add_many(bs))
    }



    /// Remove `b` from the set.
    /// # Panics
    /// If `b` is not ASCII, panics.
    pub const fn remove(self, b: u8) -> Self {
        assert!(b.is_ascii());

        Self(self.0.remove(b))
    }

    /// [`Self::remove`] each byte.
    /// # Panics
    /// If `bs` is not ASCII, panics.
    pub const fn remove_many(self, bs: &[u8]) -> Self {
        assert!(bs.is_ascii());

        Self(self.0.remove_many(bs))
    }



    /// Merge `self` and `other`.
    pub const fn merge(self, other: Self) -> Self {
        Self(self.0.merge(other.0))
    }

    /// Get the inner [`ByteSet`].
    pub const fn into_inner(self) -> ByteSet {
        self.0
    }

    /// Make a new [`Self`] from a [`ByteSet`] without validity checks.
    /// # Safety
    /// All bytes in the range `128..=255` must be in the [`ByteSet`].
    pub const fn new_unchecked(value: ByteSet) -> Self {
        Self(value)
    }
}

impl Default for PercentEncodeSet {
    fn default() -> Self {
        Self::new()
    }
}

impl From<PercentEncodeSet> for ByteSet {
    fn from(value: PercentEncodeSet) -> Self {
        value.0
    }
}

impl TryFrom<ByteSet> for PercentEncodeSet {
    type Error = ByteSet;

    fn try_from(value: ByteSet) -> Result<Self, Self::Error> {
        if !(128..=255).all(|b| value.contains(b)) {
            Err(value)?;
        }

        Ok(Self(value))
    }
}

/// [The C0 control percent-encode set](https://url.spec.whatwg.org/#c0-control-percent-encode-set).
pub const C0: PercentEncodeSet = PercentEncodeSet::new().add_many(b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F\x7F");

/// [The component percent-encode set](https://url.spec.whatwg.org/#component-percent-encode-set).
pub const COMPONENT: PercentEncodeSet = USERINFO.add_many(b"$&+,");

/// [The userinfo percent-encode set](https://url.spec.whatwg.org/#userinfo-percent-encode-set).
pub const USERINFO                          : PercentEncodeSet = PATH.add_many(b"/:;=@[\\]|");
/// The opaque host percent-encode set. Equal to [`C0`].
pub const OPAQUE_HOST                       : PercentEncodeSet = C0;

/// [`C0`].
pub const OPAQUE_PATH                       : PercentEncodeSet = C0;
/// [The path percent-encode set](https://url.spec.whatwg.org/#application-x-www-form-urlencoded-percent-encode-set).
pub const PATH                              : PercentEncodeSet = NON_SPECIAL_QUERY.add_many(b"?^`{}");
/// [`PATH`] + `/` and `%`.
pub const PATH_SEGMENT                      : PercentEncodeSet = PATH.add_many(b"/%");

/// [The application/x-www-form-urlencoded percent-encode set](https://url.spec.whatwg.org/#application-x-www-form-urlencoded-percent-encode-set).
pub const QUERY_PART                        : PercentEncodeSet = COMPONENT.add_many(b"!'()~");

/// [The query percent-encode set](https://url.spec.whatwg.org/#query-percent-encode-set).
pub const NON_SPECIAL_QUERY                 : PercentEncodeSet = C0.add_many(b" \"#<>");
/// [`NON_SPECIAL_QUERY`] + `&`.
pub const NON_SPECIAL_QUERY_SEGMENT         : PercentEncodeSet = NON_SPECIAL_QUERY.add(b'&');

/// [The special query percent-encode set](https://url.spec.whatwg.org/#special-query-percent-encode-set).
pub const SPECIAL_QUERY                     : PercentEncodeSet = NON_SPECIAL_QUERY.add_many(b"'");
/// [`SPECIAL_QUERY`] + `&`.
pub const SPECIAL_QUERY_SEGMENT             : PercentEncodeSet = SPECIAL_QUERY.add(b'&');

/// [The fragment percent-encode set](https://url.spec.whatwg.org/#application-x-www-form-urlencoded-percent-encode-set)
pub const FRAGMENT                          : PercentEncodeSet = C0.add_many(b" \"<>`");
/// [`FRAGMENT`] + `&`.
pub const FRAGMENT_QUERY_SEGMENT            : PercentEncodeSet = FRAGMENT.add(b'&');
