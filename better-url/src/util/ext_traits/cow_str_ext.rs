//! [`CowStrExt`].

use crate::prelude::*;

/// Extension trait for `Cow<'_, str>`.
pub(crate) trait CowStrExt {
    /// Retrain the range without checking for validity.
    unsafe fn retain_range_unchecked<B: RangeBounds<usize>>(&mut self, range: B);

    /// Retain a substring using pointer arithmetic and no allocations.
    fn retain_substr(&mut self, substr: *const str);

    /// Retain a subslice with no allocations.
    fn retain_range<B: RangeBounds<usize>>(&mut self, range: B);

    /// Replace a range, trying to not allocate.
    fn replace_range<B: RangeBounds<usize>>(&mut self, range: B, with: &str);

    /// Replace a substring.
    fn replace_substr(&mut self, substr: *const str, with: &str);

    /// Insert multiple [`str`]s starting at `idx` without unnecessary allocations.
    fn insert_with<const N: usize>(&mut self, idx: usize, strings: [&str; N]);

    /// Extend with multiple [`str`]s without unnecessary allocations.
    fn extend<const N: usize>(&mut self, strings: [&str; N]);

    /// Insert a string at `idx` without unnecessary allocations.
    fn with_insert_str(self, idx: usize, string: &str) -> Cow<'static, str>;
}

impl CowStrExt for Cow<'_, str> {
    unsafe fn retain_range_unchecked<B: RangeBounds<usize>>(&mut self, range: B) {
        unsafe {
            match self {
                Cow::Owned(x) => {
                    match range.end_bound() {
                        Bound::Unbounded => {},
                        Bound::Excluded(&y) => x.as_mut_vec().set_len(y),
                        Bound::Included(&y) => x.as_mut_vec().set_len(y + 1),
                    }
                    match range.start_bound() {
                        Bound::Unbounded => {},
                        Bound::Excluded(&y) => {x.as_mut_vec().drain(..=y);},
                        Bound::Included(&y) => {x.as_mut_vec().drain(.. y);},
                    }
                },
                Cow::Borrowed(x) => *x = x.get_unchecked((range.start_bound().cloned(), range.end_bound().cloned()))
            }
        }
    }

    fn retain_substr(&mut self, substr: *const str) {
        self.retain_range(self.my_substr_range(substr));
    }

    fn retain_range<B: RangeBounds<usize>>(&mut self, range: B) {
        match self {
            Cow::Owned(x) => {
                match range.end_bound() {
                    Bound::Unbounded => {},
                    Bound::Excluded(&y) => x.truncate(y),
                    Bound::Included(&y) => x.truncate(y + 1),
                }
                match range.start_bound() {
                    Bound::Unbounded => {},
                    Bound::Excluded(&y) => {x.drain(..=y);},
                    Bound::Included(&y) => {x.drain(.. y);},
                }
            },
            Cow::Borrowed(x) => *x = &x[(range.start_bound().cloned(), range.end_bound().cloned())]
        }
    }

    fn replace_range<B: RangeBounds<usize>>(&mut self, range: B, with: &str) {
        match self {
            Cow::Owned(x) => x.replace_range(range, with),
            Cow::Borrowed(x) => {
                let start = match range.start_bound() {
                    Bound::Unbounded    => 0,
                    Bound::Included(&x) => x,
                    Bound::Excluded(&x) => x + 1,
                };

                let after = match range.end_bound() {
                    Bound::Unbounded    => x.len(),
                    Bound::Included(&x) => x + 1,
                    Bound::Excluded(&x) => x,
                };

                let mut ret = String::with_capacity(x.len() - (after - start) + with.len());

                ret.extend([&x[..start], with, &x[after..]]);

                *self = ret.into()
            }
        }
    }

    fn replace_substr(&mut self, substr: *const str, with: &str) {
        self.replace_range(self.my_substr_range(substr), with)
    }

    fn insert_with<const N: usize>(&mut self, idx: usize, strings: [&str; N]) {
        match self {
            Cow::Owned(x) => x.insert_with(idx, strings),
            Cow::Borrowed(x) => {
                let mut ret = String::with_capacity(x.len() + strings.into_iter().map(str::len).sum::<usize>());

                ret.push_str(&x[..idx]);
                ret.extend(strings);
                ret.push_str(&x[idx..]);

                *self = ret.into();
            }
        }
    }

    fn extend<const N: usize>(&mut self, strings: [&str; N]) {
        match self {
            Cow::Owned(x) => x.extend(strings),
            Cow::Borrowed(x) => {
                let mut ret = String::with_capacity(x.len() + strings.into_iter().map(str::len).sum::<usize>());

                ret.push_str(x);
                ret.extend(strings);

                *self = ret.into();
            }
        }
    }

    fn with_insert_str(self, idx: usize, string: &str) -> Cow<'static, str> {
        let mut ret = String::with_capacity(self.len() + string.len());

        ret.push_str(&self[..idx]);
        ret.push_str(string);
        ret.push_str(&self[idx..]);

        ret.into()
    }
}
