//! HTTP requests.

use std::collections::HashMap;

use url::Url;
use serde::{Deserialize, Serialize};
use reqwest::{Method, header::{HeaderName, HeaderValue}};
use thiserror::Error;
#[expect(unused_imports, reason = "Used in a doc comment.")]
use reqwest::cookie::Cookie;

use crate::types::*;
use crate::glue::*;
use crate::util::*;

/// Rules for making an HTTP request.
///
/// Currently only capable of making blocking requests.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize, Suitability)]
pub struct RequestConfig {
    /// The URL to send the request to.
    ///
    /// Defaults to [`StringSource::Part`]`(`[`UrlPart::Whole`]`)`.
    #[serde(default = "get_string_source_part_whole", skip_serializing_if = "is_string_source_part_whole")]
    pub url: StringSource,
    /// The method to use.
    ///
    /// Defaults to [`Method::GET`].
    #[serde(default, skip_serializing_if = "is_default", with = "serde_method")]
    pub method: Method,
    /// The headers to send in addition to the default headers from the [`HttpClientConfig`] and [`Self::client_config_diff`].
    ///
    /// If a call to [`StringSource::get`] returns [`None`], the header it came from isn't sent. This can be useful for API keys.
    /// 
    /// Defaults to an empty set.
    #[serde(default, skip_serializing_if = "is_default")]
    pub headers: HashMap<String, StringSource>,
    /// The body to send.
    ///
    /// Defaults to [`None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub body: Option<RequestBody>,
    /// What to part of the response to return.
    ///
    /// Defaults to [`ResponseHandler::body`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub response_handler: ResponseHandler,
    /// Overrides for the [`HttpClientConfig`] this uses to make the [`reqwest::blocking::Client`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub client_config_diff: Option<HttpClientConfigDiff>
}

/// Serde helper function for [`RequestConfig::url`].
fn get_string_source_part_whole() -> StringSource {StringSource::Part(UrlPart::Whole)}
/// Serde helper function for [`RequestConfig::url`].
fn is_string_source_part_whole(value: &StringSource) -> bool {value == &get_string_source_part_whole()}

