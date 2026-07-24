//! [`AsciiSet`].

use crate::prelude::*;

/// A set of ASCII codepoints for use in [`percent_encode`].
///
/// Please note that, for performance reasons, this is implemented as a `[bool; 256]` where the last 128 [`bool`]s are always [`true`].
///
/// This is extremely stupid, but it is faster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AsciiSet([bool; 256]);

impl AsciiSet {
    /// Make a new [`Self`].
    pub const fn new() -> Self {
        let mut ret = [false; 256];
        let mut i = 128;
        while i < 256 {
            ret[i] = true;
            i += 1
        }
        Self(ret)
    }

    /// If it contains `b`.
    pub fn contains(self, b: u8) -> bool {
        self.0[b as usize]
    }

    /// Add `b` to the set.
    pub const fn add(mut self, b: u8) -> Self {
        self.0[b as usize] = true;
        self
    }

    /// Remove `b` from the set.
    /// # Panics
    /// If `b` is not ASCII, panics.
    pub const fn remove(mut self, b: u8) -> Self {
        assert!(b.is_ascii());

        self.0[b as usize] = false;

        self
    }

    /// [`Self::add`] each byte.
    pub const fn add_many(self, bs: &[u8]) -> Self {
        match bs.split_first() {
            Some((&b, bs)) => self.add(b).add_many(bs),
            None => self
        }
    }

    /// Merge `self` and `other`.
    pub const fn merge(mut self, other: Self) -> Self {
        let mut i = 0;
        while i < 256 {
            self.0[i] |= other.0[i];
            i += 1;
        }
        self
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

/// The ASCII part of [The forbidden host code point set](https://url.spec.whatwg.org/#forbidden-host-code-point).
pub const FORBIDDEN_HOST                    : AsciiSet = AsciiSet::new().add_many(b"\x00\t\n\r #/:<>?@[\\]^|");
/// [`FORBIDDEN_DOMAIN_SEGMENTS`] plus `.`.
pub const FORBIDDEN_DOMAIN_SEGMENT          : AsciiSet = FORBIDDEN_DOMAIN_SEGMENTS.add(b'.');
/// [`FORBIDDEN_HOST`] plus [`C0`] and `%`.
pub const FORBIDDEN_DOMAIN_SEGMENTS         : AsciiSet = FORBIDDEN_HOST.merge(C0).add(b'%');
/// The ASCII part of [the forbidden domain code point set](https://url.spec.whatwg.org/#application-x-www-form-urlencoded-percent-encode-set).
pub const FORBIDDEN_DOMAIN_HOST             : AsciiSet = FORBIDDEN_DOMAIN_SEGMENTS;
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
/// The set of characters to percent-encode when converting a [`NonSpecialQuery`] into a [`SpecialQuery`].
pub const NON_SPECIAL_QUERY_TO_SPECIAL_QUERY: AsciiSet = AsciiSet::new().add_many(b"'");
/// The set of characters to percent-encode when converting a [`NonSpecialQuery`] into a [`FragmentQuery`].
pub const NON_SPECIAL_QUERY_TO_FRAGMENT     : AsciiSet = AsciiSet::new().add_many(b"`");

/// [The special query percent-encode set](https://url.spec.whatwg.org/#special-query-percent-encode-set).
pub const SPECIAL_QUERY                     : AsciiSet = NON_SPECIAL_QUERY.add_many(b"'");
/// [`SPECIAL_QUERY`] + `&`.
pub const SPECIAL_QUERY_SEGMENT             : AsciiSet = SPECIAL_QUERY.add(b'&');
/// The set of characters to percent-encode when converting a [`SpecialQuery`] into a [`FragmentQuery`].
pub const SPECIAL_QUERY_TO_FRAGMENT         : AsciiSet = AsciiSet::new().add_many(b"`");

/// [The fragment percent-encode set](https://url.spec.whatwg.org/#application-x-www-form-urlencoded-percent-encode-set)
pub const FRAGMENT                          : AsciiSet = C0.add_many(b" \"<>`");
/// [`FRAGMENT`] + `&`.
pub const FRAGMENT_QUERY_SEGMENT            : AsciiSet = FRAGMENT.add(b'&');
/// The set of characters to percent-encode when converting a [`FragmentQuery`] into a [`NonSpecialQuery`].
pub const FRAGMENT_TO_NON_SPECIAL_QUERY     : AsciiSet = AsciiSet::new().add_many(b"#");
/// The set of characters to percent-encode when converting a [`FragmentQuery`] into a [`SpecialQuery`].
pub const FRAGMENT_TO_SPECIAL_QUERY         : AsciiSet = AsciiSet::new().add_many(b"#'");
