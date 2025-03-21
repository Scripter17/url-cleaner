//! Provides [`RequestConfig`], [`RequestBody`], and [`ResponseHandler`] which allows for sending HTTP requests and getting strings from their responses.
//! 
//! Enabled by the `http` feature flag.

use std::collections::HashMap;

use url::Url;
use serde::{Deserialize, Serialize};
use reqwest::{Method, header::{HeaderName, HeaderValue, HeaderMap}};
use thiserror::Error;
#[expect(unused_imports, reason = "Used in a doc comment.")]
use reqwest::cookie::Cookie;

use crate::types::*;
use crate::glue::*;
use crate::util::*;

mod string_source_json_value;
pub use string_source_json_value::*;

/// Configuration for how to make a [`reqwest::blocking::RequestBuilder`] from the client built from [`JobStateView::http_client`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize, Suitability)]
pub struct RequestConfig {
    /// The URL to send the request to.
    ///
    /// Defaults to [`JobStateView::url`] via [`StringSource::Part`]`(`[`UrlPart::Whole`]`)`
    #[serde(default = "get_string_source_part_whole", skip_serializing_if = "is_string_source_part_whole")]
    pub url: StringSource,
    /// The HTTP method to use. Defaults to [`Method::GET`].
    #[serde(default, skip_serializing_if = "is_default", with = "method")]
    pub method: Method,
    /// The headers to send in the request in addition to the default headers provided by [`Params::http_client_config`] and [`Self::client_config_diff`].
    /// 
    /// If a call to [`StringSource::get`] returns [`None`], that header is omitted from the request. For a header with an empty value, use [`StringSource::NoneToEmptyString`].
    /// 
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub headers: HashMap<String, StringSource>,
    /// The request body to send. Works with all methods but intended only for [`Method::POST`] requests.
    /// Defaults to [`None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub body: Option<RequestBody>,
    /// The method [`Self::response`] uses to get a [`String`] from the [`reqwest::blocking::Response`]
    /// Defaults to [`ResponseHandler::Body`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub response_handler: ResponseHandler,
    /// Rules for how to make the HTTP client.
    #[serde(default, skip_serializing_if = "is_default")]
    pub client_config_diff: Option<HttpClientConfigDiff>
}

/// Default value for [`RequestConfig::url`].
fn get_string_source_part_whole() -> StringSource {StringSource::Part(UrlPart::Whole)}
/// Checks if `value` is the default value for [`RequestConfig::url`].
fn is_string_source_part_whole(value: &StringSource) -> bool {value == &get_string_source_part_whole()}

/// The enum of all possible errors [`RequestConfig::make`] and [`RequestConfig::response`] can return.
#[derive(Debug, Error)]
pub enum RequestConfigError {
    /// Returned when a [`reqwest::Error`] is encountered.
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    /// Returned when a [`RequestBodyError`] is encountered.
    #[error(transparent)]
    RequestBodyError(#[from] RequestBodyError),
    /// Returned when a call to [`StringSource::get`] returns [`None`] where it has to be [`Some`].
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
    /// Returned when making the headermap fails.
    #[error("Couldn't make the HeaderMap. Name error: {name:?}. Value error: {value:?}.")]
    MakeHeaderMapError {
        /// The error in making the [`HeaderName`], if any..
        name: Option<reqwest::header::InvalidHeaderName>,
        /// The error in making the [`HeaderValue`], if any.
        value: Option<reqwest::header::InvalidHeaderValue>
    }
}

impl From<StringSourceError> for RequestConfigError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

impl RequestConfig {
    /// Makes a [`reqwest::blocking::RequestBuilder`].
    /// # Errors
    /// If the call to [`JobStateView::http_client`] returns an error, that error is returned.
    /// 
    /// If any of the header names in [`Self::headers`] are, once [`str::to_lowercase`] is applied, an invalid [`HeaderName`], the error is returned in a [`RequestConfigError::MakeHeaderMapError`].
    /// 
    /// If any of the calls to [`StringSource::get`] from [`Self::headers`] return an error, that error is returned.
    /// 
    /// If any of the calls to [`StringSource::get`] return an invalid [`HeaderValue`], the error is returned in a [`RequestConfigError::MakeHeaderMapError`].
    /// 
    /// If the call to [`RequestBody::apply`] returns an error, that error is returned.
    pub fn make(&self, job_state: &JobStateView) -> Result<reqwest::blocking::RequestBuilder, RequestConfigError> {
        let mut ret=job_state.http_client(self.client_config_diff.as_ref())?
            .request(
                self.method.clone(),
                Url::parse(get_str!(self.url, job_state, RequestConfigError))?,
            );

        ret = ret.headers(self.headers
            .iter()
            .map(
                |(k, v)| Ok(v.get(job_state)?
                    .map(|v| (
                        HeaderName::from_lowercase(k.to_lowercase().as_bytes()),
                        HeaderValue::from_str(&v)
                    ))
                )
            )
            .filter_map(|x| x.transpose())
            .map(|x| match x {
                Ok((Ok (k), Ok (v))) => Ok((k, v)),
                Ok((Ok (_), Err(v))) => Err(RequestConfigError::MakeHeaderMapError { name: None   , value: Some(v) }),
                Ok((Err(k), Ok (_))) => Err(RequestConfigError::MakeHeaderMapError { name: Some(k), value: None    }),
                Ok((Err(k), Err(v))) => Err(RequestConfigError::MakeHeaderMapError { name: Some(k), value: Some(v) }),
                Err(e) => Err(RequestConfigError::StringSourceError(Box::new(e)))
            })
            .collect::<Result<HeaderMap<_>, _>>()?);
        if let Some(body) = &self.body {ret=body.apply(ret, job_state)?;}
        Ok(ret)
    }

