//! [`try_from_cow_bytes_or_str`].

/// Impls for types that impl [`TryFrom`] of [`std::borrow::Cow`] of [`[u8]`].
macro_rules! try_from_cow_bytes_or_str {
    ($($t:ident),*) => {
        $(
            #[cfg(feature = "serde")]
            impl<'de> Deserialize<'de> for $t<'de> {
                fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    <Cow<'de, str>>::deserialize(deserializer)?.try_into().map_err(D::Error::custom)
                }
            }

            impl<'a> $t<'a> {
                /// Make a new [`Self`].
                /// # Errors
                /// If [`TryInto::try_into`] returns an error, that error is returned.
                pub fn new<T: TryInto<Self>>(value: T) -> Result<Self, T::Error> {
                    value.try_into()
                }
            }

            impl FromStr for $t<'static> {
                type Err = <Self as TryFrom<Cow<'static, str>>>::Error;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    s.to_string().try_into()
                }
            }

            impl<'a> TryFrom<&'a str          > for $t<'a     > {type Error = <Self as TryFrom<Cow<'a     , str >>>::Error;fn try_from(value: &'a str          ) -> Result<Self, Self::Error> {Cow::<str> ::Borrowed(value).try_into()}}
            impl     TryFrom<    String       > for $t<'static> {type Error = <Self as TryFrom<Cow<'static, str >>>::Error;fn try_from(value:     String       ) -> Result<Self, Self::Error> {Cow::<str> ::Owned   (value).try_into()}}
            impl<'a> TryFrom<&'a String       > for $t<'a     > {type Error = <Self as TryFrom<Cow<'a     , str >>>::Error;fn try_from(value: &'a String       ) -> Result<Self, Self::Error> {Cow::<str> ::Borrowed(value).try_into()}}
            impl<'a> TryFrom<&'a Cow<'_, str> > for $t<'a     > {type Error = <Self as TryFrom<Cow<'a     , str >>>::Error;fn try_from(value: &'a Cow<'_, str> ) -> Result<Self, Self::Error> {Cow::<str> ::Borrowed(value).try_into()}}

            impl<'a                > TryFrom<&'a [u8]            > for $t<'a     > {type Error = <Self as TryFrom<Cow<'a     , [u8]>>>::Error;fn try_from(value: &'a [u8]            ) -> Result<Self, Self::Error> {Cow::<[u8]>::Borrowed(   value).try_into()}}
            impl<'a, const N: usize> TryFrom<&'a [u8; N]         > for $t<'a     > {type Error = <Self as TryFrom<Cow<'a     , [u8]>>>::Error;fn try_from(value: &'a [u8; N]         ) -> Result<Self, Self::Error> {Cow::<[u8]>::Borrowed(   value).try_into()}}
            impl                     TryFrom<    Vec<u8>         > for $t<'static> {type Error = <Self as TryFrom<Cow<'static, [u8]>>>::Error;fn try_from(value:     Vec<u8>         ) -> Result<Self, Self::Error> {Cow::<[u8]>::Owned   (   value).try_into()}}
            impl<'a                > TryFrom<&'a Vec<u8>         > for $t<'a     > {type Error = <Self as TryFrom<Cow<'a     , [u8]>>>::Error;fn try_from(value: &'a Vec<u8>         ) -> Result<Self, Self::Error> {Cow::<[u8]>::Borrowed(   value).try_into()}}
            impl<'a                > TryFrom<&'a Cow<'_, [u8]>   > for $t<'a     > {type Error = <Self as TryFrom<Cow<'a     , [u8]>>>::Error;fn try_from(value: &'a Cow<'_, [u8]>   ) -> Result<Self, Self::Error> {Cow::<[u8]>::Borrowed(   value).try_into()}}
            impl<'a, const N: usize> TryFrom<&'a Cow<'_, [u8; N]>> for $t<'a     > {type Error = <Self as TryFrom<Cow<'a     , [u8]>>>::Error;fn try_from(value: &'a Cow<'_, [u8; N]>) -> Result<Self, Self::Error> {Cow::<[u8]>::Borrowed(&**value).try_into()}}
        )*
    };
}

pub(crate) use try_from_cow_bytes_or_str;
