//! [`FlagSourceError`].

use crate::prelude::*;

impl From<StringSourceError> for FlagSourceError {fn from(value: StringSourceError) -> Self {Box::new(value).into()}}

/// [`FlagSource::get`].
#[derive(Debug, Error)]
pub enum FlagSourceError {
    /** [`StringSourceError`]. **/ #[error(transparent)] StringSourceError(#[from] Box<StringSourceError>),
    /** [`StringNotFound`].    **/ #[error(transparent)] StringNotFound   (#[from] StringNotFound        ),
    /** [`NotInFunction`].     **/ #[error(transparent)] NotInFunction    (#[from] NotInFunction         ),
}
