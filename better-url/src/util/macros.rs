//! Macros.

/// Impls for str like types.
macro_rules! as_str_impls {
    ($t:ident) => {
        impl<'a> $t<'a> {
            /// The length.
            pub fn len(&self) -> usize {
                self.as_str().len()
            }

            /// If it's empty.
            pub fn is_empty(&self) -> bool {
                self.as_str().is_empty()
            }
        }



        impl PartialEq<str>          for $t<'_> {fn eq(&self, other: &str         ) -> bool {self.as_str() ==  other}}
        impl PartialEq<&str>         for $t<'_> {fn eq(&self, other: &&str        ) -> bool {self.as_str() == *other}}
        impl PartialEq<String>       for $t<'_> {fn eq(&self, other: &String      ) -> bool {self.as_str() ==  other}}
        impl PartialEq<Cow<'_, str>> for $t<'_> {fn eq(&self, other: &Cow<'_, str>) -> bool {self.as_str() ==  other}}

        impl PartialEq<$t<'_>> for str          {fn eq(&self, other: &$t<'_>) -> bool {other == self}}
        impl PartialEq<$t<'_>> for &str         {fn eq(&self, other: &$t<'_>) -> bool {other == self}}
        impl PartialEq<$t<'_>> for String       {fn eq(&self, other: &$t<'_>) -> bool {other == self}}
        impl PartialEq<$t<'_>> for Cow<'_, str> {fn eq(&self, other: &$t<'_>) -> bool {other == self}}

        impl PartialEq <$t<'_>> for $t<'_> {fn eq(&self, other: &$t<'_>) -> bool {self.as_str() == other.as_str()}}
        impl Eq for $t<'_> {}



        impl PartialOrd<str         > for $t<'_> {fn partial_cmp(&self, other: &str         ) -> Option<Ordering> {self.as_str().partial_cmp(   other)}}
        impl PartialOrd<&str        > for $t<'_> {fn partial_cmp(&self, other: &&str        ) -> Option<Ordering> {self.as_str().partial_cmp(  *other)}}
        impl PartialOrd<String      > for $t<'_> {fn partial_cmp(&self, other: &String      ) -> Option<Ordering> {self.as_str().partial_cmp(&**other)}}
        impl PartialOrd<Cow<'_, str>> for $t<'_> {fn partial_cmp(&self, other: &Cow<'_, str>) -> Option<Ordering> {self.as_str().partial_cmp(&**other)}}

        impl PartialOrd<$t<'_>> for str          {fn partial_cmp(&self, other: &$t<'_>) -> Option<Ordering> {   self .partial_cmp(other.as_str())}}
        impl PartialOrd<$t<'_>> for &str         {fn partial_cmp(&self, other: &$t<'_>) -> Option<Ordering> {( *self).partial_cmp(other.as_str())}}
        impl PartialOrd<$t<'_>> for String       {fn partial_cmp(&self, other: &$t<'_>) -> Option<Ordering> {(**self).partial_cmp(other.as_str())}}
        impl PartialOrd<$t<'_>> for Cow<'_, str> {fn partial_cmp(&self, other: &$t<'_>) -> Option<Ordering> {(**self).partial_cmp(other.as_str())}}

        impl PartialOrd<$t<'_>> for $t<'_> {fn partial_cmp(&self, other: &$t<'_>) -> Option<Ordering> {self.as_str().partial_cmp(other.as_str())}}
        impl Ord                for $t<'_> {fn cmp        (&self, other: &Self  ) -> Ordering         {self.as_str().cmp        (other.as_str())}}



        impl AsRef <str> for $t<'_> {fn as_ref(&self) -> &str {self.as_str()}}
        impl Borrow<str> for $t<'_> {fn borrow(&self) -> &str {self.as_str()}}



        impl Hash for $t<'_> {
            fn hash<H: Hasher>(&self, hasher: &mut H) {
                self.as_str().hash(hasher)
            }
        }

        impl Display for $t<'_> {
            fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
                write!(formatter, "{}", self.as_str())
            }
        }

        #[cfg(feature = "serde")]
        impl Serialize for $t<'_> {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                self.as_str().serialize(serializer)
            }
        }
    };

    (?$t:ident) => {
        impl<'a> $t<'a> {
            /// The length.
            pub fn len(&self) -> Option<usize> {
                self.as_str().map(str::len)
            }

            /// If it's empty.
            pub fn is_empty(&self) -> Option<bool> {
                self.as_str().map(str::is_empty)
            }
        }



        impl PartialEq<str>          for $t<'_> {fn eq(&self, other: &str         ) -> bool {self.as_str() == Some( other)}}
        impl PartialEq<&str>         for $t<'_> {fn eq(&self, other: &&str        ) -> bool {self.as_str() == Some(*other)}}
        impl PartialEq<String>       for $t<'_> {fn eq(&self, other: &String      ) -> bool {self.as_str() == Some( other)}}
        impl PartialEq<Cow<'_, str>> for $t<'_> {fn eq(&self, other: &Cow<'_, str>) -> bool {self.as_str() == Some( other)}}

        impl PartialEq<$t<'_>> for str          {fn eq(&self, other: &$t<'_>) -> bool {other == self}}
        impl PartialEq<$t<'_>> for &str         {fn eq(&self, other: &$t<'_>) -> bool {other == self}}
        impl PartialEq<$t<'_>> for String       {fn eq(&self, other: &$t<'_>) -> bool {other == self}}
        impl PartialEq<$t<'_>> for Cow<'_, str> {fn eq(&self, other: &$t<'_>) -> bool {other == self}}



        impl PartialEq <$t<'_>> for $t<'_> {fn eq(&self, other: &$t<'_>) -> bool {self.as_str() == other.as_str()}}
        impl Eq for $t<'_> {}

        impl PartialOrd<str         > for $t<'_> {fn partial_cmp(&self, other: &str         ) -> Option<Ordering> {self.as_str().partial_cmp(&Some(   other))}}
        impl PartialOrd<&str        > for $t<'_> {fn partial_cmp(&self, other: &&str        ) -> Option<Ordering> {self.as_str().partial_cmp(&Some(  *other))}}
        impl PartialOrd<String      > for $t<'_> {fn partial_cmp(&self, other: &String      ) -> Option<Ordering> {self.as_str().partial_cmp(&Some(&**other))}}
        impl PartialOrd<Cow<'_, str>> for $t<'_> {fn partial_cmp(&self, other: &Cow<'_, str>) -> Option<Ordering> {self.as_str().partial_cmp(&Some(&**other))}}

        impl PartialOrd<$t<'_>> for str          {fn partial_cmp(&self, other: &$t<'_>) -> Option<Ordering> {other.partial_cmp(self)}}
        impl PartialOrd<$t<'_>> for &str         {fn partial_cmp(&self, other: &$t<'_>) -> Option<Ordering> {other.partial_cmp(self)}}
        impl PartialOrd<$t<'_>> for String       {fn partial_cmp(&self, other: &$t<'_>) -> Option<Ordering> {other.partial_cmp(self)}}
        impl PartialOrd<$t<'_>> for Cow<'_, str> {fn partial_cmp(&self, other: &$t<'_>) -> Option<Ordering> {other.partial_cmp(self)}}

        impl PartialOrd<$t<'_>> for $t<'_> {fn partial_cmp(&self, other: &$t<'_>) -> Option<Ordering> {self.as_str().partial_cmp(&other.as_str())}}
        impl Ord                for $t<'_> {fn cmp        (&self, other: &Self  ) -> Ordering         {self.as_str().cmp        (&other.as_str())}}



        impl Hash for $t<'_> {
            fn hash<H: Hasher>(&self, hasher: &mut H) {
                self.as_str().hash(hasher)
            }
        }

        #[cfg(feature = "serde")]
        impl Serialize for $t<'_> {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                self.as_str().serialize(serializer)
            }
        }
    };

    ($t:ident, $u:ident$($rest:tt)*) => {
        impl PartialEq <$u<'_>> for $t<'_> {fn eq(&self, other: &$u<'_>) -> bool {self.as_str() == other.as_str()}}
        impl PartialEq <$t<'_>> for $u<'_> {fn eq(&self, other: &$t<'_>) -> bool {other == self}}

        impl PartialOrd<$u<'_>> for $t<'_> {fn partial_cmp(&self, other: &$u<'_>) -> Option<Ordering> {self.as_str().partial_cmp(other.as_str())}}
        impl PartialOrd<$t<'_>> for $u<'_> {fn partial_cmp(&self, other: &$t<'_>) -> Option<Ordering> {other.partial_cmp(self)}}

        as_str_impls!($t);
        as_str_impls!($u$($rest)*);
    };

    ($t:ident, ?$u:ident$($rest:tt)*) => {
        impl PartialEq <$u<'_>> for $t<'_> {fn eq(&self, other: &$u<'_>) -> bool {Some(self.as_str()) == other.as_str()}}
        impl PartialEq <$t<'_>> for $u<'_> {fn eq(&self, other: &$t<'_>) -> bool {other == self}}

        impl PartialOrd<$u<'_>> for $t<'_> {fn partial_cmp(&self, other: &$u<'_>) -> Option<Ordering> {Some(self.as_str()).partial_cmp(&other.as_str())}}
        impl PartialOrd<$t<'_>> for $u<'_> {fn partial_cmp(&self, other: &$t<'_>) -> Option<Ordering> {other.partial_cmp(self)}}

        as_str_impls!($t);
        as_str_impls!(?$u$($rest)*);
    };

    (?$t:ident, $u:ident$($rest:tt)*) => {
        impl PartialEq <$u<'_>> for $t<'_> {fn eq(&self, other: &$u<'_>) -> bool {self.as_str() == Some(other.as_str())}}
        impl PartialEq <$t<'_>> for $u<'_> {fn eq(&self, other: &$t<'_>) -> bool {other == self}}

        impl PartialOrd<$u<'_>> for $t<'_> {fn partial_cmp(&self, other: &$u<'_>) -> Option<Ordering> {self.as_str().partial_cmp(&Some(other.as_str()))}}
        impl PartialOrd<$t<'_>> for $u<'_> {fn partial_cmp(&self, other: &$t<'_>) -> Option<Ordering> {other.partial_cmp(self)}}

        as_str_impls!(?$t);
        as_str_impls!($u$($rest)*);
    };

    (?$t:ident, ?$u:ident$($rest:tt)*) => {
        impl PartialEq <$u<'_>> for $t<'_> {fn eq(&self, other: &$u<'_>) -> bool {self.as_str() == other.as_str()}}
        impl PartialEq <$t<'_>> for $u<'_> {fn eq(&self, other: &$t<'_>) -> bool {other == self}}

        impl PartialOrd<$u<'_>> for $t<'_> {fn partial_cmp(&self, other: &$u<'_>) -> Option<Ordering> {self.as_str().partial_cmp(&other.as_str())}}
        impl PartialOrd<$t<'_>> for $u<'_> {fn partial_cmp(&self, other: &$t<'_>) -> Option<Ordering> {other.partial_cmp(self)}}

        as_str_impls!(?$t);
        as_str_impls!(?$u$($rest)*);
    };
}

/// Impls for types that impl [`From`] of [`Cow`] of [`str`].
macro_rules! from_cow_impls {
    ($($t:ident),*) => {
        $(
            impl<'a> $t<'a> {
                /// Make a new [`Self`].
                pub fn new<T: Into<Self>>(value: T) -> Self {
                    value.into()
                }
            }

            impl FromStr for $t<'static> {
                type Err = std::convert::Infallible;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Ok(s.to_string().into())
                }
            }

            impl<'a> From<&'a str> for $t<'a     > {fn from(value: &'a str) -> Self {Cow::from(value).into()}}
            impl     From<String > for $t<'static> {fn from(value: String ) -> Self {Cow::from(value).into()}}

            #[cfg(feature = "serde")]
            impl<'de> Deserialize<'de> for $t<'de> {
                fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    <Cow<'de, str>>::deserialize(deserializer).map(Into::into)
                }
            }
        )*
    };
}

/// Impls for types that impl [`From`] of [`Option`] of [`Cow`] of [`str`].
macro_rules! from_option_cow_impls {
    ($($t:ident),*) => {
        $(
            impl<'a> $t<'a> {
                /// Make a new [`Self`].
                pub fn new<T: Into<Self>>(value: T) -> Self {
                    value.into()
                }
            }

            impl FromStr for $t<'static> {
                type Err = std::convert::Infallible;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Ok(s.to_string().into())
                }
            }

            impl<'a> From<&'a str     > for $t<'a     > {fn from(value: &'a str     ) -> Self {Cow::from(value).into()}}
            impl     From<String      > for $t<'static> {fn from(value: String      ) -> Self {Cow::from(value).into()}}
            impl<'a> From<Cow<'a, str>> for $t<'a     > {fn from(value: Cow<'a, str>) -> Self {Some(value).into()}}

            impl<'a> From<Option<&'a str>> for $t<'a     > {fn from(value: Option<&'a str>) -> Self {value.map(Cow::from).into()}}
            impl     From<Option<String >> for $t<'static> {fn from(value: Option<String >) -> Self {value.map(Cow::from).into()}}

            #[cfg(feature = "serde")]
            impl<'de> Deserialize<'de> for $t<'de> {
                fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    <Option<Cow<'de, str>>>::deserialize(deserializer).map(Into::into)
                }
            }
        )*
    };
}

/// Impls for types that impl [`TryFrom`] of [`Cow`] of [`str`].
macro_rules! try_from_cow_impls {
    ($($t:ident),*) => {
        $(
            impl<'a> $t<'a> {
                /// Make a new [`Self`].
                /// # Errors
                /// If the call to [`TryInto::try_into`] returns an error, that error is returned.
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

            impl<'a> TryFrom<&'a str> for $t<'a> {
                type Error = <Self as TryFrom<Cow<'a, str>>>::Error;

                fn try_from(value: &'a str) -> Result<Self, Self::Error> {
                    Cow::from(value).try_into()
                }
            }

            impl TryFrom<String> for $t<'static> {
                type Error = <Self as TryFrom<Cow<'static, str>>>::Error;

                fn try_from(value: String) -> Result<Self, Self::Error> {
                    Cow::from(value).try_into()
                }
            }

            #[cfg(feature = "serde")]
            impl<'de> Deserialize<'de> for $t<'de> {
                fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    <Cow<'de, str>>::deserialize(deserializer)?.try_into().map_err(D::Error::custom)
                }
            }
        )*
    };
}

/// Impls for types that impl [`TryFrom`] of [`Option`] of [`Cow`] of [`str`].
macro_rules! try_from_option_cow_impls {
    ($($t:ident),*) => {
        $(
            impl<'a> $t<'a> {
                /// Make a new [`Self`].
                /// # Errors
                /// If the call to [`TryInto::try_into`] returns an error, that error is returned.
                pub fn new<T: TryInto<Self>>(value: T) -> Result<Self, T::Error> {
                    value.try_into()
                }
            }

            impl FromStr for $t<'static> {
                type Err = <Self as TryFrom<String>>::Error;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    s.to_string().try_into()
                }
            }

            impl<'a> TryFrom<&'a str     > for $t<'a     > {
                type Error = <Self as TryFrom<Option<Cow<'a, str>>>>::Error;

                fn try_from(value: &'a str     ) -> Result<Self, Self::Error> {
                    Cow::from(value).try_into()
                }
            }
            impl     TryFrom<String      > for $t<'static> {
                type Error = <Self as TryFrom<Option<Cow<'static, str>>>>::Error;

                fn try_from(value: String      ) -> Result<Self, Self::Error> {
                    Cow::from(value).try_into()
                }
            }
            impl<'a> TryFrom<Cow<'a, str>> for $t<'a     > {
                type Error = <Self as TryFrom<Option<Cow<'a, str>>>>::Error;

                fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
                    Some(value).try_into()
                }
            }

            impl<'a> TryFrom<Option<&'a str>> for $t<'a     > {
                type Error = <Self as TryFrom<Option<Cow<'a, str>>>>::Error;

                fn try_from(value: Option<&'a str>) -> Result<Self, Self::Error> {
                    value.map(Cow::from).try_into()
                }
            }

            impl     TryFrom<Option<String >> for $t<'static> {
                type Error = <Self as TryFrom<Option<Cow<'static, str>>>>::Error;

                fn try_from(value: Option<String >) -> Result<Self, Self::Error> {
                    value.map(Cow::from).try_into()
                }
            }

            #[cfg(feature = "serde")]
            impl<'de> Deserialize<'de> for $t<'de> {
                fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    <Option<Cow<'de, str>>>::deserialize(deserializer)?.try_into().map_err(D::Error::custom)
                }
            }
        )*
    };
}

/// Impls for `T::borrowed`.
macro_rules! borrowed_impls {
    ($($t:ident),*) => {
        $(
            impl<'a> From<&'a $t<'_>> for $t<'a> {
                fn from(value: &'a $t<'_>) -> Self {
                    value.borrowed()
                }
            }
        )*
    }
}

pub(crate) use as_str_impls;
pub(crate) use from_cow_impls;
pub(crate) use from_option_cow_impls;
pub(crate) use try_from_cow_impls;
pub(crate) use try_from_option_cow_impls;
pub(crate) use borrowed_impls;
