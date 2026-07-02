//! [`StringMatcherError`].

use crate::prelude::*;

/// The enum of errors [`StringMatcher::check`] can return.
#[allow(clippy::enum_variant_names, reason = "I disagree.")]
#[derive(Debug, Error)]
pub enum StringMatcherError {
    /** [`ExplicitError`].               **/ #[error(transparent)] ExplicitError              (#[from] ExplicitError              ),
    /** [`TryElseError`].                **/ #[error(transparent)] TryElseError               (#[from] Box<TryElseError<Self>>    ),
    /** [`FirstNotErrorErrors`].         **/ #[error(transparent)] FirstNotErrorErrors        (#[from] FirstNotErrorErrors<Self>  ),

    /** [`SubjectIsNone`]                **/ #[error(transparent)] SubjectIsNone              (#[from] SubjectIsNone              ),
    /** [`StringSourceError`].           **/ #[error(transparent)] StringSourceError          (#[from] StringSourceError          ),
    /** [`StringNotFound`].              **/ #[error(transparent)] StringNotFound             (#[from] StringNotFound             ),
    /** [`StringModificationError`].     **/ #[error(transparent)] StringModificationError    (#[from] StringModificationError    ),
    /** [`StringLocationError`].         **/ #[error(transparent)] StringLocationError        (#[from] StringLocationError        ),
    /** [`CharMatcherError`].            **/ #[error(transparent)] CharMatcherError           (#[from] CharMatcherError           ),
    /** [`ListNotFound`].                **/ #[error(transparent)] ListNotFound               (#[from] ListNotFound               ),
    /** [`SetSourceError`].              **/ #[error(transparent)] SetSourceError             (#[from] SetSourceError             ),
    /** [`SetNotFound`].                 **/ #[error(transparent)] SetNotFound                (#[from] SetNotFound                ),
    /** [`regex::Error`].                **/ #[error(transparent)] RegexError                 (#[from] regex::Error               ),

    /** [`FunctionNotFound`].            **/ #[error(transparent)] FunctionNotFound           (#[from] FunctionNotFound           ),
    /** [`NotInFunction`].               **/ #[error(transparent)] NotInFunction              (#[from] NotInFunction              ),
    /** [`FunctionArgFunctionNotFound`]. **/ #[error(transparent)] FunctionArgFunctionNotFound(#[from] FunctionArgFunctionNotFound),

    /** [`ExternError`].                 **/ #[error(transparent)] Extern                     (#[from] ExternError                ),
}

impl From<TryElseError<Self>> for StringMatcherError {fn from(value: TryElseError<Self>) -> Self {Box::new(value).into()}}
