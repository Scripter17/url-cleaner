//! [`SpecialQuery`].

use crate::prelude::*;

mod get;
mod set;

/// Either [`SpecialQuery`], [`NonSpecialQuery`], or [`FragmentQuery`].
#[derive(Debug, Clone)]
pub enum Query<'a> {
    /// [`SpecialQuery`].
    Special(SpecialQuery<'a>),
    /// [`NonSpecialQuery`].
    NonSpecial(NonSpecialQuery<'a>),
    /// [`FragmentQuery`].
    Fragment(FragmentQuery<'a>),
}

impl<'a> Query<'a> {
    /// [`Self::Special`].
    pub fn new_special<T: Into<SpecialQuery<'a>>>(query: T) -> Self {
        query.into().into()
    }

    /// [`Self::NonSpecial`].
    pub fn new_non_special<T: Into<NonSpecialQuery<'a>>>(query: T) -> Self {
        query.into().into()
    }

    /// [`Self::Fragment`].
    pub fn new_fragment<T: Into<FragmentQuery<'a>>>(fragment: T) -> Self {
        fragment.into().into()
    }



    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::Special   (x) => x.as_str(),
            Self::NonSpecial(x) => x.as_str(),
            Self::Fragment  (x) => x.as_str(),
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

    /// If it's [`Self::Fragment`].
    pub fn is_fragment(&self) -> bool {
        matches!(self, Self::Fragment(_))
    }



    /// The [`SpecialQuery`].
    pub fn special(self) -> Option<SpecialQuery<'a>> {
        match self {
            Self::Special   (x) => Some(x),
            Self::NonSpecial(_) => None,
            Self::Fragment  (_) => None,
        }
    }

    /// The [`NonSpecialQuery`].
    pub fn non_special(self) -> Option<NonSpecialQuery<'a>> {
        match self {
            Self::Special   (_) => None,
            Self::NonSpecial(x) => Some(x),
            Self::Fragment  (_) => None,
        }
    }

    /// The [`FragmentQuery`].
    pub fn fragment(self) -> Option<FragmentQuery<'a>> {
        match self {
            Self::Special   (_) => None,
            Self::NonSpecial(_) => None,
            Self::Fragment  (x) => Some(x),
        }
    }



    /// Turn into a [`SpecialQuery`].
    pub fn into_special(self) -> SpecialQuery<'a> {
        self.into()
    }

    /// Turn into a [`NonSpecialQuery`].
    pub fn into_non_special(self) -> NonSpecialQuery<'a> {
        self.into()
    }

    /// Turn into a [`FragmentQuery`].
    pub fn into_fragment(self) -> FragmentQuery<'a> {
        self.into()
    }



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        match self {
            Self::Special   (x) => x.into_inner(),
            Self::NonSpecial(x) => x.into_inner(),
            Self::Fragment  (x) => x.into_inner(),
        }
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> Query<'static> {
        match self {
            Self::Special   (x) => x.into_owned().into(),
            Self::NonSpecial(x) => x.into_owned().into(),
            Self::Fragment  (x) => x.into_owned().into(),
        }
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> Query<'_> {
        match self {
            Self::Special   (x) => x.borrowed().into(),
            Self::NonSpecial(x) => x.borrowed().into(),
            Self::Fragment  (x) => x.borrowed().into(),
        }
    }
}



impl<'a> From<QuerySegment<'a>> for Query<'a> {
    fn from(value: QuerySegment<'a>) -> Self {
        match value {
            QuerySegment::Special   (x) => x.into(),
            QuerySegment::NonSpecial(x) => x.into(),
            QuerySegment::Fragment  (x) => x.into(),
        }
    }
}

impl<'a> From<SpecialQuerySegment   <'a>> for Query<'a> {fn from(value: SpecialQuerySegment   <'a>) -> Self {Self::Special   (value.into())}}
impl<'a> From<NonSpecialQuerySegment<'a>> for Query<'a> {fn from(value: NonSpecialQuerySegment<'a>) -> Self {Self::NonSpecial(value.into())}}
impl<'a> From<FragmentQuerySegment  <'a>> for Query<'a> {fn from(value: FragmentQuerySegment  <'a>) -> Self {Self::Fragment  (value.into())}}



impl<'a> From<SpecialQuery   <'a>> for Query<'a> {fn from(value: SpecialQuery   <'a>) -> Self {Self::Special   (value)}}
impl<'a> From<NonSpecialQuery<'a>> for Query<'a> {fn from(value: NonSpecialQuery<'a>) -> Self {Self::NonSpecial(value)}}
impl<'a> From<FragmentQuery  <'a>> for Query<'a> {fn from(value: FragmentQuery  <'a>) -> Self {Self::Fragment  (value)}}

impl<'a> From<Fragment       <'a>> for Query<'a> {fn from(value: Fragment       <'a>) -> Self {Self::Fragment  (value.into())}}
