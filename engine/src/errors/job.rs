//! Job.

use crate::prelude::*;

/// [`ParamsDiff::load`].
#[derive(Debug, Error)]
pub enum LoadParamsDiffError {
    /** [`io::Error`].         **/ #[error(transparent)] IoError       (#[from] io::Error        ),
    /** [`serde_json::Error`]. **/ #[error(transparent)] SerdeJsonError(#[from] serde_json::Error),
}

/// [`ProfilesConfig::load`].
#[derive(Debug, Error)]
pub enum LoadProfilesConfigError {
    /** [`io::Error`].         **/ #[error(transparent)] IoError       (#[from] io::Error        ),
    /** [`serde_json::Error`]. **/ #[error(transparent)] SerdeJsonErrro(#[from] serde_json::Error),
}

/// [`Cleaner::load`].
#[derive(Debug, Error)]
pub enum LoadCleanerError {
    /** [`io::Error`].         **/ #[error(transparent)] IoError       (#[from] io::Error        ),
    /** [`serde_json::Error`]. **/ #[error(transparent)] SetdeJsonError(#[from] serde_json::Error),
}

/// [`Secrets::load`].
#[derive(Debug, Error)]
pub enum LoadSecretsError {
    /** [`io::Error`].         **/ #[error(transparent)] IoError       (#[from] io::Error        ),
    /** [`serde_json::Error`]. **/ #[error(transparent)] SetdeJsonError(#[from] serde_json::Error),
}

/// [`Cleaner::apply`].
#[derive(Debug, Error)]
pub enum ApplyCleanerError {
    /** [`ActionError`]. **/ #[error(transparent)] ActionError(#[from] ActionError)
}

/// [`Task::try_from`].
#[derive(Debug, Error)]
pub enum MakeTaskError {
    /** [`InvalidUrl`].          **/  #[error(transparent)]  InvalidUrl    (#[from] InvalidUrl         ),
    /** [`std::str::Utf8Error`]. **/  #[error(transparent)]  Utf8Error     (#[from] std::str::Utf8Error),
    /** [`serde_json::Error`].   **/  #[error(transparent)]  SerdeJsonError(#[from] serde_json::Error  ),

    /// Returned when a line that was meant to be ignored is't.
    #[error("A line that was meant to be ignored wasn't.")]
    IgnoreLineNotIgnored,
    /// Returned when a line is otherwise invalid.
    #[error("A line was otherwise invalid.")]
    OtherwiseInvalid,
}

impl From<std::convert::Infallible> for MakeTaskError {
    fn from(value: std::convert::Infallible) -> Self {
        match value {}
    }
}

/// [`JobContext::load`].
#[derive(Debug, Error)]
pub enum LoadJobContextError {
    /** [`io::Error`].         **/ #[error(transparent)] IoError        (#[from] io::Error        ),
    /** [`serde_json::Error`]. **/ #[error(transparent)] SerdeJsonError (#[from] serde_json::Error),
}

/// [`Job::do`].
#[derive(Debug, Error)]
pub enum DoTaskError {
    /** [`MakeTaskError`].     **/ #[error(transparent)] MakeTaskError    (#[from] MakeTaskError    ),
    /** [`ApplyCleanerError`]. **/ #[error(transparent)] ApplyCleanerError(#[from] ApplyCleanerError),
}
