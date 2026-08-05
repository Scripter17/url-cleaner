//! [`ByteSet`].

use crate::prelude::*;

/// A set of bytes.
///
/// Please note that, while using `[bool; 256]` *is* stupid, it is also faster.
///
/// This is extremely stupid, but it is faster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ByteSet(pub [bool; 256]);

impl ByteSet {
    /// A new all-empty [`Self`].
    pub const fn new() -> Self {
        Self([false; 256])
    }
    
    /// If it contains `b`.
    pub const fn contains(self, b: u8) -> bool {
        self.0[b as usize]
    }



    /// Add `b` to the set.
    pub const fn add(mut self, b: u8) -> Self {
        self.0[b as usize] = true;
        self
    }

    /// [`Self::add`] each byte.
    pub const fn add_many(mut self, bs: &[u8]) -> Self {
        let mut i = 0;

        while i < bs.len() {
            self = self.add(bs[i]);
            i += 1;
        }

        self
    }



    /// Remove `b` from the set.
    pub const fn remove(mut self, b: u8) -> Self {
        self.0[b as usize] = false;

        self
    }

    /// [`Self::remove`] each byte.
    pub const fn remove_many(mut self, bs: &[u8]) -> Self {
        let mut i = 0;

        while i < bs.len() {
            self = self.remove(bs[i]);
            i += 1;
        }

        self
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

    /// Invert the set.
    pub const fn invert(mut self) -> Self {
        let mut i = 0;
        while i < 256 {
            self.0[i] = !self.0[i];
            i += 1;
        }
        self
    }



    /// Add all non-ASCII bytes.
    pub const fn add_non_ascii(mut self) -> Self {
        let mut i = 128;
        while i < 256 {
            self.0[i] = true;
            i += 1;
        }
        self
    }

    /// Remove all non-ASCII bytes.
    pub const fn remove_non_ascii(mut self) -> Self {
        let mut i = 128;
        while i < 256 {
            self.0[i] = false;
            i += 1;
        }
        self
    }
}

impl Default for ByteSet {
    fn default() -> Self {
        Self::new()
    }
}

/** [The forbidden host code point set](https://url.spec.whatwg.org/#forbidden-host-code-point).                              **/ pub const FORBIDDEN_HOST_INPUT           : ByteSet = ByteSet::new().add_many(b"\x00\t\n\r #/:<>?@[\\]^|");
/** [`FORBIDDEN_DOMAIN_SEGMENTS_INPUT`] plus `.`.                                                                             **/ pub const FORBIDDEN_DOMAIN_SEGMENT_INPUT : ByteSet = FORBIDDEN_DOMAIN_SEGMENTS_INPUT.add(b'.');
/** [`FORBIDDEN_HOST_INPUT`] plus [`C0`] and `%`.                                                                             **/ pub const FORBIDDEN_DOMAIN_SEGMENTS_INPUT: ByteSet = FORBIDDEN_HOST_INPUT.merge(C0.to_byte_set().remove_non_ascii()).add(b'%');
/** [the forbidden domain code point set](https://url.spec.whatwg.org/#application-x-www-form-urlencoded-percent-encode-set). **/ pub const FORBIDDEN_DOMAIN_HOST_INPUT    : ByteSet = FORBIDDEN_DOMAIN_SEGMENTS_INPUT;

/** [`FORBIDDEN_HOST_INPUT`] with non-ASCII.            **/ pub const FORBIDDEN_HOST_LITERAL           : ByteSet = FORBIDDEN_HOST_INPUT           .add_non_ascii().add_many(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
/** [`FORBIDDEN_DOMAIN_SEGMENT_INPUT`] with non-ASCII.  **/ pub const FORBIDDEN_DOMAIN_SEGMENT_LITERAL : ByteSet = FORBIDDEN_DOMAIN_SEGMENT_INPUT .add_non_ascii().add_many(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
/** [`FORBIDDEN_DOMAIN_SEGMENTS_INPUT`] with non-ASCII. **/ pub const FORBIDDEN_DOMAIN_SEGMENTS_LITERAL: ByteSet = FORBIDDEN_DOMAIN_SEGMENTS_INPUT.add_non_ascii().add_many(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
/** [`FORBIDDEN_DOMAIN_HOST_INPUT`] with non-ASCII.     **/ pub const FORBIDDEN_DOMAIN_HOST_LITERAL    : ByteSet = FORBIDDEN_DOMAIN_HOST_INPUT    .add_non_ascii().add_many(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
