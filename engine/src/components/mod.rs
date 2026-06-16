//! Components.

/// Returned when both a `TryElse`'s `try` and `else` return an error.
#[derive(Debug, Error)]
#[error("Both a TryElse's try and else returned an error.")]
pub struct TryElseError<E> {
    /// The error returned by the `try`.
    pub try_error: E,
    /// The error returned by the `else`.
    pub else_error: E
}

impl From<TryElseError<Self>> for ConditionError          {fn from(value: TryElseError<Self>) -> Self {Box::new(value).into()}}
impl From<TryElseError<Self>> for ActionError             {fn from(value: TryElseError<Self>) -> Self {Box::new(value).into()}}
impl From<TryElseError<Self>> for StringSourceError       {fn from(value: TryElseError<Self>) -> Self {Box::new(value).into()}}
impl From<TryElseError<Self>> for StringModificationError {fn from(value: TryElseError<Self>) -> Self {Box::new(value).into()}}
impl From<TryElseError<Self>> for StringMatcherError      {fn from(value: TryElseError<Self>) -> Self {Box::new(value).into()}}

/// Returned when the subject is [`None`] when it has to be [`Some`].
#[derive(Debug, Error)]
#[error("The subject was None when it had to be Some.")]
pub struct SubjectIsNone;

/// Returned when a URL part isn't found.
#[derive(Debug, Error)]
#[error("The URL part wasn't found.")]
pub struct UrlPartNotFound;

/// Retuerned when an `AssertMatches` variant fails.
#[derive(Debug, Error)]
#[error("Assert matches error: {0}")]
pub struct AssertMatchesError(pub String);

/// Returned when an `Error` varaint is run.
#[derive(Debug, Error)]
#[error("Explicit error: {0}")]
pub struct ExplicitError(pub String);

/// Returned when all components in a `FirstNotError` variant fail.
#[derive(Debug, Error)]
#[error("All components in a FirstNotError vairant failed.")]
pub struct FirstNotErrorErrors<E>(pub Vec<E>);

/// Returned when a [`StringSource`] is [`None`] when it has to be [`Some`].
#[derive(Debug, Error)]
#[error("A StringSource was None when it had to be Some.")]
pub struct StringNotFound;

/// Returned when a [`List`] isn't found.
#[derive(Debug, Error)]
#[error("The List wasn't found.")]
pub struct ListNotFound;

/// Returned when a [`Set`] isn't found.
#[derive(Debug, Error)]
#[error("The Set wasn't found.")]
pub struct SetNotFound;

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

/// Returned when a [`Map`] isn't found.
#[derive(Debug, Error)]
#[error("The Map wasn't found.")]
pub struct MapNotFound;

/// Returned when a [`Partitioning`] isn't found.
#[derive(Debug, Error)]
#[error("The Partitioning wasn't found.")]
pub struct PartitioningNotFound;

/// Returned when a substring isn't found.
#[derive(Debug, Error)]
#[error("The substring wasn't found.")]
pub struct SubstringNotFound;

/// Returned when a string has to be [`Some`] but is [`None`].
#[derive(Debug, Error)]
#[error("The string had to be Some but was None.")]
pub struct StringIsNone;

use crate::prelude::*;

/// The function pointer type for [`Condition::Custom`].
pub type ConditionExtern          =             fn(&    TaskState   , Option<&   FunctionArgs>                           ) -> Result<bool                , ConditionError         >;
/// The function pointer type for [`Action::Custom`].
pub type ActionExtern             =             fn(&mut TaskState   , Option<&   FunctionArgs>                           ) -> Result<bool                , ActionError            >;
/// The function pointer type for [`StringMatcher::Custom`].
pub type StringMatcherExtern      =             fn(&   TaskState    , Option<&   FunctionArgs>,      Option<&str>        ) -> Result<bool                , StringMatcherError     >;
/// The function pointer type for [`StringModification::Custom`].
pub type StringModificationExtern = for<'j, 't> fn(&'t TaskState<'j>, Option<&'j FunctionArgs>, &mut Option<Cow<'t, str>>) -> Result<bool                , StringModificationError>;
/// The function pointer type for [`StringSource::Custom`].
pub type StringSourceExtern       = for<'j, 't> fn(&'t TaskState<'j>, Option<&'j FunctionArgs>                           ) -> Result<Option<Cow<'t, str>>, StringSourceError      >;

mod sources;
pub use sources::*;

mod strings;
pub use strings::*;

mod condition;
mod action;
pub use condition::*;
pub use action::*;


mod set;
mod map;
mod partitioning;

pub use set::*;
pub use map::*;
pub use partitioning::*;

mod url_part;
mod host_part;
mod query_param_selector;


mod regex;
mod base64;

mod parsing;

pub use url_part::*;
pub use host_part::*;
pub use query_param_selector::*;


pub use regex::prelude::*;
pub use base64::prelude::*;

pub use parsing::prelude::*;
