//! [`PartitioningSourceError`].

use crate::prelude::*;

/// [`PartitioningSource::get`].
#[derive(Debug, Error)]
pub enum PartitioningSourceError {
    /** [`StringSourceError`]. **/ #[error(transparent)] StringSourceError(#[from] StringSourceError),
    /** [`StringNotFound`].    **/ #[error(transparent)] StringNotFound   (#[from] StringNotFound   ),
}
