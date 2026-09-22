//! [`as_str`].

/// Impls for str like types.
macro_rules! as_str {
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

        as_str!($t);
        as_str!($u$($rest)*);
    };

    ($t:ident, ?$u:ident$($rest:tt)*) => {
        impl PartialEq <$u<'_>> for $t<'_> {fn eq(&self, other: &$u<'_>) -> bool {Some(self.as_str()) == other.as_str()}}
        impl PartialEq <$t<'_>> for $u<'_> {fn eq(&self, other: &$t<'_>) -> bool {other == self}}

        impl PartialOrd<$u<'_>> for $t<'_> {fn partial_cmp(&self, other: &$u<'_>) -> Option<Ordering> {Some(self.as_str()).partial_cmp(&other.as_str())}}
        impl PartialOrd<$t<'_>> for $u<'_> {fn partial_cmp(&self, other: &$t<'_>) -> Option<Ordering> {other.partial_cmp(self)}}

        as_str!($t);
        as_str!(?$u$($rest)*);
    };

    (?$t:ident, $u:ident$($rest:tt)*) => {
        impl PartialEq <$u<'_>> for $t<'_> {fn eq(&self, other: &$u<'_>) -> bool {self.as_str() == Some(other.as_str())}}
        impl PartialEq <$t<'_>> for $u<'_> {fn eq(&self, other: &$t<'_>) -> bool {other == self}}

        impl PartialOrd<$u<'_>> for $t<'_> {fn partial_cmp(&self, other: &$u<'_>) -> Option<Ordering> {self.as_str().partial_cmp(&Some(other.as_str()))}}
        impl PartialOrd<$t<'_>> for $u<'_> {fn partial_cmp(&self, other: &$t<'_>) -> Option<Ordering> {other.partial_cmp(self)}}

        as_str!(?$t);
        as_str!($u$($rest)*);
    };

    (?$t:ident, ?$u:ident$($rest:tt)*) => {
        impl PartialEq <$u<'_>> for $t<'_> {fn eq(&self, other: &$u<'_>) -> bool {self.as_str() == other.as_str()}}
        impl PartialEq <$t<'_>> for $u<'_> {fn eq(&self, other: &$t<'_>) -> bool {other == self}}

        impl PartialOrd<$u<'_>> for $t<'_> {fn partial_cmp(&self, other: &$u<'_>) -> Option<Ordering> {self.as_str().partial_cmp(&other.as_str())}}
        impl PartialOrd<$t<'_>> for $u<'_> {fn partial_cmp(&self, other: &$t<'_>) -> Option<Ordering> {other.partial_cmp(self)}}

        as_str!(?$t);
        as_str!(?$u$($rest)*);
    };
}

pub(crate) use as_str;
