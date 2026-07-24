//! [`HttpClient`].

use std::sync::OnceLock;

use reqwest::header::{HeaderName, HeaderValue};

use crate::prelude::*;

/// A [`reqwest::Client`] with a default constructor suitable for URL Cleaner.
///
/// PLEASE note that this uses [`tokio::runtime::Handle::block_on`] and thus can only be used in multithreaded runtimes.
///
/// Specifically:
///
/// 1. Header `User-Agent` set to `Firefox`.
/// 2. Header `Sec-Gpc` set to `1`.
/// 3. Header `Dnt` set to `1`.
/// 4. [`reqwest::ClientBuilder::redirect`] set to [`reqwest::redirect::Policy::none`].
/// 5. [`reqwest::ClientBuilder::referer`] set to [`false`].
#[derive(Debug, Clone)]
pub struct HttpClient {
    /// The lazily made [`reqwest::Client`].
    pub client: OnceLock<reqwest::Client>,
    /// The [`tokio::runtime::Handle`] to use.
    pub handle: tokio::runtime::Handle,
}

impl HttpClient {
    /// Make a new [`Self`] with the default config.
    pub fn new(handle: tokio::runtime::Handle) -> Self {
        Self {
            client: OnceLock::new(),
            handle
        }
    }

    /// [`Self::do_async`] + [`tokio::runtime::Handle::block_on`].
    /// # Errors
    /// If the call to [`Self::do_async`] returns an error, that error is returned.
    pub fn r#do<'j: 't, 't>(&'j self, request: &'j HttpRequestSource, response: &'j HttpResponseHandler, task_state: &'t TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Option<Cow<'t, str>>, DoHttpRequestError> {
        self.handle.block_on(self.do_async(request, response, task_state, args))
    }

    /// [`HttpRequestSource::get`] + [`HttpResponseHandler::handle`].
    /// # Errors
    /// If the call to [`HttpRequestSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`reqwest::RequestBuilder::send`] returns an error, that error is returned.
    ///
    /// If the call to [`HttpResponseHandler::handle`] returns an error, that error is returned.
    pub async fn do_async<'j: 't, 't>(&'j self, request: &'j HttpRequestSource, response: &'j HttpResponseHandler, task_state: &'t TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Option<Cow<'t, str>>, DoHttpRequestError> {
        Ok(response.handle(task_state, args, &mut request.get(self, task_state, args)?.send().await?).await?)
    }

    /// Gets [`Self::client`] or, if it's uninitialized, creates the default client.
    /// # Errors
    /// If the call to [`reqwest::ClientBuilder::build`] returns an error, that error is returned.
    pub fn get_inner(&self) -> Result<&reqwest::Client, reqwest::Error> {
        if let Some(client) = self.client.get() {
            Ok(client)
        } else {
            let ret = reqwest::Client::builder().default_headers([
    		    (HeaderName::from_static("user-agent"), HeaderValue::from_static("Firefox")),
    		    (HeaderName::from_static("sec-gpc"   ), HeaderValue::from_static("1"      )),
    		    (HeaderName::from_static("dnt"       ), HeaderValue::from_static("1"      ))
            ].into_iter().collect())
                .redirect(reqwest::redirect::Policy::none()).referer(false)
                .build()?;
            Ok(self.client.get_or_init(|| ret))
        }
    }
}
