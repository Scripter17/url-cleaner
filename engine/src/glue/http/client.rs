//! [`HttpClient`].

use std::sync::OnceLock;

use url::Url;
use thiserror::Error;
use reqwest::{Proxy, header::{HeaderName, HeaderValue}};

use crate::prelude::*;

/// A thin wrapper around [`reqwest::blocking::Client`] to allow `&Default::default` to give sensible defaults.
///
/// Specifically:
///
/// 1. Header `User-Agent` set to `Firefox`.
/// 2. Header `Sec-Gpc` set to `1`.
/// 3. Header `Dnt` set to `1`.
/// 4. [`reqwest::blocking::ClientBuilder::redirect`] set to [`reqwest::redirect::Policy::none`].
/// 5. [`reqwest::blocking::ClientBuilder::referer`] set to [`false`].
///
/// [`HttpClient`] implements [`From`] for [`reqwest::blocking::Client`] to allow more advanced usage.
#[derive(Debug, Clone)]
pub struct HttpClient {
    /// The lazily made [`reqwest::blocking::Client`].
    client: OnceLock<reqwest::blocking::Client>,
    /// The [`Proxy`]s to use.
    proxies: Vec<Proxy>
}

impl HttpClient {
    /// Make a new [`HttpClient`], optionally with proxies.
    pub fn new(proxies: Vec<reqwest::Proxy>) -> Self {
        Self {
            client: OnceLock::new(),
            proxies
        }
    }

    /// Gets the compiled [`reqwest::blocking::Client`] or, if it hasn't been compiled. compiles it.
    /// # Errors
    #[doc = edoc!(callerr(reqwest::blocking::ClientBuilder::build))]
    pub fn get(&self) -> Result<&reqwest::blocking::Client, reqwest::Error> {
        debug!(HttpClient::get_response, self);
        if let Some(client) = self.client.get() {
            Ok(client)
        } else {
            let mut temp = reqwest::blocking::Client::builder().default_headers([
    		    (HeaderName::from_static("user-agent"), HeaderValue::from_static("Firefox")),
    		    (HeaderName::from_static("sec-gpc"   ), HeaderValue::from_static("1"      )),
    		    (HeaderName::from_static("dnt"       ), HeaderValue::from_static("1"      ))
            ].into_iter().collect())
                .redirect(reqwest::redirect::Policy::none())
                .referer(false);
            for proxy in self.proxies.clone() {
                temp = temp.proxy(proxy);
            }
            let ret = temp.build()?;
            Ok(self.client.get_or_init(|| ret))
        }
    }

    /// Send a [`HttpRequestConfig`] and return the response.
    /// # Errors
    #[doc = edoc!(geterr(Self), geterr(StringSource), getnone(StringSource, DoHttpRequestError), callerr(HeaderName::try_from, 3), callerr(HeaderValue::try_from, 3), callerr(HttpRequestBodyConfig::apply), callerr(reqwest::blocking::RequestBuilder::send))]
    pub fn get_response(&self, config: HttpRequestConfig, task_state: &TaskStateView) -> Result<reqwest::blocking::Response, DoHttpRequestError> {
        debug!(HttpClient::get_response, self, config, task_state.debug_helper());
        let mut req = self.get()?.request(
            get_str!(config.method, task_state, DoHttpRequestError).parse()?,
            Url::parse(get_str!(config.url, task_state, DoHttpRequestError))?,
        );
        for (name, value) in config.headers.iter() {
            if let Some(value) = value.get(task_state)? {
                req = req.header(HeaderName::try_from(name)?, HeaderValue::try_from(value.into_owned())?);
            }
        }
        if let Some(body) = &config.body {
            req = body.apply(req, task_state)?;
        }
        Ok(req.send()?)
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl From<reqwest::blocking::Client> for HttpClient {
    fn from(value: reqwest::blocking::Client) -> Self {
        Self {
            client: value.into(),
            proxies: Vec::new()
        }
    }
}

impl TryFrom<HttpClient> for reqwest::blocking::Client {
    type Error = reqwest::Error;

    fn try_from(value: HttpClient) -> Result<Self, Self::Error> {
        if let Some(x) = value.client.into_inner() {
            Ok(x)
        } else {
            let mut temp = reqwest::blocking::Client::builder().default_headers([
    		    (HeaderName::from_static("user-agent"), HeaderValue::from_static("Firefox")),
    		    (HeaderName::from_static("sec-gpc"   ), HeaderValue::from_static("1"      )),
    		    (HeaderName::from_static("dnt"       ), HeaderValue::from_static("1"      ))
            ].into_iter().collect())
                .redirect(reqwest::redirect::Policy::none())
                .referer(false);
            for proxy in value.proxies {
                temp = temp.proxy(proxy);
            }
            temp.build()
        }
    }
}

/// The enum of errors [`HttpClient::get_response`] can return.
#[derive(Debug, Error)]
pub enum DoHttpRequestError {
    /// Returned when a [`reqwest::Error`] is encountered.
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    /// Returned when a [`ApplyRequestBodyError`] is encountered.
    #[error(transparent)]
    ApplyRequestBodyError(#[from] ApplyRequestBodyError),
    /// Returned when a call to [`StringSource::get`] returns [`None`] where it has to return [`Some`].
    #[error("A StringSource was None where it had to be Some.")]
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
    /// Returned when a [`http::method::InvalidMethod`] is encountered.
    #[error(transparent)]
    InvalidMethod(#[from] http::method::InvalidMethod),
    /// Returned when a [`reqwest::header::InvalidHeaderName`] is encountered.
    #[error(transparent)]
    InvalidHeaderName(#[from] reqwest::header::InvalidHeaderName),
    /// Returned when a [`reqwest::header::InvalidHeaderValue`] is encountered.
    #[error(transparent)]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue)
}

impl From<StringSourceError> for DoHttpRequestError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(value.into())
    }
}
