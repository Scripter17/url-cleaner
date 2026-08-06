//! Memchr.

use crate::prelude::*;



/// A simple wrapper around [`libc`]'s memchr.
pub fn memchr(bytes: &[u8], byte: u8) -> Option<usize> {
    bytes.memchr(byte)
}

/// A simple wrapper around [`libc`]'s memrchr.
pub fn memrchr(bytes: &[u8], byte: u8) -> Option<usize> {
    bytes.memrchr(byte)
}

/// Like [`memchr`] but searches for the first of a set of bytes.
///
/// `neeldes` should be ordered in descending likiness of each needle being first.
///
/// This is because it calls [`memchr`] in a loop.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(memchrn(b"a1b2c3", *b"123"), Some(1));
/// assert_eq!(memchrn(b"a1b2c3", *b"231"), Some(1));
/// assert_eq!(memchrn(b"a1b2c3", *b"312"), Some(1));
/// ```
pub fn memchrn<const N: usize>(haystack: &[u8], needles: [u8; N]) -> Option<usize> {
    haystack.memchrn(needles)
}

/// Like [`memrchr`] but searches for the first of a set of bytes.
///
/// `neeldes` should be ordered in descending likiness of each needle being last.
///
/// This is because it calls [`memrchr`] in a loop.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(memrchrn(b"a1b2c3", *b"123"), Some(5));
/// assert_eq!(memrchrn(b"a1b2c3", *b"231"), Some(5));
/// assert_eq!(memrchrn(b"a1b2c3", *b"312"), Some(5));
/// ```
pub fn memrchrn<const N: usize>(haystack: &[u8], needles: [u8; N]) -> Option<usize> {
    haystack.memrchrn(needles)
}



/// An [`Iterator`] of each [`memchr`].
#[derive(Debug)]
pub struct MemchrIter<'a> {
    /// The remainder.
    pub remainder: Option<&'a [u8]>,
    /// The byte to search for.
    pub byte: u8,
}

impl<'a> Iterator for MemchrIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder?;

        match remainder.memchr(self.byte) {
            Some(i) => {
                self.remainder = Some(unsafe {remainder.get_unchecked(i + 1 ..)});
                Some(i)
            },
            None => {
                self.remainder = None;
                None
            }
        }
    }
}

impl<'a> DoubleEndedIterator for MemchrIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder?;

        match remainder.memrchr(self.byte) {
            Some(i) => {
                self.remainder = Some(unsafe {remainder.get_unchecked(.. i)});
                Some(i)
            },
            None => {
                self.remainder = None;
                None
            }
        }
    }
}



/// An [`Iterator`] of each [`memchrn`].
#[derive(Debug)]
pub struct MemchrnIter<'a, const N: usize> {
    /// The remainder.
    pub remainder: Option<&'a [u8]>,
    /// The bytes to search for.
    pub bytes: [u8; N]
}

impl<'a, const N: usize> Iterator for MemchrnIter<'a, N> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder?;

        match remainder.memchrn(self.bytes) {
            Some(i) => {
                self.remainder = Some(unsafe {remainder.get_unchecked(i + 1 ..)});
                Some(i)
            },
            None => {
                self.remainder = None;
                None
            }
        }
    }
}

impl<'a, const N: usize> DoubleEndedIterator for MemchrnIter<'a, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder?;

        match remainder.memrchrn(self.bytes) {
            Some(i) => {
                self.remainder = Some(unsafe {remainder.get_unchecked(.. i)});
                Some(i)
            },
            None => {
                self.remainder = None;
                None
            }
        }
    }
}



/// An [`Iterator`] of of bytes split on each [`memchr`].
#[derive(Debug)]
pub struct MemchrSplit<'a> {
    /// The remainder.
    pub remainder: Option<&'a [u8]>,
    /// The byte to search for.
    pub byte: u8,
}

impl<'a> Iterator for MemchrSplit<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder?;

        match remainder.memchr(self.byte) {
            Some(i) => {
                self.remainder = Some(unsafe {remainder.get_unchecked(i + 1 ..)});
                Some(unsafe {remainder.get_unchecked(.. i)})
            },
            None => {
                self.remainder = None;
                Some(remainder)
            },
        }
    }
}

impl<'a> DoubleEndedIterator for MemchrSplit<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder?;

        match remainder.memrchr(self.byte) {
            Some(i) => {
                self.remainder = Some(unsafe {remainder.get_unchecked(.. i)});
                Some(unsafe {remainder.get_unchecked(i + 1 ..)})
            },
            None => {
                self.remainder = None;
                Some(remainder)
            },
        }
    }
}



/// An [`Iterator`] of of bytes split on each [`memchrn`].
#[derive(Debug)]
pub struct MemchrnSplit<'a, const N: usize> {
    /// The remainder.
    pub remainder: Option<&'a [u8]>,
    /// The bytes to search for.
    pub bytes: [u8; N],
}

impl<'a, const N: usize> Iterator for MemchrnSplit<'a, N> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder?;

        match remainder.memchrn(self.bytes) {
            Some(i) => {
                self.remainder = Some(unsafe {remainder.get_unchecked(i + 1 ..)});
                Some(unsafe {remainder.get_unchecked(.. i)})
            },
            None => {
                self.remainder = None;
                Some(remainder)
            },
        }
    }
}

impl<'a, const N: usize> DoubleEndedIterator for MemchrnSplit<'a, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder?;

        match remainder.memrchrn(self.bytes) {
            Some(i) => {
                self.remainder = Some(unsafe {remainder.get_unchecked(.. i)});
                Some(unsafe {remainder.get_unchecked(i + 1 ..)})
            },
            None => {
                self.remainder = None;
                Some(remainder)
            },
        }
    }
}



/// An [`Iterator`] of bytes split on `\r\n`/`\n`.
#[derive(Debug)]
pub struct MemchrLines<'a> {
    /// The remainder.
    pub remainder: Option<&'a [u8]>,
}

impl<'a> Iterator for MemchrLines<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder?;

        match remainder.memchr(b'\n') {
            Some(i) => {
                self.remainder = Some(unsafe {remainder.get_unchecked(i + 1 ..)});
                Some(match unsafe {remainder.get_unchecked(.. i)} {
                    [line @ .., b'\r'] | line => line
                })
            },
            None => {
                self.remainder = None;
                Some(match remainder {
                    [line @ .., b'\r'] | line => line
                })
            }
        }
    }
}

impl<'a> DoubleEndedIterator for MemchrLines<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder?;

        match remainder.memrchr(b'\n') {
            Some(i) => {
                self.remainder = Some(unsafe {remainder.get_unchecked(.. i)});
                Some(match unsafe {remainder.get_unchecked(i + 1 ..)} {
                    [line @ .., b'\r'] | line => line
                })
            },
            None => {
                self.remainder = None;
                Some(match remainder {
                    [line @ .., b'\r'] | line => line
                })
            }
        }
    }
}
