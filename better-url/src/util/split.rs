//! Splitters.

/// Generate a more efficient [`std::str::Split`].
macro_rules! gen_splitter {
    ($name:ident, $byte:literal, $str:literal) => {
        #[doc = concat!("Split a [`str`] on `", $str, "` more efficiently than [`std::str::Split`].")]
        ///
        /// See [rust-lang/rust#158229](https://github.com/rust-lang/rust/issues/158229) for details.
        #[repr(transparent)]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name<'a>(pub Option<&'a str>);

        impl<'a> Iterator for $name<'a> {
            type Item = &'a str;

            fn next(&mut self) -> Option<Self::Item> {
                let a = self.0?;
                match a.bytes().position(|b| b == $byte) {
                    Some(i) => unsafe {
                        let ret = a.get_unchecked(..i);
                        self.0 = Some(a.get_unchecked(i+1..));
                        Some(ret)
                    },
                    None => {
                        self.0 = None;
                        Some(a)
                    }
                }
            }
        }

        impl<'a> DoubleEndedIterator for $name<'a> {
            fn next_back(&mut self) -> Option<Self::Item> {
                let a = self.0?;
                match a.bytes().rposition(|b| b == $byte) {
                    Some(i) => unsafe {
                        let ret = a.get_unchecked(i+1..);
                        self.0 = Some(a.get_unchecked(..i));
                        Some(ret)
                    },
                    None => {
                        self.0 = None;
                        Some(a)
                    }
                }
            }
        }
    };
}

gen_splitter!(SplitDots      , b'.', ".");
gen_splitter!(SplitSlashes   , b'/', "/");
gen_splitter!(SplitAmpersands, b'&', "&");
