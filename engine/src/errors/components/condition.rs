//! [`ConditionError`].

use crate::prelude::*;

/// The enum of errors [`Condition::check`] can return.
#[derive(Debug, Error)]
pub enum ConditionError {
    /** [`ExplicitError`].               **/ #[error(transparent)] ExplicitError              (#[from] ExplicitError              ),
    /** [`TryElseError`].                **/ #[error(transparent)] TryElseError               (#[from] Box<TryElseError<Self>>    ),
    /** [`FirstNotErrorErrors`].         **/ #[error(transparent)] FirstNotErrorErrors        (#[from] FirstNotErrorErrors<Self>  ),

    /** [`StringNotFound`].              **/ #[error(transparent)] StringNotFound             (#[from] StringNotFound             ),
    /** [`StringSourceError`].           **/ #[error(transparent)] StringSourceError          (#[from] StringSourceError          ),
    /** [`StringMatcherError`].          **/ #[error(transparent)] StringMatcherError         (#[from] StringMatcherError         ),
    /** [`StringLocationError`].         **/ #[error(transparent)] StringLocationError        (#[from] StringLocationError        ),


    /** [`FlagSourceError`].             **/ #[error(transparent)] FlagSourceError            (#[from] FlagSourceError            ),
    /** [`VarSourceError`].              **/ #[error(transparent)] VarSourceError             (#[from] VarSourceError             ),
    /** [`SetSourceError`].              **/ #[error(transparent)] SetSourceError             (#[from] SetSourceError             ),
    /** [`SetNotFound`].                 **/ #[error(transparent)] SetNotFound                (#[from] SetNotFound                ),
    /** [`PartitioningSourceError`].     **/ #[error(transparent)] PartitioningSourceError    (#[from] PartitioningSourceError    ),
    /** [`PartitioningNotFound`].        **/ #[error(transparent)] PartitioningNotFound       (#[from] PartitioningNotFound       ),


    /** [`UrlPartNotFound`].             **/ #[error(transparent)] UrlPartNotFound            (#[from] UrlPartNotFound            ),
    /** [`PathSegmentNotFound`].         **/ #[error(transparent)] PathSegmentNotFound        (#[from] PathSegmentNotFound        ),
    /** [`QueryNotFound`].               **/ #[error(transparent)] QueryNotFound              (#[from] QueryNotFound              ),
    /** [`QueryParamNotFound`].          **/ #[error(transparent)] QueryParamNotFound         (#[from] QueryParamNotFound         ),

    /** [`FunctionNotFound`].            **/ #[error(transparent)] FunctionNotFound           (#[from] FunctionNotFound           ),
    /** [`NotInFunction`].               **/ #[error(transparent)] NotInFunction              (#[from] NotInFunction              ),
    /** [`FunctionArgFunctionNotFound`]. **/ #[error(transparent)] FunctionArgFunctionNotFound(#[from] FunctionArgFunctionNotFound),

    /** [`ExternError`]. **/ #[error(transparent)] Extern(#[from] ExternError),
}

impl From<TryElseError<Self>> for ConditionError {fn from(value: TryElseError<Self>) -> Self {Box::new(value).into()}}
