//! [`CharMatcherError`].

use crate::prelude::*;

/// [`CharMatcher::check`].
#[derive(Debug, Error)]
pub enum CharMatcherError {
    /** [`ExplicitError`].       **/ #[error(transparent)] ExplicitError       (#[from] ExplicitError            ),
    /** [`TryElseError`].        **/ #[error(transparent)] TryElseError        (#[from] Box<TryElseError<Self>>  ),
    /** [`FirstNotErrorErrors`]. **/ #[error(transparent)] FirstNotErrorErrors (#[from] FirstNotErrorErrors<Self>),
}

impl From<TryElseError<Self>> for CharMatcherError {fn from(value: TryElseError<Self>) -> Self {Box::new(value).into()}}
