//! [`HttpResponseHandler`].

use serde::{Deserialize, Serialize};
use thiserror::Error;
#[expect(unused_imports, reason = "Used in doc comments.")]
use reqwest::header::HeaderValue;

use crate::prelude::*;

/// What part of a response a [`HttpRequestConfig`] should return.
///
/// Defaults to [`Self::Body`].
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Suitability)]
#[serde(deny_unknown_fields)]
pub enum HttpResponseHandler {
    /// Get the response body.
    /// # Errors
    /// If the call to [`reqwest::blocking::Response::text`] returns an error, that error is returned.
    #[default]
    Body,
    /// Get the specified header.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`ResponseHandlerError::StringSourceIsNone`].
    ///
    /// If the header isn't found, returns the error [`ResponseHandlerError::HeaderNotFound`].
    ///
    /// If the call to [`HeaderValue::to_str`] returns an error, that error is returned.
    Header(StringSource),
    /// Get the final URL.
    Url,
    /// Get the specified cookie.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`ResponseHandlerError::CookieNotFound`].
    Cookie(StringSource)
}

/// The enum of errors [`HttpResponseHandler::handle`] can return.
#[derive(Debug, Error)]
pub enum ResponseHandlerError {
    /// Returned when a [`reqwest::Error`] is encountered.
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(Box<StringSourceError>),
    /// Returned when a call to [`StringSource::get`] returns [`None`] where it has to return [`Some`].
    #[error("A StringSource was None where it has to be Some.")]
    StringSourceIsNone,
    /// Returned when a requested header isn't found.
    #[error("The requested header was not found.")]
    HeaderNotFound,
    /// Returned when a [`reqwest::header::ToStrError`] is encountered.
    #[error(transparent)]
    ToStrError(#[from] reqwest::header::ToStrError),
    /// Returned when a requested cookie isn't found.
    #[error("The requested cookie was not found.")]
    CookieNotFound
}

impl From<StringSourceError> for ResponseHandlerError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

impl HttpResponseHandler {
    /// Gets the specified part of a [`reqwest::blocking::Response`].
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn handle<'j>(&'j self, response: reqwest::blocking::Response, task_state: &TaskState<'j>) -> Result<String, ResponseHandlerError> {
        Ok(match self {
            Self::Body => response.text()?,
            Self::Header(name) => response.headers().get(get_str!(name, task_state, ResponseHandlerError)).ok_or(ResponseHandlerError::HeaderNotFound)?.to_str()?.to_string(),
            Self::Url => response.url().as_str().to_string(),
            Self::Cookie(source) => {
                let name = get_string!(source, task_state, ResponseHandlerError);
                response.cookies().find(|cookie| cookie.name()==name).ok_or(ResponseHandlerError::CookieNotFound)?.value().to_string()
            }
        })
    }
}
