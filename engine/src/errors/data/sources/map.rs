//! [`MapSourceError`].

use crate::prelude::*;

impl From<ConditionError> for MapSourceError  {fn from(value: ConditionError) -> Self {Box::new(value).into()}}

/// [`ListSource::get`].
#[derive(Debug, Error)]
pub enum MapSourceError {
    /** [`StringNotFound`].    **/ #[error(transparent)] StringNotFound   (#[from] StringNotFound     ),
    /** [`StringSourceError`]. **/ #[error(transparent)] StringSourceError(#[from] StringSourceError  ),
    /** [`ConditionError`].    **/ #[error(transparent)] ConditionError   (#[from] Box<ConditionError>),
    /** [`NotInFunction`].     **/ #[error(transparent)] NotInFunction    (#[from] NotInFunction      ),
}
