//! [`AsciiSet`].

use crate::prelude::*;

/// A [`ByteSet`] with all non-ASCII bytes always set.
///
/// Mainly for use in [`percent_encode`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AsciiSet(ByteSet);

impl AsciiSet {
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
    pub const fn to_byte_set(self) -> ByteSet {
        self.0
    }
}

impl Default for AsciiSet {
    fn default() -> Self {
        Self::new()
    }
}

/// [The C0 control percent-encode set](https://url.spec.whatwg.org/#c0-control-percent-encode-set).
pub const C0: AsciiSet = AsciiSet::new().add_many(b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F\x7F");

/// [The component percent-encode set](https://url.spec.whatwg.org/#component-percent-encode-set).
pub const COMPONENT: AsciiSet = USERINFO.add_many(b"$&+,");

/// [The userinfo percent-encode set](https://url.spec.whatwg.org/#userinfo-percent-encode-set).
pub const USERINFO                          : AsciiSet = PATH.add_many(b"/:;=@[\\]|");
/// The opaque host percent-encode set. Equal to [`C0`].
pub const OPAQUE_HOST                       : AsciiSet = C0;

/// [`C0`].
pub const OPAQUE_PATH                       : AsciiSet = C0;
/// [The path percent-encode set](https://url.spec.whatwg.org/#application-x-www-form-urlencoded-percent-encode-set).
pub const PATH                              : AsciiSet = NON_SPECIAL_QUERY.add_many(b"?^`{}");
/// [`PATH`] + `/`.
pub const PATH_SEGMENT                      : AsciiSet = PATH.add(b'/');

/// [The application/x-www-form-urlencoded percent-encode set](https://url.spec.whatwg.org/#application-x-www-form-urlencoded-percent-encode-set).
pub const QUERY_PART                        : AsciiSet = COMPONENT.add_many(b"!'()~");

/// [The query percent-encode set](https://url.spec.whatwg.org/#query-percent-encode-set).
pub const NON_SPECIAL_QUERY                 : AsciiSet = C0.add_many(b" \"#<>");
/// [`NON_SPECIAL_QUERY`] + `&`.
pub const NON_SPECIAL_QUERY_SEGMENT         : AsciiSet = NON_SPECIAL_QUERY.add(b'&');

/// [The special query percent-encode set](https://url.spec.whatwg.org/#special-query-percent-encode-set).
pub const SPECIAL_QUERY                     : AsciiSet = NON_SPECIAL_QUERY.add_many(b"'");
/// [`SPECIAL_QUERY`] + `&`.
pub const SPECIAL_QUERY_SEGMENT             : AsciiSet = SPECIAL_QUERY.add(b'&');

/// [The fragment percent-encode set](https://url.spec.whatwg.org/#application-x-www-form-urlencoded-percent-encode-set)
pub const FRAGMENT                          : AsciiSet = C0.add_many(b" \"<>`");
/// [`FRAGMENT`] + `&`.
pub const FRAGMENT_QUERY_SEGMENT            : AsciiSet = FRAGMENT.add(b'&');
