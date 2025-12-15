//! [`TaskConfig`].

use std::borrow::Cow;
use std::error::Error;

use url::Url;
use thiserror::Error;

use crate::prelude::*;

/// A type that can be made into a [`Task`].
///
/// The implementations for the vairious string and byte slice types parses them as individual task lines per the standard format.
/// # Tests
/// ```
/// use url_cleaner_engine::prelude::*;
///
///         r#"https://example.com"#   .make_task().unwrap();
///        r#""https://example.com""#  .make_task().unwrap();
/// r#"{"url":"https://example.com"}"# .make_task().unwrap();
///
///         br#"https://example.com"#  .make_task().unwrap();
///        br#""https://example.com""# .make_task().unwrap();
/// br#"{"url":"https://example.com"}"#.make_task().unwrap();
///
/// serde_json::json!{          "https://example.com"   }.make_task().unwrap();
/// serde_json::json!{       r#""https://example.com""# }.make_task().unwrap();
/// serde_json::json!{   {"url":"https://example.com"}  }.make_task().unwrap();
/// serde_json::json!{r#"{"url":"https://example.com"}"#}.make_task().unwrap();
/// ```
pub trait TaskConfig {
    /// Make a [`Task`].
    ///
    /// Please see [`Task`] for details on how exactly strings and byte slices are parsed.
    /// # Errors
    /// If the input isn't a valid [`Task`], returns [`MakeTaskError`].
    fn make_task(self) -> Result<Task, MakeTaskError>;
}

impl TaskConfig for &'_ str       { fn make_task(self) -> Result<Task, MakeTaskError> {    self .try_into()} }
impl TaskConfig for String        { fn make_task(self) -> Result<Task, MakeTaskError> {(&* self).try_into()} }
impl TaskConfig for &'_ String    { fn make_task(self) -> Result<Task, MakeTaskError> {(&**self).try_into()} }
impl TaskConfig for Cow<'_, str>  { fn make_task(self) -> Result<Task, MakeTaskError> {(&* self).try_into()} }
impl TaskConfig for &'_ [u8]      { fn make_task(self) -> Result<Task, MakeTaskError> {    self .try_into()} }
impl TaskConfig for Vec<u8>       { fn make_task(self) -> Result<Task, MakeTaskError> {(&* self).try_into()} }
impl TaskConfig for &'_ Vec<u8>   { fn make_task(self) -> Result<Task, MakeTaskError> {(&**self).try_into()} }
impl TaskConfig for Cow<'_, [u8]> { fn make_task(self) -> Result<Task, MakeTaskError> {(&* self).try_into()} }

impl TaskConfig for Task      { fn make_task(self) -> Result<Task, MakeTaskError> {Ok(self       )} }
impl TaskConfig for Url       { fn make_task(self) -> Result<Task, MakeTaskError> {Ok(self.into())} }
impl TaskConfig for BetterUrl { fn make_task(self) -> Result<Task, MakeTaskError> {Ok(self.into())} }

impl TaskConfig for serde_json::Value { fn make_task(self) -> Result<Task, MakeTaskError> {self.try_into().map_err(Into::into)} }

impl<T: TaskConfig, E> TaskConfig for Result<T, E> where MakeTaskError: From<E> {
    fn make_task(self) -> Result<Task, MakeTaskError> {
        self?.make_task()
    }
}

/// The enum of errors [`TaskConfig::make_task`] can return.
#[derive(Debug, Error)]
pub enum MakeTaskError {
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when a [`std::str::Utf8Error`] is encountered.
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
    /// Returned when a [`serde_json::Error`] is encountered.
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    /// Returned when a line that was meant to be ignored is't.
    #[error("A line that was meant to be ignored wasn't.")]
    IgnoreLineNotIgnored,
    /// Returned when a line is otherwise invalid.
    #[error("A line was otherwise invalid.")]
    OtherwiseInvalid,
    /// Any other [`Error`].
    #[error(transparent)]
    Other(#[from] Box<dyn Error + Send + Sync>)
}

/// The enums of errors that [`Job::do`] can return.
#[derive(Debug, Error)]
pub enum DoTaskError {
    /// Returned when an [`MakeTaskError`] is encountered.
    #[error(transparent)]
    MakeTaskError(#[from] MakeTaskError),
    /// Returned when an [`ApplyCleanerError`] is encountered.
    #[error(transparent)]
    ApplyCleanerError(#[from] ApplyCleanerError)
}
