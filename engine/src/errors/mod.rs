//! Error types.

use crate::prelude::*;

mod data; pub use data::*;
mod components; pub use components::*;
mod job       ; pub use job       ::*;
mod parsing   ; pub use parsing   ::*;
mod regex     ; pub use regex     ::*;

#[cfg(feature = "http" )] mod http ; #[cfg(feature = "http" )] pub use http ::*;
#[cfg(feature = "cache")] mod cache; #[cfg(feature = "cache")] pub use cache::*;



/// Returned when an `Error` varaint is run.
#[derive(Debug, Error)]
#[error("Explicit error: {0:?}")]
pub struct ExplicitError(pub String);

/// Returned when both a `TryElse`'s `try` and `else` return an error.
#[derive(Debug, Error)]
#[error("Both a TryElse's try and else returned an error.")]
pub struct TryElseError<E> {
    /// The error returned by the `try`.
    pub try_error: E,
    /// The error returned by the `else`.
    pub else_error: E
}

/// Returned when all components in a `FirstNotError` variant fail.
#[derive(Debug, Error)]
#[error("All components in a FirstNotError vairant failed: {0:?}")]
pub struct FirstNotErrorErrors<E>(pub Vec<E>);



/// Returned when a subject is [`None`] when it has to be [`Some`].
#[derive(Debug, Error)]
#[error("A subject was None when it had to be Some.")]
pub struct SubjectIsNone;

/// Returned when a [`UrlPart`] returns [`None`] when it has to return [`Some`].
#[derive(Debug, Error)]
#[error("A UrlPart returned None when it had to return Some.")]
pub struct UrlPartNotFound;

/// Retuerned when an assert fails.
#[derive(Debug, Error)]
#[error("Assert error: {0:?}")]
pub struct AssertError(pub String);



/// Returned when a [`StringSource`] returned  [`None`] when it has to return [`Some`].
#[derive(Debug, Error)]
#[error("A StringSource returned None when it had to return Some.")]
pub struct StringNotFound;

/// Returned when a [`ListSource`] returns [`None`] when it has to return [`Some`].
#[derive(Debug, Error)]
#[error("A ListSource returned None when it had to return Some.")]
pub struct ListNotFound;

/// Returned when a [`SetSource`] returns [`None`] when it has to return [`Some`].
#[derive(Debug, Error)]
#[error("A SetSource returned None when it had to return Some.")]
pub struct SetNotFound;

/// Returned when a [`MapSource`] returns [`None`] when it has to return [`Some`].
#[derive(Debug, Error)]
#[error("A MapSource returned None when it had to return Some.")]
pub struct MapNotFound;

/// Returned when a [`PartitioningSource`] returns [`None`] when it has to return [`Some`].
#[derive(Debug, Error)]
#[error("A PartitioningSource returned None when it had to return Some.")]
pub struct PartitioningNotFound;



/// Returned when attempting to use a [`FunctionArgs`] outside a function.
#[derive(Debug, Error)]
#[error("Attempted to use a FunctionArgs outside a function.")]
pub struct NotInFunction;

/// Returned when a [`FunctionArgs`] function isn't found.
#[derive(Debug, Error)]
#[error("The FunctionArgs function wasn't found.")]
pub struct FunctionArgFunctionNotFound;

/// Returned when a function isn't found.
#[derive(Debug, Error)]
#[error("The function wasn't found.")]
pub struct FunctionNotFound;

/// Returned when a substring isn't found.
#[derive(Debug, Error)]
#[error("The substring wasn't found.")]
pub struct SubstringNotFound;

/// Returned when a string has to be [`Some`] but is [`None`].
#[derive(Debug, Error)]
#[error("The string had to be Some but was None.")]
pub struct StringIsNone;

/// Returned when a query is required but not found.
#[derive(Debug, Error)]
#[error("A query was required but not found.")]
pub struct QueryNotFound;

/// Returned when a query param is required but not found.
#[derive(Debug, Error)]
#[error("A query was required but not found.")]
pub struct QueryParamNotFound;

/// Returned when a path segment is required but not found.
#[derive(Debug, Error)]
#[error("A path segment was required but not found.")]
pub struct PathSegmentNotFound;

/// Returned when attempting to make an invalid [`Radix`].
#[derive(Debug, Error)]
#[error("Invalid radix: {0}")]
pub struct InvalidRadix(pub u8);

/// An error from an Extern variant.
#[derive(Debug, Error)]
#[error(transparent)]
pub struct ExternError(#[from] pub Box<dyn std::error::Error + Send + Sync>);

impl ExternError {
    /// Make a new [`Self`].
    pub fn new<T: std::error::Error + Send + Sync + 'static>(value: T) -> Self {
        Self(Box::new(value))
    }
}
