//! Macros.

mod as_str               ; pub(crate) use as_str               ::*;
mod from_cow_bytes       ; pub(crate) use from_cow_bytes       ::*;
mod from_option_cow_bytes; pub(crate) use from_option_cow_bytes::*;
mod try_from_cow_bytes   ; pub(crate) use try_from_cow_bytes   ::*;

/// Impls for `T::borrowed`.
macro_rules! borrowed {
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

/// [`From`] for references of similar types.
macro_rules! from_borrowed {
    ($t:ident$(, $x:ident)*$(,)?) => {
        $(
            impl<'a> From<&'a $x<'_>> for $t<'a> {fn from(value: &'a $x<'_>) -> Self {value.borrowed().into()}}
        )*
    }
}

/// [`From`] for [`Option`]s and borrows of [`Option`]s
macro_rules! from_options {
    ($t:ident$(, $x:ident)*$(,)?) => {
        $(
            impl<'a> From<               $x<'a> > for $t<'a> {fn from(value:                $x<'a> ) -> Self {Some(value).into()}}
            impl<'a> From<&'a Option<    $x<'_>>> for $t<'a> {fn from(value: &'a Option<    $x<'_>>) -> Self {value.as_ref().into()}}
            impl<'a> From<    Option<&'a $x<'_>>> for $t<'a> {fn from(value:     Option<&'a $x<'_>>) -> Self {value.map($x::borrowed).into()}}
        )*
    }
}

pub(crate) use borrowed;
pub(crate) use from_borrowed;
pub(crate) use from_options;
