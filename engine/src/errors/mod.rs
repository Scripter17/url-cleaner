//! Error types.

use crate::prelude::*;

mod data      ; pub use data      ::*;
mod components; pub use components::*;
mod job       ; pub use job       ::*;
mod parsing   ; pub use parsing   ::*;
mod regex     ; pub use regex     ::*;

#[cfg(feature = "http" )] mod http ; #[cfg(feature = "http" )] pub use http ::*;
#[cfg(feature = "cache")] mod cache; #[cfg(feature = "cache")] pub use cache::*;



/// Returned when an error is explicitly requested.
#[derive(Debug, Error)]
#[error("Explicit error: {0:?}")]
pub struct ExplicitError(pub String);

/// Retuerned when an assert fails.
#[derive(Debug, Error)]
#[error("Assert error: {0:?}")]
pub struct AssertError(pub String);

/// Returned when both a `TryElse`'s `try` and `else` return an error.
#[derive(Debug, Error)]
#[error("Both a TryElse's try and else returned an error: {try_error:?} + {else_error:?}")]
pub struct TryElseError<E> {
    /** The try's error.  **/ pub try_error : E,
    /** The else's error. **/ pub else_error: E,
}

/// Returned when all components in a `FirstNotError` variant fail.
#[derive(Debug, Error)]
#[error("All components in a FirstNotError vairant failed: {0:?}")]
pub struct FirstNotErrorErrors<E>(pub Vec<E>);



/// Returned when a subject is [`None`] when it has to be [`Some`].
#[derive(Debug, Error)]
#[error("A subject was None when it had to be Some.")]
pub struct SubjectIsNone;



/** Returned when a required [`UrlPart`] isn't found.      **/ #[derive(Debug, Error)] #[error("A required UrlPart wasn't found."     )] pub struct UrlPartNotFound     ;
/** Returned when a required string isn't found.           **/ #[derive(Debug, Error)] #[error("A required string wasn't found."      )] pub struct StringNotFound      ;
/** Returned when a required [`List`] isn't found.         **/ #[derive(Debug, Error)] #[error("A required List wasn't found."        )] pub struct ListNotFound        ;
/** Returned when a required [`Set`] isn't found.          **/ #[derive(Debug, Error)] #[error("A required Set wasn't found."         )] pub struct SetNotFound         ;
/** Returned when a required [`Map`] isn't found.          **/ #[derive(Debug, Error)] #[error("A required Map wasn't found."         )] pub struct MapNotFound         ;
/** Returned when a required [`Partitioning`] isn't found. **/ #[derive(Debug, Error)] #[error("A required Partitioning wasn't found.")] pub struct PartitioningNotFound;



/// Returned when a function isn't found.
#[derive(Debug, Error)]
#[error("The function wasn't found.")]
pub struct FunctionNotFound;

/// Returned when attempting to use a [`FunctionArgs`] outside a function.
#[derive(Debug, Error)]
#[error("Attempted to use a FunctionArgs outside a function.")]
pub struct NotInFunction;

/// Returned when a [`FunctionArgs`] function isn't found.
#[derive(Debug, Error)]
#[error("The FunctionArgs function wasn't found.")]
pub struct FunctionArgFunctionNotFound;



/// Returned when a string is [`None`] when it has to be [`Some`].
#[derive(Debug, Error)]
#[error("A was None when it had to be Some.")]
pub struct StringIsNone;



/// Returned when a required substring isn't found.
#[derive(Debug, Error)]
#[error("A required substring wasn't found.")]
pub struct SubstringNotFound;

/// Returned when a required query isn't found.
#[derive(Debug, Error)]
#[error("A required query wasn't found.")]
pub struct QueryNotFound;

/// Returned when a required query param isn't found.
#[derive(Debug, Error)]
#[error("A required query param wasn't found.")]
pub struct QueryParamNotFound;

/// Returned when a required path segment isn't found.
#[derive(Debug, Error)]
#[error("A required path segment wasn't found.")]
pub struct PathSegmentNotFound;

/// Returned when attempting to make an invalid [`Radix`].
#[derive(Debug, Error)]
#[error("Attempted to make an invalid Radix of base {0}.")]
pub struct InvalidRadix(pub u8);

/// An error from an Extern variant.
#[derive(Debug, Error)]
#[error("External error: {0:?}")]
pub struct ExternError(#[from] pub Box<dyn std::error::Error + Send + Sync + 'static>);

impl ExternError {
    /// Make a new [`Self`].
    pub fn new<T: std::error::Error + Send + Sync + 'static>(value: T) -> Self {
        Self(Box::new(value))
    }
}
