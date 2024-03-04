use std::str::FromStr;
use std::collections::HashMap;

use url::{Url, ParseError};
use serde::{Deserialize, Serialize, de::{Deserializer, Error as _}, ser::Serializer};
use serde_json::value::Value;
use reqwest::{Method, header::{HeaderMap, ToStrError}, blocking::RequestBuilder, Error as ReqwestError, blocking::Response};
use thiserror::Error;

use crate::types::*;
use crate::glue::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RequestConfig {
    #[cfg(feature = "string-source")]
    #[serde(default)]
    pub url: Option<StringSource>,
    #[cfg(not(feature = "string-source"))]
    #[serde(default)]
    pub url: Option<String>,
    #[serde(deserialize_with = "deserialize_method", serialize_with = "serialize_method")]
    pub method: Method,
    #[serde(with = "headermap")]
    pub headers: HeaderMap,
    pub body: Option<RequestBody>
}

#[derive(Debug, Error)]
pub enum RequestConfigError {
    #[error(transparent)]
    ReqwestError(#[from] ReqwestError),
    #[error(transparent)]
    RequestBodyError(#[from] RequestBodyError),
    #[cfg(feature = "string-source")]
    #[error("TODO")]
    StringSourceIsNone,
    #[cfg(feature = "string-source")]
    #[error(transparent)]
    StringSourceError(Box<StringSourceError>),
    #[error(transparent)]
    UrlParseError(#[from] ParseError)
}

#[cfg(feature = "string-source")]
impl From<StringSourceError> for RequestConfigError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

fn deserialize_method<'de, D: Deserializer<'de>>(d: D) -> Result<Method, D::Error> {
    Method::from_str(Deserialize::deserialize(d)?).map_err(D::Error::custom)
}

fn serialize_method<S: Serializer>(method: &Method, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(method.as_str())
}

impl RequestConfig {
    /// # Errors
    /// If the call to [`Params::http_client`] returns an error, that error is returned.
    /// If the call to [`RequestBody::apply`] returns an error, that error is returned.
    pub fn make(&self, url: &Url, params: &Params) -> Result<RequestBuilder, RequestConfigError> {
        #[cfg(feature = "string-source")]
        let mut ret=params.http_client()?.request(self.method.clone(), match self.url {Some(ref source) => Url::parse(&source.get(url, params, false)?.ok_or(RequestConfigError::StringSourceIsNone)?)?, None => url.clone()});
        #[cfg(not(feature = "string-source"))]
        let mut ret=params.http_client()?.request(self.method.clone(), match self.url {Some(ref url) => Url::parse(url)?, None => url.clone()});

        ret = ret.headers(self.headers.clone());
        if let Some(body) = &self.body {ret=body.apply(ret, url, params)?;}
        Ok(ret)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum RequestBody {
    /// # Errors
    /// TODO
    #[cfg(feature = "string-source")]
    Text(StringSource),
    #[cfg(not(feature = "string-source"))]
    Text(String),
    /// # Errors
    /// TODO
    #[cfg(feature = "string-source")]
    Form(HashMap<String, StringSource>),
    #[cfg(not(feature = "string-source"))]
    Form(HashMap<String, String>),
    Json(Value)
}

#[derive(Debug, Error)]
pub enum RequestBodyError {
    #[cfg(feature = "string-source")]
    #[error(transparent)]
    StringSourceError(Box<StringSourceError>),
    #[error("TODO")]
    StringSourceIsNone
}

#[cfg(feature = "string-source")]
impl From<StringSourceError> for RequestBodyError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

impl RequestBody {
    /// # Errors
    /// See [`RequestBody`]'s documentation for details.
    pub fn apply(&self, request: RequestBuilder, url: &Url, params: &Params) -> Result<RequestBuilder, RequestBodyError> {
        Ok(match self {
            #[cfg(feature = "string-source")]
            Self::Text(source) => request.body(source.get(url, params, false)?.ok_or(RequestBodyError::StringSourceIsNone)?.into_owned()),
            #[cfg(not(feature = "string-source"))]
            Self::Text(text) => request.body(text.clone()),
            #[cfg(feature = "string-source")]
            Self::Form(map) => request.form(&map.iter()
                .map(|(k, source)| source.get(url, params, false)
                    .map(|maybe_string| maybe_string
                        .map(|string| (k, string.into_owned()))
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum ResponseHandler {
    #[default]
    Body,
    #[cfg(feature = "string-source")]
    HeaderValue(StringSource),
    #[cfg(not(feature = "string-source"))]
    HeaderValue(String),
    Url,
    #[cfg(feature = "string-source")]
    Cookie(StringSource),
    #[cfg(not(feature = "string-source"))]
    Cookie(String)
}

#[derive(Debug, Error)]
pub enum ResponseHandlerError {
    #[error(transparent)]
    ReqwestError(#[from] ReqwestError),
    #[cfg(feature = "string-source")]
    #[error(transparent)]
    StringSourceError(Box<StringSourceError>),
    #[cfg(feature = "string-source")]
    #[error("TODO")]
    StringSourceIsNone,
    #[error("TODO")]
    HeaderNotFound,
    #[error(transparent)]
    ToStrError(#[from] ToStrError),
    #[error("TODO")]
    CookieNotFound
}

#[cfg(feature = "string-source")]
impl From<StringSourceError> for ResponseHandlerError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

impl ResponseHandler {
    /// # Errors
    /// TODO
    pub fn handle(&self, url: &Url, params: &Params, response: Response) -> Result<String, ResponseHandlerError> {
        Ok(match self {
            Self::Body => response.text()?,
            #[cfg(feature = "string-source")]
            Self::HeaderValue(source) => response.headers().get(&*source.get(url, params, false)?.ok_or(ResponseHandlerError::StringSourceIsNone)?).ok_or(ResponseHandlerError::HeaderNotFound)?.to_str()?.to_string(),
            #[cfg(not(feature = "string-source"))]
            Self::HeaderValue(name) => response.headers().get(name).ok_or(ResponseHandlerError::HeaderNotFound)?.to_str()?.to_string(),
            Self::Url => response.url().as_str().to_string(),
            #[cfg(feature = "string-source")]
            Self::Cookie(source) => {
                let name = source.get(url, params, false)?.ok_or(ResponseHandlerError::StringSourceIsNone)?;
                response.cookies().find(|cookie| cookie.name()==name).ok_or(ResponseHandlerError::CookieNotFound)?.value().to_string()
            }
            #[cfg(not(feature = "string-source"))]
            Self::Cookie(name) => response.cookies().find(|cookie| cookie.name()==name).ok_or(ResponseHandlerError::CookieNotFound)?.value().to_string()
        })
    }
}
