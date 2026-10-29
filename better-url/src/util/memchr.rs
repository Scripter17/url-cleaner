//! Memchr.

/// A simple wrapper around [`libc::memchr`].
pub fn memchr(haystack: &[u8], needle: u8) -> Option<usize> {
    unsafe {libc::memchr(
        haystack.as_ptr() as _,
        needle as _,
        haystack.len()
    )}.addr().checked_sub(haystack.as_ptr().addr())
}

/// A simple wrapper around [`libc`]'s memrchr.
pub fn memrchr(haystack: &[u8], needle: u8) -> Option<usize> {
    cfg_select! {
        target_os = "linux" => {
            unsafe {libc::memrchr(
                haystack.as_ptr() as _,
                needle as _,
                haystack.len()
            )}.addr().checked_sub(haystack.as_ptr().addr())
        },
        _ => memchr::memrchr(needle, haystack)
    }
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
