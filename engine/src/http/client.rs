

use std::sync::OnceLock;

use url::Url;
use thiserror::Error;
use reqwest::header::{HeaderName, HeaderValue};

use crate::prelude::*;

/// A [`reqwest::blocking::Client`] with a default constructor suitable for URL Cleaner.
///
/// Specifically:
///
/// 1. Header `User-Agent` set to `Firefox`.
/// 2. Header `Sec-Gpc` set to `1`.
/// 3. Header `Dnt` set to `1`.
/// 4. [`reqwest::blocking::ClientBuilder::redirect`] set to [`reqwest::redirect::Policy::none`].
/// 5. [`reqwest::blocking::ClientBuilder::referer`] set to [`false`].
#[derive(Debug, Clone, Default)]
pub struct HttpClient {
    /// The lazily made [`reqwest::blocking::Client`].
    pub client: OnceLock<reqwest::blocking::Client>,
}

impl HttpClient {
    /// Gets [`Self::client`] or, if it's uninitialized, creates the default client.
    /// # Errors
    #[doc = edoc!(callerr(reqwest::blocking::ClientBuilder::build))]
    pub fn get(&self) -> Result<&reqwest::blocking::Client, reqwest::Error> {
        if let Some(client) = self.client.get() {
            Ok(client)
        } else {
            let ret = reqwest::blocking::Client::builder().default_headers([
    		    (HeaderName::from_static("user-agent"), HeaderValue::from_static("Firefox")),
    		    (HeaderName::from_static("sec-gpc"   ), HeaderValue::from_static("1"      )),
    		    (HeaderName::from_static("dnt"       ), HeaderValue::from_static("1"      ))
            ].into_iter().collect())
                .redirect(reqwest::redirect::Policy::none())
                .referer(false).build()?;
            Ok(self.client.get_or_init(|| ret))
        }
    }
}
