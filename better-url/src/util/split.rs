//! Splitters.

use crate::prelude::*;

/// Generate a more efficient [`std::str::Split`].
macro_rules! gen_splitter {
    ($name:ident, $byte:literal, $str:literal) => {
        #[doc = concat!("Split a [`str`] on `", $str, "` more efficiently than [`std::str::Split`].")]
        #[repr(transparent)]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name<'a>(pub Option<&'a str>);

        impl<'a> $name<'a> {
            /// The remainder.
            pub fn remainder(&self) -> Option<&'a str> {
                self.0
            }

            /// The range of the remainder.
            pub fn range<B: RangeBounds<isize>>(&self, range: B) -> Option<&'a str> {
                let remainder = self.remainder()?;

                let start = match range.start_bound() {
                    Bound::Unbounded    => self.clone().next()?,
                    Bound::Excluded(-1) => None?,
                    Bound::Excluded(&x) => self.clone().neg_nth(x + 1)?,
                    Bound::Included(&x) => self.clone().neg_nth(x)?,
                }.addr() - remainder.addr();

                let after = match range.end_bound() {
                    Bound::Unbounded    => self.clone().next_back()?,
                    Bound::Excluded(&0) => None?,
                    Bound::Excluded(&x) => self.clone().neg_nth(x - 1)?,
                    Bound::Included(&x) => self.clone().neg_nth(x)?,
                }.end_addr() - remainder.addr();

                if after < start {
                    None?;
                }

                Some(unsafe {remainder.get_unchecked(start..after)})
            }
        }

        impl<'a> Iterator for $name<'a> {
            type Item = &'a str;

            fn next(&mut self) -> Option<Self::Item> {
                let remainder = self.remainder()?;
                match remainder.memchr($byte) {
                    Some(i) => unsafe {
                        let ret = remainder.get_unchecked(..i);
                        self.0 = Some(remainder.get_unchecked(i+1..));
                        Some(ret)
                    },
                    None => {
                        self.0 = None;
                        Some(remainder)
                    }
                }
            }
        }

        impl<'a> DoubleEndedIterator for $name<'a> {
            fn next_back(&mut self) -> Option<Self::Item> {
                let remainder = self.remainder()?;
                match remainder.memrchr($byte) {
                    Some(i) => unsafe {
                        let ret = remainder.get_unchecked(i+1..);
                        self.0 = Some(remainder.get_unchecked(..i));
                        Some(ret)
                    },
                    None => {
                        self.0 = None;
                        Some(remainder)
                    }
                }
            }
        }
    };
}

gen_splitter!(SplitDots      , b'.', ".");
gen_splitter!(SplitSlashes   , b'/', "/");
gen_splitter!(SplitAmpersands, b'&', "&");
