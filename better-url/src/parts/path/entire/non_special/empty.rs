//! [`NonSpecialEmptyPath`].

use crate::prelude::*;

/// An empty non-special path.
///
/// Pretends to but does not actually contain a [`Cow`].
#[derive(Debug, Clone, Default)]
pub struct NonSpecialEmptyPath<'a>(std::marker::PhantomData<Cow<'a, str>>);

impl<'a> NonSpecialEmptyPath<'a> {
    /// "Borrow" as a [`str`].
    pub fn as_str(&self) -> &str {
        ""
    }

    /// Turn into "the inner" [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        "".into()
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> NonSpecialEmptyPath<'static> {
        NonSpecialEmptyPath::default()
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> NonSpecialEmptyPath<'_> {
        NonSpecialEmptyPath::default()
    }
}

impl<'a> TryFrom<Cow<'a, str>> for NonSpecialEmptyPath<'a> {
    type Error = InvalidEmptyPath;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        match value.is_empty() {
            true  => Ok (Self::default()),
            false => Err(InvalidEmptyPath),
        }
    }
}
