//! Memchr.

/// A simple wrapper around [`libc`]'s memchr.
pub fn memchr(haystack: &[u8], needle: u8) -> Option<usize> {
    match unsafe {libc::memchr(
        haystack.as_ptr() as _,
        needle as _,
        haystack.len()
    )}.addr() {
        0 => None,
        x => Some(x - haystack.as_ptr().addr())
    }
}

/// A simple wrapper around [`libc`]'s memrchr.
pub fn memrchr(haystack: &[u8], needle: u8) -> Option<usize> {
    cfg_select! {
        target_os = "linux" => {
            match unsafe {libc::memrchr(
                haystack.as_ptr() as _,
                needle as _,
                haystack.len()
            )}.addr() {
                0 => None,
                x => Some(x - haystack.as_ptr().addr())
            }
        },
        _ => memchr::memrchr(needle, haystack)
    }
}

/// Like [`memchr()`] but searches for the first of a set of needles.
///
/// `neeldes` should be ordered in descending likiness of each needle being first.
///
/// This is because it calls [`memchr()`] in a loop.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(memchrn(b"a1b2c3", b"123"), Some(1));
/// assert_eq!(memchrn(b"a1b2c3", b"231"), Some(1));
/// assert_eq!(memchrn(b"a1b2c3", b"312"), Some(1));
/// ```
pub fn memchrn(haystack: &[u8], needles: &[u8]) -> Option<usize> {
    let mut ret = haystack.len();

    for &needle in needles {
        let found = unsafe {libc::memchr(
            haystack.as_ptr() as _,
            needle as _,
            ret,
        )}.addr();

        if found != 0 {
            ret = found - haystack.as_ptr().addr();
        }
    }

    if ret == haystack.len() {
        None
    } else {
        Some(ret)
    }
}

/// Like [`memrchr`] but searches for the first of a set of needles.
///
/// `neeldes` should be ordered in descending likiness of each needle being last.
///
/// This is because it calls [`memrchr`] in a loop.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(memrchrn(b"a1b2c3", b"123"), Some(5));
/// assert_eq!(memrchrn(b"a1b2c3", b"231"), Some(5));
/// assert_eq!(memrchrn(b"a1b2c3", b"312"), Some(5));
/// ```
pub fn memrchrn(haystack: &[u8], needles: &[u8]) -> Option<usize> {
    let mut ret = None;
    let mut x = 0;

    for &needle in needles {
        if let Some(found) = memrchr(unsafe {haystack.get_unchecked(x..)}, needle) {
            x += found;
            ret = Some(x);
        }
    }

    ret
}



/// An [`Iterator`] of each [`memchr()`].
#[derive(Debug)]
pub struct MemchrIter<'a> {
    /// The remainder.
    pub remainder: Option<&'a [u8]>,
    /// The needle to search for.
    pub needle: u8,
}

impl<'a> Iterator for MemchrIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder?;

        match memchr(remainder, self.needle) {
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

        match memrchr(remainder, self.needle) {
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
pub struct MemchrnIter<'a> {
    /// The remainder.
    pub remainder: Option<&'a [u8]>,
    /// The needles to search for.
    pub needles: &'a [u8]
}

impl<'a> Iterator for MemchrnIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder?;

        match memchrn(remainder, self.needles) {
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

impl<'a> DoubleEndedIterator for MemchrnIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder?;

        match memrchrn(remainder, self.needles) {
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



/// An [`Iterator`] of bytes split on each [`memchr()`].
#[derive(Debug)]
pub struct MemchrSplit<'a> {
    /// The remainder.
    pub remainder: Option<&'a [u8]>,
    /// The needle to search for.
    pub needle: u8,
}

impl<'a> Iterator for MemchrSplit<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder?;

        match memchr(remainder, self.needle) {
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

        match memrchr(remainder, self.needle) {
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



/// An [`Iterator`] of bytes split on each [`memchrn`].
#[derive(Debug)]
pub struct MemchrnSplit<'a> {
    /// The remainder.
    pub remainder: Option<&'a [u8]>,
    /// The needles to search for.
    pub needles: &'a [u8],
}

impl<'a> Iterator for MemchrnSplit<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder?;

        match memchrn(remainder, self.needles) {
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

impl<'a> DoubleEndedIterator for MemchrnSplit<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder?;

        match memrchrn(remainder, self.needles) {
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

        match memchr(remainder, b'\n') {
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

        match memrchr(remainder, b'\n') {
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
