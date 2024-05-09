//! Provides [`RequestConfig`], [`RequestBody`], and [`ResponseHandler`] which allows for sending HTTP requests and getting strings from their responses.

use std::collections::HashMap;

use url::Url;
use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use reqwest::{Method, header::HeaderMap};
use thiserror::Error;
#[allow(unused_imports)] // Used for documentation.
use reqwest::{header::HeaderValue, cookie::Cookie};

use crate::types::*;
use crate::glue::*;
use crate::util::*;

/// Configuration for how to make a [`reqwest::blocking::RequestBuilder`] from the client built from [`Params::http_client`].
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct RequestConfig {
    /// The URL to send the request to. If [`None`], uses the URL being cleaned. Defaults to [`None`].
    #[cfg(feature = "string-source")]
    #[serde(default)]
    pub url: Option<StringSource>,
    /// The URL to send the request to. If [`None`], uses the URL being cleaned. Defaults to [`None`].
    #[cfg(not(feature = "string-source"))]
    #[serde(default)]
    pub url: Option<String>,
    /// The HTTP method to use. Defaults to [`Method::GET`].
    #[serde(default, with = "method")]
    pub method: Method,
    /// The headers to send in the request in addition to the default headers provided by [`Params::http_client_config`] and [`Self::client_config_diff`].
    /// Defaults to an empty [`HeaderMap`].
    #[serde(default, with = "headermap")]
    pub headers: HeaderMap,
    /// The request body to send. Works with all methods but intended only for [`Method::POST`] requests.
    /// Defaults to [`None`].
    #[serde(default)]
    pub body: Option<RequestBody>,
    /// The method [`Self::response`] uses to get a [`String`] from the [`reqwest::blocking::Response`]
    /// Defaults to [`ResponseHandler::Body`].
    #[serde(default)]
    pub response_handler: ResponseHandler,
    /// Rules for how to make the HTTP client.
    #[serde(default)]
    pub client_config_diff: Option<HttpClientConfigDiff>
}

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
    #[cfg(feature = "string-source")]
    #[error("A StringSource was None where it has to be Some.")]
    StringSourceIsNone,
    /// Returned when a [`StringSourceError`] is encountered.
    #[cfg(feature = "string-source")]
    #[error(transparent)]
    StringSourceError(Box<StringSourceError>),
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when a [`ResponseHandlerError`] is encountered.
    #[error(transparent)]
    ResponseHandlerError(#[from] ResponseHandlerError)
}

#[cfg(feature = "string-source")]
impl From<StringSourceError> for RequestConfigError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

impl RequestConfig {
    /// Makes a [`reqwest::blocking::RequestBuilder`].
    /// # Errors
    /// If the call to [`Params::http_client`] returns an error, that error is returned.
    /// 
    /// If the call to [`RequestBody::apply`] returns an error, that error is returned.
    pub fn make(&self, url: &Url, params: &Params) -> Result<reqwest::blocking::RequestBuilder, RequestConfigError> {
        let mut ret=params.http_client(self.client_config_diff.as_ref())?.request(self.method.clone(), match self.url {Some(ref source) => Url::parse(get_string!(source, url, params, RequestConfigError))?, None => url.clone()});

        ret = ret.headers(self.headers.clone());
        if let Some(body) = &self.body {ret=body.apply(ret, url, params)?;}
        Ok(ret)
    }

    /// Sends the request then uses [`Self::response_handler`] to get a [`String`] from the [`reqwest::blocking::Response`].
    /// # Errors
    /// If the call to [`Self::make`] returns an error, that error is returned.
    /// 
    /// If the call to [`reqwest::blocking::RequestBuilder::send`] returns an error, that error is returned.
    /// 
    /// If the call to [`ResponseHandler`] returns an error, that error is returned.
    pub fn response(&self, url: &Url, params: &Params) -> Result<String, RequestConfigError> {
        Ok(self.response_handler.handle(self.make(url, params)?.send()?, url, params)?)
    }
}

/// The ways one can set the body in an HTTP request.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum RequestBody {
    /// [`reqwest::blocking::RequestBuilder::body`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned in a [`RequestBodyError::StringSourceError`].
    /// 
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`RequestBodyError::StringSourceIsNone`]`.
    #[cfg(feature = "string-source")]
    Text(StringSource),
    /// [`reqwest::blocking::RequestBuilder::body`].
    #[cfg(not(feature = "string-source"))]
    Text(String),
    /// [`reqwest::blocking::RequestBuilder::form`].
    /// # Errors
    /// If a call to [`StringSource::get`] returns an error, that error is returned in a [`RequestBodyError::StringSourceError`].
    /// 
    /// If a call to [`StringSource::get`] returns [`None`], returns the error [`RequestBodyError::StringSourceIsNone`]`.
    #[cfg(feature = "string-source")]
    Form(HashMap<String, StringSource>),
    /// [`reqwest::blocking::RequestBuilder::form`].
    #[cfg(not(feature = "string-source"))]
    Form(HashMap<String, String>),
    /// [`reqwest::blocking::RequestBuilder::json`].
    Json(Value)
}

