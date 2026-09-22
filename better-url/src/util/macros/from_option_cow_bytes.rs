//! [`from_option_cow_bytes`].

/// Impls for types that impl [`From`] of [`Option`] of [`std::borrow::Cow`] of [`[u8]`].
macro_rules! from_option_cow_bytes {
    (root; $($t:ident),*) => {
        $(
            #[cfg(feature = "serde")]
            impl<'de> Deserialize<'de> for $t<'de> {
                fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    <Option<Cow<'de, str>>>::deserialize(deserializer).map(Into::into)
                }
            }

            impl<'a> $t<'a> {
                /// Make a new [`Self`].
                pub fn new<T: Into<Self>>(value: T) -> Self {
                    value.into()
                }
            }
        )*

        from_option_cow_bytes!($($t),*);
    };
    ($($t:ident),*) => {
        $(
            impl<'a> From<Cow<'a, [u8]>> for $t<'a> {fn from(value: Cow<'a, [u8]>) -> Self {Some(value).into()}}

            impl<'a                > From<Option<&'a [u8   ]         >> for $t<'a> {fn from(value: Option<&'a [u8   ]>         ) -> Self {value.map(Cow::Borrowed).into()}}
            impl<'a, const N: usize> From<Option<&'a [u8; N]         >> for $t<'a> {fn from(value: Option<&'a [u8; N]>         ) -> Self {value.map(|x| Cow::Borrowed(   x.as_slice())).into()}}
            impl<'a                > From<Option<&'a Vec<u8>         >> for $t<'a> {fn from(value: Option<&'a Vec<u8>>         ) -> Self {value.map(|x| Cow::Borrowed(&**x           )).into()}}
            impl<'a                > From<Option<&'a Cow<'_, [u8   ]>>> for $t<'a> {fn from(value: Option<&'a Cow<'_, [u8   ]>>) -> Self {value.map(|x| Cow::Borrowed(&**x           )).into()}}
            impl<'a, const N: usize> From<Option<&'a Cow<'_, [u8; N]>>> for $t<'a> {fn from(value: Option<&'a Cow<'_, [u8; N]>>) -> Self {value.map(|x| Cow::Borrowed(   x.as_slice())).into()}}

            impl<'a                > From<&'a Option<&[u8   ]        >> for $t<'a> {fn from(value: &'a Option<&[u8   ]>        ) -> Self {value.as_deref().into()}}
            impl<'a, const N: usize> From<&'a Option<&[u8; N]        >> for $t<'a> {fn from(value: &'a Option<&[u8; N]>        ) -> Self {value.as_deref().into()}}
            impl<'a                > From<&'a Option<Vec<u8>         >> for $t<'a> {fn from(value: &'a Option<Vec<u8>>         ) -> Self {value.as_deref().into()}}
            impl<'a                > From<&'a Option<Cow<'_, [u8   ]>>> for $t<'a> {fn from(value: &'a Option<Cow<'_, [u8   ]>>) -> Self {value.as_deref().into()}}
            impl<'a, const N: usize> From<&'a Option<Cow<'_, [u8; N]>>> for $t<'a> {fn from(value: &'a Option<Cow<'_, [u8; N]>>) -> Self {value.as_deref().into()}}

            impl<'a> From<Option<Cow<'a, str>>> for $t<'a> {
                fn from(value: Option<Cow<'a, str>>) -> Self {
                    value.map(cow_str_to_bytes).into()
                }
            }

            impl<'a> From<Option<&'a str         >> for $t<'a     > {fn from(value: Option<&'a str         >) -> Self {value.map(Cow::Borrowed    ).into()}}
            impl     From<Option<String          >> for $t<'static> {fn from(value: Option<String          >) -> Self {value.map(Cow::<str>::Owned).into()}}
            impl<'a> From<Option<&'a String      >> for $t<'a     > {fn from(value: Option<&'a String      >) -> Self {value.map(|x| Cow::Borrowed(&**x)).into()}}
            impl<'a> From<Option<&'a Cow<'_, str>>> for $t<'a     > {fn from(value: Option<&'a Cow<'_, str>>) -> Self {value.map(|x| Cow::Borrowed(&**x)).into()}}

            impl<'a> From<&'a Option<&str        >> for $t<'a     > {fn from(value: &'a Option<&str        >) -> Self {value.as_deref().into()}}
            impl<'a> From<&'a Option<String      >> for $t<'a     > {fn from(value: &'a Option<String      >) -> Self {value.as_deref().into()}}
            impl<'a> From<&'a Option<Cow<'_, str>>> for $t<'a     > {fn from(value: &'a Option<Cow<'_, str>>) -> Self {value.as_deref().into()}}
        )*

        from_cow_bytes!($($t),*);
    };
}

pub(crate) use from_option_cow_bytes;