    /// Sends the request then uses [`Self::response_handler`] to get a [`String`] from the [`reqwest::blocking::Response`].
    /// # Errors
    /// If the call to [`Self::make`] returns an error, that error is returned.
    /// 
    /// If the call to [`reqwest::blocking::RequestBuilder::send`] returns an error, that error is returned.
    /// 
    /// If the call to [`ResponseHandler`] returns an error, that error is returned.
    pub fn response(&self, job_state: &JobStateView) -> Result<String, RequestConfigError> {
        Ok(self.response_handler.handle(self.make(job_state)?.send()?, job_state)?)
    }
}

/// The ways one can set the body in an HTTP request.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Suitability)]
pub enum RequestBody {
    /// [`reqwest::blocking::RequestBuilder::body`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned in a [`RequestBodyError::StringSourceError`].
    /// 
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`RequestBodyError::StringSourceIsNone`]`.
    Text(StringSource),
    /// [`reqwest::blocking::RequestBuilder::form`].
    /// # Errors
    /// If a call to [`StringSource::get`] returns an error, that error is returned in a [`RequestBodyError::StringSourceError`].
    /// 
    /// If a call to [`StringSource::get`] returns [`None`], returns the error [`RequestBodyError::StringSourceIsNone`]`.
    Form(HashMap<String, StringSource>),
    /// [`reqwest::blocking::RequestBuilder::json`].
    Json(StringSourceJsonValue)
}

/// The enum of all possible errors [`RequestBody::apply`] can return.
#[derive(Debug, Error)]
pub enum RequestBodyError {
    /// Returned when a [`StringSourceError`] is encountered.
    /// [`Box`]ed to avoid recursive types.
    #[error(transparent)]
    StringSourceError(Box<StringSourceError>),
    /// Returned when a call to [`StringSource::get`] returns [`None`] when it has to be [`Some`].
    #[error("A StringSource was None where it has to be Some.")]
    StringSourceIsNone
}

impl From<StringSourceError> for RequestBodyError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

impl RequestBody {
    /// Applies the specified body to the provided [`reqwest::blocking::RequestBuilder`].
    /// # Errors
    /// See each of [`Self`]'s variant's documentation for details.
    pub fn apply(&self, request: reqwest::blocking::RequestBuilder, job_state: &JobStateView) -> Result<reqwest::blocking::RequestBuilder, RequestBodyError> {
        Ok(match self {
            Self::Text(source) => request.body(get_string!(source, job_state, RequestBodyError)),
            Self::Form(map) => request.form(&map.iter()
                .map(|(k, v_source)| v_source.get(job_state)
                    .map(|maybe_v| maybe_v
                        .map(|v| (k, v.into_owned()))
                    )
                )
                .collect::<Result<Option<HashMap<_, _>>, _>>()?
                .ok_or(RequestBodyError::StringSourceIsNone)?
            ),
            Self::Json(json) => request.json(&json.make(job_state)?)
        })
    }
}

/// The ways one can get a [`String`] from a [`reqwest::blocking::Response`].
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Suitability)]
pub enum ResponseHandler {
    /// [`reqwest::blocking::Response::text`].
    /// # Errors
    /// If [`reqwest::blocking::Response::text`] returns an error, that error is returned.
    #[default]
    Body,
    /// Finds the header in [`reqwest::blocking::Response::headers`] with the specified name and returns its value.
    /// # Errors
    /// If [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`HeaderMap::get`] returns [`None`], returns the error [`ResponseHandlerError::HeaderNotFound`].
    /// 
    /// If the call to [`HeaderValue::to_str`] returns an error, that error is returned.
    Header(StringSource),
    /// [`reqwest::blocking::Response::url`].
    Url,
    /// Finds the cookie in [`reqwest::blocking::Response::cookies`] with the specified name and returns its value.
    /// # Errors
    /// If [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If [`reqwest::blocking::Response::cookies`] returns an iterator that does not contain a [`Cookie`] with the specified name, returns the error [`ResponseHandlerError::CookieNotFound`].
    Cookie(StringSource)
}

/// The enum of all possible errors [`ResponseHandler::handle`] can return.
#[derive(Debug, Error)]
pub enum ResponseHandlerError {
    /// Returned when a [`reqwest::Error`] is encountered.
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(Box<StringSourceError>),
    /// Returned when a call to [`StringSource::get`] returns [`None`] where it has to be [`Some`].
    #[error("A StringSource was None where it has to be Some.")]
    StringSourceIsNone,
    /// Returned when the requested header is not found.
    #[error("The requested header was not found.")]
    HeaderNotFound,
    /// Returned when a [`reqwest::header::ToStrError`] is encountered.
    #[error(transparent)]
    ToStrError(#[from] reqwest::header::ToStrError),
    /// Returned when the requested cookie is not found.
    #[error("The requested cookie was not found.")]
    CookieNotFound
}

impl From<StringSourceError> for ResponseHandlerError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

impl ResponseHandler {
    /// Returns a string from the requested part of the response.
    /// # Errors
    /// See each of [`Self`]'s variant's documentation for details.
    pub fn handle(&self, response: reqwest::blocking::Response, job_state: &JobStateView) -> Result<String, ResponseHandlerError> {
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