/// The enum of all possible errors [`RequestBody::apply`] can return.
#[derive(Debug, Error)]
pub enum RequestBodyError {
    /// Returned when a [`StringSourceError`] is encountered.
    /// [`Box`]ed to avoid recursive types.
    #[cfg(feature = "string-source")]
    #[error(transparent)]
    StringSourceError(Box<StringSourceError>),
    /// Returned when a call to [`StringSource::get`] returns [`None`] when it has to be [`Some`].
    #[cfg(feature = "string-source")]
    #[error("A StringSource was None where it has to be Some.")]
    StringSourceIsNone
}

#[cfg(feature = "string-source")]
impl From<StringSourceError> for RequestBodyError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

impl RequestBody {
    /// Applies the specified body to the provided [`reqwest::blocking::RequestBuilder`].
    /// # Errors
    /// See each of [`Self`]'s variant's documentation for details.
    pub fn apply(&self, request: reqwest::blocking::RequestBuilder, url: &Url, params: &Params) -> Result<reqwest::blocking::RequestBuilder, RequestBodyError> {
        Ok(match self {
            Self::Text(source) => request.body(get_string!(source, url, params, RequestBodyError).to_string()),
            #[cfg(feature = "string-source")]
            Self::Form(map) => request.form(&map.iter()
                .map(|(k, v_source)| v_source.get(url, params)
                    .map(|maybe_v| maybe_v
                        .map(|v| (k, v.into_owned()))
                    )
                )
                .collect::<Result<Option<HashMap<_, _>>, _>>()?
                .ok_or(RequestBodyError::StringSourceIsNone)?
            ),
            #[cfg(not(feature = "string-source"))]
            Self::Form(map) => request.form(map),
            Self::Json(json) => request.json(json)
        })
    }
}

/// The ways one can get a [`String`] from a [`reqwest::blocking::Response`].
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
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
    #[cfg(feature = "string-source")]
    Header(StringSource),
    /// Finds the header in [`reqwest::blocking::Response::headers`] with the specified name and returns its value.
    /// # Errors
    /// If the call to [`HeaderMap::get`] returns [`None`], returns the error [`ResponseHandlerError::HeaderNotFound`].
    /// 
    /// If the call to [`HeaderValue::to_str`] returns an error, that error is returned.
    #[cfg(not(feature = "string-source"))]
    Header(String),
    /// [`reqwest::blocking::Response::url`].
    Url,
    /// Finds the cookie in [`reqwest::blocking::Response::cookies`] with the specified name and returns its value.
    /// # Errors
    /// If [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If [`reqwest::blocking::Response::cookies`] returns an iterator that does not contain a [`Cookie`] with the specified name, returns the error [`ResponseHandlerError::CookieNotFound`].
    #[cfg(feature = "string-source")]
    Cookie(StringSource),
    /// Finds the cookie in [`reqwest::blocking::Response::cookies`] with the specified name and returns its value.
    /// # Errors
    /// If [`reqwest::blocking::Response::cookies`] returns an iterator that does not contain a [`Cookie`] with the specified name, returns the error [`ResponseHandlerError::CookieNotFound`].
    #[cfg(not(feature = "string-source"))]
    Cookie(String)
}

/// The enum of all possible errors [`ResponseHandler::handle`] can return.
#[derive(Debug, Error)]
pub enum ResponseHandlerError {
    /// Returned when a [`reqwest::Error`] is encountered.
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    /// Returned when a [`StringSourceError`] is encountered.
    #[cfg(feature = "string-source")]
    #[error(transparent)]
    StringSourceError(Box<StringSourceError>),
    /// Returned when a call to [`StringSource::get`] returns [`None`] where it has to be [`Some`].
    #[cfg(feature = "string-source")]
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

#[cfg(feature = "string-source")]
impl From<StringSourceError> for ResponseHandlerError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

impl ResponseHandler {
    /// Returns a string from the requested part of the response.
    /// # Errors
    /// See each of [`Self`]'s variant's documentation for details.
    pub fn handle(&self, response: reqwest::blocking::Response, url: &Url, params: &Params) -> Result<String, ResponseHandlerError> {
        Ok(match self {
            Self::Body => response.text()?,
            Self::Header(source) => response.headers().get(get_string!(source, url, params, ResponseHandlerError)).ok_or(ResponseHandlerError::HeaderNotFound)?.to_str()?.to_string(),
            Self::Url => response.url().as_str().to_string(),
            Self::Cookie(source) => {
                let name = get_string!(source, url, params, ResponseHandlerError);
                response.cookies().find(|cookie| cookie.name()==name).ok_or(ResponseHandlerError::CookieNotFound)?.value().to_string()
            }
        })
    }
}
