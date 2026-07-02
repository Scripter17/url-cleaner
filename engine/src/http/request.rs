//! [`HttpRequestSource`].

use crate::prelude::*;

/// Rules for making an HTTP request.
///
/// Currently only capable of making blocking requests.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct HttpRequestSource {
    /// The URL to send the request to.
    ///
    /// Defaults to [`UrlPart::Whole`].
    #[serde(default = "get_default_url", skip_serializing_if = "is_default_url")]
    pub url: StringSource,
    /// The method to use.
    ///
    /// Defaults to `GET`.
    #[serde(default = "get_default_method", skip_serializing_if = "is_default_method")]
    pub method: StringSource,
    /// The headers to send that never change.
    ///
    /// [`Map::if_none`] is ignored.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub const_headers: MapSource,
    /// The headers to send that may change.
    ///
    /// Headers set to [`None`] are not sent.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub dynamic_headers: HashMap<String, StringSource>,
    /// The body to send.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub body: Option<HttpBodyConfig>
}

impl Default for HttpRequestSource {
    fn default() -> Self {
        Self {
            url            : get_default_url(),
            method         : get_default_method(),
            const_headers  : Default::default(),
            dynamic_headers: Default::default(),
            body           : None
        }
    }
}

/** Serde helper. **/ fn get_default_url   (                    ) -> StringSource {UrlPart::Whole.into()         }
/** Serde helper. **/ fn get_default_method(                    ) -> StringSource {"GET".into()                  }
/** Serde helper. **/ fn is_default_url    (value: &StringSource) -> bool         {value == &get_default_url()   }
/** Serde helper. **/ fn is_default_method (value: &StringSource) -> bool         {value == &get_default_method()}

impl HttpRequestSource {
    /// Get a [`reqwest::blocking::RequestBuilder`].
    /// # Errors
    /// If the call to [`HttpClient::get`] returns an error, that error is returned.
    ///
    /// If any call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`MapSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`HttpBodyConfig::apply`] returns an error, that error is returned.
    pub fn get(&self, task_state: &TaskState<'_>, args: Option<&FunctionArgs>) -> Result<reqwest::blocking::RequestBuilder, HttpRequestSourceError> {
        debug!(HttpRequestSource::get, self; self._get(task_state, args))
    }

    /// [`Self::get`].
    fn _get(&self, task_state: &TaskState<'_>, args: Option<&FunctionArgs>) -> Result<reqwest::blocking::RequestBuilder, HttpRequestSourceError> {
        let mut ret = task_state.job.http_client.get()?.request(get!(self.method).parse()?, get!(&self.url));

        if let Some(map) = get!(?self.const_headers) {
            for (key, value) in map.map.iter() {
                ret = ret.header(key, value);
            }
        }

        for (key, value) in self.dynamic_headers.iter() {
            if let Some(value) = get!(?&value) {
                ret = ret.header(key, value);
            }
        }

        if let Some(body) = &self.body {
            ret = body.apply(ret, task_state, args)?;
        }

        Ok(ret)
    }
}
