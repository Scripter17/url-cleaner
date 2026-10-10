//! [`try_from_cow_bytes`].

/// Impls for types that impl [`TryFrom`] of [`std::borrow::Cow`] of [`[u8]`].
macro_rules! try_from_cow_bytes {
    ($($t:ident),*) => {
        $(
            impl<'a> TryFrom<Cow<'a, str>> for $t<'a> {
                type Error = <Self as TryFrom<Cow<'a, [u8]>>>::Error;

                fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
                    cow_str_to_bytes(value).try_into()
                }
            }

            try_from_cow_bytes_or_str!($t);
        )*
    };
}

pub(crate) use try_from_cow_bytes;
