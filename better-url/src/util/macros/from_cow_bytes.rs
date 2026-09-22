//! [`from_cow_bytes`].

/// Impls for types that impl [`From`] of [`std::borrow::Cow`] of [`[u8]`].
macro_rules! from_cow_bytes {
    (root; $($t:ident),*) => {
        $(
            #[cfg(feature = "serde")]
            impl<'de> Deserialize<'de> for $t<'de> {
                fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    <Cow<'de, str>>::deserialize(deserializer).map(Into::into)
                }
            }

            impl<'a> $t<'a> {
                /// Make a new [`Self`].
                pub fn new<T: Into<Self>>(value: T) -> Self {
                    value.into()
                }
            }
        )*

        from_cow_bytes!($($t),*);
    };
    ($($t:ident),*) => {
        $(
            impl From<Vec<u8>> for $t<'static> {fn from(value: Vec<u8>) -> Self {Cow::<[u8]>::Owned(value).into()}}

            impl<'a                > From<&'a [u8   ]         > for $t<'a> {fn from(value: &'a [u8   ]         ) -> Self {Cow::Borrowed(   value           ).into()}}
            impl<'a, const N: usize> From<&'a [u8; N]         > for $t<'a> {fn from(value: &'a [u8; N]         ) -> Self {Cow::Borrowed(   value.as_slice()).into()}}
            impl<'a                > From<&'a Vec<u8>         > for $t<'a> {fn from(value: &'a Vec<u8>         ) -> Self {Cow::Borrowed(&**value           ).into()}}
            impl<'a                > From<&'a Cow<'_, [u8   ]>> for $t<'a> {fn from(value: &'a Cow<'_, [u8   ]>) -> Self {Cow::Borrowed(&**value           ).into()}}
            impl<'a, const N: usize> From<&'a Cow<'_, [u8; N]>> for $t<'a> {fn from(value: &'a Cow<'_, [u8; N]>) -> Self {Cow::Borrowed(   value.as_slice()).into()}}

            impl<'a> From<Cow<'a, str>> for $t<'a> {fn from(value: Cow<'a, str>) -> Self {cow_str_to_bytes(value).into()}}
            impl<'a> From<&'a str         > for $t<'a     > {fn from(value: &'a str         ) -> Self {Cow::Borrowed    (   value).into()}}
            impl     From<String          > for $t<'static> {fn from(value: String          ) -> Self {Cow::<str>::Owned(   value).into()}}
            impl<'a> From<&'a String      > for $t<'a     > {fn from(value: &'a String      ) -> Self {Cow::Borrowed    (&**value).into()}}
            impl<'a> From<&'a Cow<'_, str>> for $t<'a     > {fn from(value: &'a Cow<'_, str>) -> Self {Cow::Borrowed    (&**value).into()}}

            impl FromStr for $t<'static> {
                type Err = std::convert::Infallible;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Ok(s.to_string().into())
                }
            }
        )*
    };
}

pub(crate) use from_cow_bytes;
