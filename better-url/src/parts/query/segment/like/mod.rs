//! [`QueryLikeSegment`].

use crate::prelude::*;

mod name;
mod value;

/// Either [`QuerySegment`] or [`FragmentQuerySegment`].
#[derive(Debug, Clone)]
pub enum QueryLikeSegment<'a> {
    /** [`QuerySegment`].         **/ Query   (QuerySegment        <'a>),
    /** [`FragmentQuerySegment`]. **/ Fragment(FragmentQuerySegment<'a>),
}

impl<'a> QueryLikeSegment<'a> {
    /// The [`QueryLikeType`].
    pub fn r#type(&self) -> QueryLikeType {
        match self {
            Self::Query   (x) => x.r#type().into(),
            Self::Fragment(_) => QueryLikeType::Fragment,
        }
    }

    /// Either [`Self::new_query`] or [`Self::new_fragment`].
    pub fn new<T: Into<SpecialQuerySegment<'a>> + Into<NonSpecialQuerySegment<'a>> + Into<FragmentQuerySegment<'a>>>(value: T, r#type: QueryLikeType) -> Self {
        match r#type {
            QueryLikeType::Query   (r#type) => Self::new_query   (value, r#type),
            QueryLikeType::Fragment         => Self::new_fragment(value        ),
        }
    }

    /** [`QuerySegment::new`].           **/ pub fn new_query            <T: Into<SpecialQuerySegment   <'a>> + Into<NonSpecialQuerySegment<'a>>>(value: T, r#type: QueryType) -> Self {QuerySegment          ::new(value, r#type).into()}
    /** [`SpecialQuerySegment::new`].    **/ pub fn new_special_query    <T: Into<SpecialQuerySegment   <'a>>                                   >(value: T                   ) -> Self {SpecialQuerySegment   ::new(value        ).into()}
    /** [`NonSpecialQuerySegment::new`]. **/ pub fn new_non_special_query<T: Into<NonSpecialQuerySegment<'a>>                                   >(value: T                   ) -> Self {NonSpecialQuerySegment::new(value        ).into()}
    /** [`FragmentQuerySegment::new`].   **/ pub fn new_fragment         <T: Into<FragmentQuerySegment  <'a>>                                   >(value: T                   ) -> Self {FragmentQuerySegment  ::new(value        ).into()}



    /// Either [`Self::new_query_unchecked`] or [`Self::new_fragment_unchecked`].
    /// # Safety
    /// Either [`Self::new_query_unchecked`] or [`Self::new_fragment_unchecked`].
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T, r#type: QueryLikeType) -> Self {
        unsafe {
            match r#type {
                QueryLikeType::Query   (r#type) => Self::new_query_unchecked   (value, r#type),
                QueryLikeType::Fragment         => Self::new_fragment_unchecked(value        ),
            }
        }

    }

    /// [`QuerySegment::new_unchecked`].
    /// # Safety
    /// [`QuerySegment::new_unchecked`].
    pub unsafe fn new_query_unchecked            <T: Into<Cow<'a, str>>>(value: T, r#type: QueryType) -> Self {unsafe {QuerySegment          ::new_unchecked(value, r#type).into()}}

    /// [`SpecialQuerySegment::new_unchecked`].
    /// # Safety
    /// [`SpecialQuerySegment::new_unchecked`].
    pub unsafe fn new_special_query_unchecked    <T: Into<Cow<'a, str>>>(value: T                   ) -> Self {unsafe {SpecialQuerySegment   ::new_unchecked(value        ).into()}}

    /// [`NonSpecialQuerySegment::new_unchecked`].
    /// # Safety
    /// [`NonSpecialQuerySegment::new_unchecked`].
    pub unsafe fn new_non_special_query_unchecked<T: Into<Cow<'a, str>>>(value: T                   ) -> Self {unsafe {NonSpecialQuerySegment::new_unchecked(value        ).into()}}

    /// [`FragmentQuerySegment::new_unchecked`].
    /// # Safety
    /// [`FragmentQuerySegment::new_unchecked`].
    pub unsafe fn new_fragment_unchecked         <T: Into<Cow<'a, str>>>(value: T                   ) -> Self {unsafe {FragmentQuerySegment  ::new_unchecked(value        ).into()}}



    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::Query   (x) => x.as_str(),
            Self::Fragment(x) => x.as_str(),
        }
    }



    /** If it's [`Self::Query`].       **/ pub fn is_query            (&self) -> bool {matches!(self, Self::Query   (_                          ))}
    /** If it's [`Query::Special`].    **/ pub fn is_special_query    (&self) -> bool {matches!(self, Self::Query   (QuerySegment::Special   (_)))}
    /** If it's [`Query::NonSpecial`]. **/ pub fn is_non_special_query(&self) -> bool {matches!(self, Self::Query   (QuerySegment::NonSpecial(_)))}
    /** If it's [`Self::Fragment`].    **/ pub fn is_fragment         (&self) -> bool {matches!(self, Self::Fragment(_                          ))}




    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> QueryLikeSegment<'_> {
        match self {
            Self::Query   (x) => x.borrowed().into(),
            Self::Fragment(x) => x.borrowed().into(),
        }
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> QueryLikeSegment<'static> {
        match self {
            Self::Query   (x) => x.into_owned().into(),
            Self::Fragment(x) => x.into_owned().into(),
        }
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        match self {
            Self::Query   (x) => x.into_inner(),
            Self::Fragment(x) => x.into_inner(),
        }
    }
}

impl<'a> From<QuerySegment          <'a>> for QueryLikeSegment<'a> {fn from(value: QuerySegment          <'a>) -> Self {Self::Query   (value       )}}
impl<'a> From<SpecialQuerySegment   <'a>> for QueryLikeSegment<'a> {fn from(value: SpecialQuerySegment   <'a>) -> Self {Self::Query   (value.into())}}
impl<'a> From<NonSpecialQuerySegment<'a>> for QueryLikeSegment<'a> {fn from(value: NonSpecialQuerySegment<'a>) -> Self {Self::Query   (value.into())}}
impl<'a> From<FragmentQuerySegment  <'a>> for QueryLikeSegment<'a> {fn from(value: FragmentQuerySegment  <'a>) -> Self {Self::Fragment(value       )}}
