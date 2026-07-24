//! [`SpecialQuery`].

use crate::prelude::*;

mod get;
mod set;

/// Either [`SpecialQuery`] or [`NonSpecialQuery`].
#[derive(Debug, Clone)]
pub enum Query<'a> {
    /** [`SpecialQuery`].    **/ Special   (SpecialQuery   <'a>),
    /** [`NonSpecialQuery`]. **/ NonSpecial(NonSpecialQuery<'a>),
}

impl<'a> Query<'a> {
    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::Special   (x) => x.as_str(),
            Self::NonSpecial(x) => x.as_str(),
        }
    }

    /// [`Self::len`] + 1 for the `?`.
    pub fn search_len(&self) -> usize {
        self.len() + 1
    }

    /// The [`QueryType`].
    pub fn r#type(&self) -> QueryType {
        match self {
            Self::Special   (_) => QueryType::Special   ,
            Self::NonSpecial(_) => QueryType::NonSpecial,
        }
    }



    /// Either [`Self::new_special`] or [`Self::new_non_special`].
    pub fn new<T: Into<SpecialQuery<'a>> + Into<NonSpecialQuery<'a>>>(value: T, r#type: QueryType) -> Self {
        match r#type {
            QueryType::Special    => Self::new_special    (value),
            QueryType::NonSpecial => Self::new_non_special(value),
        }
    }

    /** [`SpecialQuery::new`].    **/ pub fn new_special    <T: Into<SpecialQuery   <'a>>>(value: T) -> Self {SpecialQuery   ::new(value).into()}
    /** [`NonSpecialQuery::new`]. **/ pub fn new_non_special<T: Into<NonSpecialQuery<'a>>>(value: T) -> Self {NonSpecialQuery::new(value).into()}



    /// Either [`Self::new_special_unchecked`] or [`Self::new_non_special_unchecked`].
    /// # Safety
    /// Either [`Self::new_special_unchecked`] or [`Self::new_non_special_unchecked`].
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T, r#type: QueryType) -> Self {
        match r#type {
            QueryType::Special    => unsafe {Self::new_special_unchecked    (value)},
            QueryType::NonSpecial => unsafe {Self::new_non_special_unchecked(value)},
        }
    }

    /// [`SpecialQuery::new_unchecked`].
    /// # Safety
    /// [`SpecialQuery::new_unchecked`].
    pub unsafe fn new_special_unchecked    <T: Into<Cow<'a, str>>>(value: T) -> Self {unsafe {SpecialQuery   ::new_unchecked(value).into()}}

    /// [`NonSpecialQuery::new_unchecked`].
    /// # Safety
    /// [`NonSpecialQuery::new_unchecked`].
    pub unsafe fn new_non_special_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {unsafe {NonSpecialQuery::new_unchecked(value).into()}}




    /** If it's [`Self::Special`].    **/ pub fn is_special    (&self) -> bool {matches!(self, Self::Special   (_))}
    /** If it's [`Self::NonSpecial`]. **/ pub fn is_non_special(&self) -> bool {matches!(self, Self::NonSpecial(_))}




    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> Query<'_> {
        match self {
            Self::Special   (x) => x.borrowed().into(),
            Self::NonSpecial(x) => x.borrowed().into(),
        }
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> Query<'static> {
        match self {
            Self::Special   (x) => x.into_owned().into(),
            Self::NonSpecial(x) => x.into_owned().into(),
        }
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        match self {
            Self::Special   (x) => x.into_inner(),
            Self::NonSpecial(x) => x.into_inner(),
        }
    }
}



impl<'a> TryFrom<QueryLike<'a>> for Query<'a> {
    type Error = FragmentQuery<'a>;

    fn try_from(value: QueryLike<'a>) -> Result<Self, Self::Error> {
        match value {
            QueryLike::Query   (x) => Ok (x),
            QueryLike::Fragment(x) => Err(x),
        }
    }
}

impl<'a> From<QuerySegment<'a>> for Query<'a> {
    fn from(value: QuerySegment<'a>) -> Self {
        match value {
            QuerySegment::Special   (x) => x.into(),
            QuerySegment::NonSpecial(x) => x.into(),
        }
    }
}

impl<'a> TryFrom<QueryLikeSegment<'a>> for Query<'a> {
    type Error = FragmentQuerySegment<'a>;

    fn try_from(value: QueryLikeSegment<'a>) -> Result<Self, Self::Error> {
        match value {
            QueryLikeSegment::Query   (x) => Ok (x.into()),
            QueryLikeSegment::Fragment(x) => Err(x),
        }
    }
}

impl<'a> From<SpecialQuery   <'a>> for Query<'a> {fn from(value: SpecialQuery   <'a>) -> Self {Self::Special   (value)}}
impl<'a> From<NonSpecialQuery<'a>> for Query<'a> {fn from(value: NonSpecialQuery<'a>) -> Self {Self::NonSpecial(value)}}

impl<'a> From<SpecialQuerySegment   <'a>> for Query<'a> {fn from(value: SpecialQuerySegment   <'a>) -> Self {Self::Special   (value.into())}}
impl<'a> From<NonSpecialQuerySegment<'a>> for Query<'a> {fn from(value: NonSpecialQuerySegment<'a>) -> Self {Self::NonSpecial(value.into())}}
