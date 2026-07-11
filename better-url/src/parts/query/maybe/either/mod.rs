//! [`MaybeQuery`].

use crate::prelude::*;

mod get;
mod set;
mod filter;

/// Either [`MaybeSpecialQuery`] or [`MaybeNonSpecialQuery`].
#[derive(Debug, Clone)]
pub enum MaybeQuery<'a> {
    /// [`MaybeSpecialQuery`].
    Special(MaybeSpecialQuery<'a>),
    /// [`MaybeNonSpecialQuery`].
    NonSpecial(MaybeNonSpecialQuery<'a>)
}

impl<'a> MaybeQuery<'a> {
    /// Borrow as a [`str`].
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Special   (x) => x.as_str(),
            Self::NonSpecial(x) => x.as_str(),
        }
    }



    /// Make a new [`Self`].
    pub fn new<T: Into<MaybeSpecialQuery<'a>> + Into<MaybeNonSpecialQuery<'a>>>(value: T, special: bool) -> Self {
        match special {
            true  => Self::new_special    (value),
            false => Self::new_non_special(value),
        }
    }

    /// Make a new [`Self::Special`].
    pub fn new_special<T: Into<MaybeSpecialQuery<'a>>>(query: T) -> Self {
        query.into().into()
    }

    /// Make a new [`Self::NonSpecial`].
    pub fn new_non_special<T: Into<MaybeNonSpecialQuery<'a>>>(query: T) -> Self {
        query.into().into()
    }



    /// Make a new [`Self`] without doing any validity checks.
    /// # Safety
    /// `value` must be a valid [`Self`] literal.
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: Option<T>, special: bool) -> Self {
        unsafe {
            match special {
                true  => Self::new_special_unchecked    (value),
                false => Self::new_non_special_unchecked(value),
            }
        }
    }

    /// Make a new [`Self::Special`] without doing any validity checks.
    /// # Safety
    /// `value` must be a valid [`Self::Special`] literal.
    pub unsafe fn new_special_unchecked<T: Into<Cow<'a, str>>>(value: Option<T>) -> Self {
        unsafe {
            MaybeSpecialQuery::new_unchecked(value).into()
        }
    }

    /// Make a new [`Self::NonSpecial`] without doing any validity checks.
    /// # Safety
    /// `value` must be a valid [`Self::NonSpecial`] literal.
    pub unsafe fn new_non_special_unchecked<T: Into<Cow<'a, str>>>(value: Option<T>) -> Self {
        unsafe {
            MaybeNonSpecialQuery::new_unchecked(value).into()
        }
    }



    /// If it's [`Self::Special`].
    pub fn is_special(&self) -> bool {
        matches!(self, Self::Special(_))
    }

    /// If it's [`Self::NonSpecial`].
    pub fn is_non_special(&self) -> bool {
        matches!(self, Self::NonSpecial(_))
    }

    /// If it's [`Some`].
    pub fn is_some(&self) -> bool {
        match self {
            Self::Special   (x) => x.is_some(),
            Self::NonSpecial(x) => x.is_some(),
        }
    }

    /// If it's [`None`].
    pub fn is_none(&self) -> bool {
        match self {
            Self::Special   (x) => x.is_none(),
            Self::NonSpecial(x) => x.is_none(),
        }
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> MaybeQuery<'_> {
        match self {
            Self::Special   (x) => x.borrowed().into(),
            Self::NonSpecial(x) => x.borrowed().into(),
        }
    }

    /// Turn into an owned [`MaybeQuery`].
    pub fn into_owned(self) -> MaybeQuery<'static> {
        match self {
            Self::Special   (x) => x.into_owned().into(),
            Self::NonSpecial(x) => x.into_owned().into(),
        }
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Option<Cow<'a, str>> {
        match self {
            Self::Special   (x) => x.into_inner(),
            Self::NonSpecial(x) => x.into_inner(),
        }
    }
}



impl<'a> TryFrom<QueryLike<'a>> for MaybeQuery<'a> {
    type Error = FragmentQuery<'a>;

    fn try_from(value: QueryLike<'a>) -> Result<Self, Self::Error> {
        match value {
            QueryLike::Query   (x) => Ok (x.into()),
            QueryLike::Fragment(x) => Err(x)
        }
    }
}

impl<'a> From<Query<'a>> for MaybeQuery<'a> {
    fn from(value: Query<'a>) -> Self {
        match value {
            Query::Special   (x) => Self::Special   (x.into()),
            Query::NonSpecial(x) => Self::NonSpecial(x.into()),
        }
    }
}

impl<'a> TryFrom<QueryLikeSegment<'a>> for MaybeQuery<'a> {
    type Error = FragmentQuerySegment<'a>;

    fn try_from(value: QueryLikeSegment<'a>) -> Result<Self, Self::Error> {
        match value {
            QueryLikeSegment::Query   (x) => Ok (x.into()),
            QueryLikeSegment::Fragment(x) => Err(x),
        }
    }
}

impl<'a> From<QuerySegment<'a>> for MaybeQuery<'a> {
    fn from(value: QuerySegment<'a>) -> Self {
        match value {
            QuerySegment::Special   (x) => Self::Special   (x.into()),
            QuerySegment::NonSpecial(x) => Self::NonSpecial(x.into()),
        }
    }
}

impl<'a> From<MaybeSpecialQuery            <'a> > for MaybeQuery<'a> {fn from(value: MaybeSpecialQuery            <'a> ) -> Self {Self::Special   (value)       }}
impl<'a> From<MaybeNonSpecialQuery         <'a> > for MaybeQuery<'a> {fn from(value: MaybeNonSpecialQuery         <'a> ) -> Self {Self::NonSpecial(value)       }}

impl<'a> From<SpecialQuery                 <'a> > for MaybeQuery<'a> {fn from(value: SpecialQuery                 <'a> ) -> Self {Self::Special   (value.into())}}
impl<'a> From<NonSpecialQuery              <'a> > for MaybeQuery<'a> {fn from(value: NonSpecialQuery              <'a> ) -> Self {Self::NonSpecial(value.into())}}

impl<'a> From<SpecialQuerySegment          <'a> > for MaybeQuery<'a> {fn from(value: SpecialQuerySegment          <'a> ) -> Self {Self::Special   (value.into())}}
impl<'a> From<NonSpecialQuerySegment       <'a> > for MaybeQuery<'a> {fn from(value: NonSpecialQuerySegment       <'a> ) -> Self {Self::NonSpecial(value.into())}}

impl<'a> From<Option<SpecialQuery          <'a>>> for MaybeQuery<'a> {fn from(value: Option<SpecialQuery          <'a>>) -> Self {Self::Special   (value.into())}}
impl<'a> From<Option<NonSpecialQuery       <'a>>> for MaybeQuery<'a> {fn from(value: Option<NonSpecialQuery       <'a>>) -> Self {Self::NonSpecial(value.into())}}

impl<'a> From<Option<SpecialQuerySegment   <'a>>> for MaybeQuery<'a> {fn from(value: Option<SpecialQuerySegment   <'a>>) -> Self {Self::Special   (value.into())}}
impl<'a> From<Option<NonSpecialQuerySegment<'a>>> for MaybeQuery<'a> {fn from(value: Option<NonSpecialQuerySegment<'a>>) -> Self {Self::NonSpecial(value.into())}}
