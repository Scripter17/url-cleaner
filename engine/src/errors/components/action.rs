//! [`ActionError`].

use crate::prelude::*;

/// The enum of errors [`Action::apply`] can return.
#[derive(Debug, Error)]
pub enum ActionError {
    /** [`ExplicitError`].               **/ #[error(transparent)] ExplicitError      (#[from] ExplicitError            ),
    /** [`TryElseError`].                **/ #[error(transparent)] TryElseError       (#[from] Box<TryElseError<Self>>  ),
    /** [`FirstNotErrorErrors`].         **/ #[error(transparent)] FirstNotErrorErrors(#[from] FirstNotErrorErrors<Self>),

    /** [`url::ParseError`].             **/ #[error(transparent)] UrlParseError (#[from] url::ParseError),

    /** [`SetSchemeError`].              **/ #[error(transparent)] SetSchemeError  (#[from] SetSchemeError  ), 
    /** [`SetHostError`].                **/ #[error(transparent)] SetHostError    (#[from] SetHostError    ), 
    /** [`SetDomainError`].              **/ #[error(transparent)] SetDomainError  (#[from] SetDomainError  ), 
    /** [`SetPathError`].                **/ #[error(transparent)] SetPathError    (#[from] SetPathError    ), 
    /** [`SetQueryError`].               **/ #[error(transparent)] SetQueryError   (#[from] SetQueryError   ),
    /** [`SetFragmentError`].            **/ #[error(transparent)] SetFragmentError(#[from] SetFragmentError),

    /** [`ConditionError`].              **/ #[error(transparent)] ConditionError         (#[from] ConditionError         ),
    /** [`StringSourceError`].           **/ #[error(transparent)] StringSourceError      (#[from] StringSourceError      ),
    /** [`StringNotFound`].              **/ #[error(transparent)] StringNotFound         (#[from] StringNotFound         ),
    /** [`StringModificationError`].     **/ #[error(transparent)] StringModificationError(#[from] StringModificationError),
    /** [`StringMatcherError`].          **/ #[error(transparent)] StringMatcherError     (#[from] StringMatcherError     ),
    /** [`StringLocationError`].         **/ #[error(transparent)] StringLocationError    (#[from] StringLocationError    ),

    /** [`PartitioningSourceError`].     **/ #[error(transparent)] PartitioningSourceError(#[from] PartitioningSourceError),
    /** [`PartitioningNotFound`].        **/ #[error(transparent)] PartitioningNotFound   (#[from] PartitioningNotFound   ),
    /** [`ListSourceError`].             **/ #[error(transparent)] ListSourceError        (#[from] ListSourceError        ),
    /** [`ListNotFound`].                **/ #[error(transparent)] ListNotFound           (#[from] ListNotFound           ),
    /** [`SetSourceError`].              **/ #[error(transparent)] SetSourceError         (#[from] SetSourceError         ),
    /** [`SetNotFound`].                 **/ #[error(transparent)] SetNotFound            (#[from] SetNotFound            ),

    /** [`QueryParamNotFound`].          **/ #[error(transparent)] QueryParamNotFound     (#[from] QueryParamNotFound     ),

    /** [`ReadFromCacheError`].          **/ #[cfg(feature = "cache")] #[error(transparent)] ReadFromCacheError(#[from] ReadFromCacheError),
    /** [`WriteToCacheError`].           **/ #[cfg(feature = "cache")] #[error(transparent)] WriteToCacheError (#[from] WriteToCacheError ),

    /** [`FunctionNotFound`].            **/ #[error(transparent)] FunctionNotFound           (#[from] FunctionNotFound           ),
    /** [`NotInFunction`].               **/ #[error(transparent)] NotInFunction              (#[from] NotInFunction              ),
    /** [`FunctionArgFunctionNotFound`]. **/ #[error(transparent)] FunctionArgFunctionNotFound(#[from] FunctionArgFunctionNotFound),

    /** [`ExternError`]. **/ #[error(transparent)] Extern(#[from] ExternError),
}

impl From<TryElseError<Self>> for ActionError {fn from(value: TryElseError<Self>) -> Self {Box::new(value).into()}}
