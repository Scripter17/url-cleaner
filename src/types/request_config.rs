use std::str::FromStr;
use std::collections::HashMap;

use url::{Url, ParseError};
use serde::{Deserialize, Serialize, de::{Deserializer, Error as _}, ser::Serializer};
use serde_json::value::Value;
use reqwest::{Method, header::HeaderMap, blocking::RequestBuilder, Error as ReqwestError};
use thiserror::Error;

use crate::types::*;
use crate::glue::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RequestConfig {
    #[serde(default = "whole_url")]
    pub url: StringSource,
    #[serde(deserialize_with = "deserialize_method", serialize_with = "serialize_method")]
    pub method: Method,
    #[serde(with = "headermap")]
    pub headers: HeaderMap,
    pub body: Option<RequestBody>
}

fn whole_url() -> StringSource {StringSource::Part(UrlPart::Whole)}

#[derive(Debug, Error)]
pub enum RequestConfigError {
    #[error(transparent)]
    ReqwestError(#[from] ReqwestError),
    #[error(transparent)]
    RequestBodyError(#[from] RequestBodyError),
    #[error("")]
    StringSourceIsNone,
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
    #[error(transparent)]
    UrlParseError(#[from] ParseError)
}

fn deserialize_method<'de, D: Deserializer<'de>>(d: D) -> Result<Method, D::Error> {
    Method::from_str(Deserialize::deserialize(d)?).map_err(|e| D::Error::custom(e))
}

fn serialize_method<S: Serializer>(method: &Method, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(method.as_str())
}

impl RequestConfig {
    /// # Errors
    /// If the call to [`Params::http_client`] returns an error, that error is returned.
    /// If the call to [`RequestBody::apply`] returns an error, that error is returned.
    pub fn make(&self, url: &Url, params: &Params) -> Result<RequestBuilder, RequestConfigError> {
        let mut ret=params.http_client()?
            .request(self.method.clone(), Url::parse(&self.url.get(url, params, false)?.ok_or(RequestConfigError::StringSourceIsNone)?)?)
            .headers(self.headers.clone());
        if let Some(body) = &self.body {ret=body.apply(ret, url, params)?;}
        Ok(ret)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum RequestBody {
    /// # Errors
    /// TODO
    Text(StringSource),
    /// # Errors
    /// TODO
    Form(HashMap<String, StringSource>),
    Json(Value)
}

#[derive(Debug, Error)]
pub enum RequestBodyError {
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
    #[error("TODO")]
    StringSourceIsNone
}
            // #[cfg(all(feature = "http", not(target_family = "wasm")))]
            // Self::BypassVip => {
            //     // requests.post("https://api.bypass.vip/", data="url=https://t.co/3XdBbanQpQ", headers={"Origin": "https://bypass.vip", "Content-Type": "application/x-www-form-urlencoded"}).json()["destination"]g
            //     if let Some(cached_result) = params.get_url_from_cache(url)? {
            //         *url = cached_result;
            //         return Ok(())
            //     }
            //     let new_url=Url::parse(params.http_client()?.post("https://api.bypass.vip")
            //         .form(&HashMap::<&str, &str>::from_iter([("url", url.as_str())]))
            //         .headers(HeaderMap::from_iter([(HeaderName::from_static("origin"), HeaderValue::from_static("https://bypass.vip"))]))
            //         .send()?
            //         .json::<serde_json::value::Value>()?
            //         .as_object().ok_or(MapperError::UnexpectedBypassVipResponse)?
            //         .get("destination").ok_or(MapperError::UnexpectedBypassVipResponse)?
            //         .as_str().ok_or(MapperError::UnexpectedBypassVipResponse)?)?;
            //     params.write_url_map_to_cache(url, &new_url)?;
            //     *url=new_url;
            // },

impl RequestBody {
    /// # Errors
    /// See [`RequestBody`]'s documentation for details.
    pub fn apply(&self, request: RequestBuilder, url: &Url, params: &Params) -> Result<RequestBuilder, RequestBodyError> {
        Ok(match self {
            Self::Text(source) => request.body(source.get(url, params, false)?.ok_or(RequestBodyError::StringSourceIsNone)?.into_owned()),
            Self::Form(map) => request.form(&map.iter().map(|(k, source)| source.get(url, params, false).map(|maybe_string| maybe_string.map(|string| (k, string.into_owned())))).collect::<Result<Option<HashMap<_, _>>, _>>()?.ok_or(RequestBodyError::StringSourceIsNone)?),
            Self::Json(json) => request.json(json)
        })
    }
}