/// The enum of erros [`RequestConfig::make`] can return.
#[derive(Debug, Error)]
pub enum MakeHttpRequestError {
    /// Returned when a [`reqwest::Error`] is encountered.
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    /// Returned when a [`ReqwestBodyError`] is encountered.
    #[error(transparent)]
    RequestBodyError(#[from] RequestBodyError),
    /// Returned when a call to [`StringSource::get`] returns [`None`] where it has to return [`Some`].
    #[error("A StringSource was None where it has to be Some.")]
    StringSourceIsNone,
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] Box<StringSourceError>),
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when a [`ResponseHandlerError`] is encountered.
    #[error(transparent)]
    ResponseHandlerError(#[from] ResponseHandlerError),
    /// Returned when a [`reqwest::header::InvalidHeaderName`] is encountered.
    #[error(transparent)]
    InvalieHeaderName(#[from] reqwest::header::InvalidHeaderName),
    /// Returned when a [`reqwest::header::InvalidHeaderValue`] is encountered.
    #[error(transparent)]
    InvalieHeaderValue(#[from] reqwest::header::InvalidHeaderValue)
}

/// The enum of errors [`RequestConfig::send`] can return.
#[derive(Debug, Error)]
pub enum SendHttpRequestError {
    /// Returned when a [`MakeHttpRequestError`] is encountered.
    #[error(transparent)]
    MakeHttpRequestError(#[from] MakeHttpRequestError),
    /// Returned when a [`reqwest::Error`] is encountered.
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error)
}

/// The enum of errors [`RequestConfig::response`] can return.
#[derive(Debug, Error)]
pub enum HttpResponseError {
    /// Returned when a [`SendHttpRequestError`] is encountered.
    #[error(transparent)]
    SendHttpRequestError(#[from] SendHttpRequestError),
    /// Returned when a [`reqwest::Error`] is encountered.
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    /// Returned when a [`ResponseHandlerError`] is encountered.
    #[error(transparent)]
    ResponseHandlerError(#[from] ResponseHandlerError)
}

impl From<StringSourceError> for MakeHttpRequestError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

impl RequestConfig {
    /// Makes the request.
    /// # Errors
    /// If the call to [`TaskStateView::http_client`] returns an error, that error is returned.
    ///
    /// If [`Self::url`]'s call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If [`Self::url`]'s call to [`StringSource::get`] returns [`None`], returns the error [`RequestConfigError::StringSourceIsNone`].
    /// 
    /// If any of [`Self::headers`]'s calls to [`StringSource::get`] returun an error, that error is returned.
    ///
    /// If any of [`Self::headers`]'s calls to [`HeaderName::try_from`] returns an error, that error is returned.
    ///
    /// If the call to [`RequestBody::apply`] returns an error, that error is returned.
    pub fn make(&self, job_state: &TaskStateView) -> Result<reqwest::blocking::RequestBuilder, MakeHttpRequestError> {
        let mut ret=job_state.http_client(self.client_config_diff.as_ref())?
            .request(
                self.method.clone(),
                Url::parse(get_str!(self.url, job_state, MakeHttpRequestError))?,
            );
        for (name, value) in self.headers.iter() {
            if let Some(value) = value.get(job_state)? {
                ret = ret.header(HeaderName::try_from(name)?, HeaderValue::try_from(value.into_owned())?);
            }
        }
        if let Some(body) = &self.body {ret=body.apply(ret, job_state)?;}
        Ok(ret)
    }

    /// Makes and sends the request.
    /// # Errors
    /// If the call to [`Self::make`] returns an error, that error is returned.
    ///
    /// If the call to [`reqwest::blocking::RequestBuilder::send`] returns an error, that error is returned.
    pub fn send(&self, job_state: &TaskStateView) -> Result<reqwest::blocking::Response, SendHttpRequestError> {
        Ok(self.make(job_state)?.send()?)
    }

    /// Make the request, send it, and return the response specified by [`Self::response_handler`].
    /// # Errors
    /// If the call to [`Self::send`] returns an error, that error is returned.
    ///
    /// If the call to [`RequestHandler::handle`} returns an error, that error is returned.
    pub fn response(&self, job_state: &TaskStateView) -> Result<String, HttpResponseError> {
        Ok(self.response_handler.handle(self.send(job_state)?, job_state)?)
    }
}

/// How a [`RequestConfig`] should construct its body.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Suitability)]
pub enum RequestBody {
    /// Send the specified text.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`RequestBodyError::StringSourceIsNone`].
    Text(StringSource),
    /// Sends the HTML form.
    ///
    /// If a call to [`StringSource::get`] returns [`None`], the field it came from isn't sent. This can be useful for API keys.
    /// # Errors
    /// If a call to [`StringSource::get`] returns an error, that error is returned.
    Form(HashMap<String, StringSource>),
    /// Sends JSON.
    /// # Errors
    /// If the call to [`StringSourceJsonValue::make`] returns an error, that error is returned.
    Json(StringSourceJsonValue)
}

/// The enum of errors [`RequestBody::apply`] can return.
#[derive(Debug, Error)]
pub enum RequestBodyError {
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(Box<StringSourceError>),
    /// Returned when a call to [`StringSource::get`] returns [`None`] where it must return [`Some`].
    #[error("A StringSource was None where it has to be Some.")]
    StringSourceIsNone
}

impl From<StringSourceError> for RequestBodyError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

impl RequestBody {
    /// Inserts the specified body into a [`reqwest::blocking::ReqwestBuilder`].
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn apply(&self, request: reqwest::blocking::RequestBuilder, job_state: &TaskStateView) -> Result<reqwest::blocking::RequestBuilder, RequestBodyError> {
        Ok(match self {
            Self::Text(source) => request.body(get_string!(source, job_state, RequestBodyError)),
            Self::Form(map) => {
                let mut ret = HashMap::new();
                for (k, v) in map.iter() {
                    if let Some(v) = v.get(job_state)? {
                        ret.insert(k, v);
                    }
                }
                request.form(&ret)
            },
            Self::Json(json) => request.json(&json.make(job_state)?)
        })
    }
}

/// What part of a response an [`HttpRequestConfig`] should return.
///
/// Defaults to [`Self::Body`].
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Suitability)]
pub enum ResponseHandler {
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

/// The enum of errors [`ResponseHandler::handle`] can return.
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

impl ResponseHandler {
    /// Gets the specified part of a [`reqwest::blocking::Response`].
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn handle(&self, response: reqwest::blocking::Response, job_state: &TaskStateView) -> Result<String, ResponseHandlerError> {
        Ok(match self {
            Self::Body => response.text()?,
            Self::Header(source) => response.headers().get(get_str!(source, job_state, ResponseHandlerError)).ok_or(ResponseHandlerError::HeaderNotFound)?.to_str()?.to_string(),
            Self::Url => response.url().as_str().to_string(),
            Self::Cookie(source) => {
                let name = get_string!(source, job_state, ResponseHandlerError);
                response.cookies().find(|cookie| cookie.name()==name).ok_or(ResponseHandlerError::CookieNotFound)?.value().to_string()
            }
        })
    }
}
