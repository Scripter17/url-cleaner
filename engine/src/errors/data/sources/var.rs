//! [`VarSourceError`].

use crate::prelude::*;

impl From<StringSourceError> for VarSourceError  {fn from(value: StringSourceError) -> Self {Box::new(value).into()}}

/// [`VarSource::get`].
#[derive(Debug, Error)]
pub enum VarSourceError {
    /** [`StringSourceError`]. **/ #[error(transparent)] StringSourceError(#[from] Box<StringSourceError>),
    /** [`StringNotFound`].    **/ #[error(transparent)] StringNotFound   (#[from] StringNotFound        ),
    /** [`NotInFunction`].     **/ #[error(transparent)] NotInFunction    (#[from] NotInFunction         ),
}
