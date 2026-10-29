//! [`HttpRequestSource`].

use reqwest::header::{HeaderName, HeaderValue};

use crate::prelude::*;

/// Rules for making a [`reqwest::Request`].
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
    pub dynamic_headers: FxHashMap<String, StringSource>,
    /// The body to send.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub body: StringSource,
}

impl Default for HttpRequestSource {
    fn default() -> Self {
        Self {
            url            : get_default_url(),
            method         : get_default_method(),
            const_headers  : Default::default(),
            dynamic_headers: Default::default(),
            body           : Default::default(),
        }
    }
}

/** Serde helper. */ fn get_default_url   (                    ) -> StringSource {UrlPart::Whole.into()         }
/** Serde helper. */ fn get_default_method(                    ) -> StringSource {"GET".into()                  }
/** Serde helper. */ fn is_default_url    (value: &StringSource) -> bool         {value == &get_default_url()   }
/** Serde helper. */ fn is_default_method (value: &StringSource) -> bool         {value == &get_default_method()}

impl HttpRequestSource {
    /// Get a [`reqwest::RequestBuilder`].
    /// # Errors
    /// If [`reqwest::Method::from_str`] returns an error, that error is returned.
    ///
    /// If [`url::Url::parse`] returns an error, that error is returned.
    ///
    /// If any call to [`HeaderName::from_str`] returns an error, that error is returned.
    ///
    /// If any call to [`HeaderValue::from_str`] returns an error, that error is returned.
    ///
    /// If any call to [`StringSource::get`] returns an error, that error is returned.
    pub fn get(&self, ts: &TaskState<'_>, args: Option<&FunctionArgs>) -> Result<reqwest::Request, HttpRequestSourceError> {
        let mut ret = reqwest::Request::new(self.method.get_some(ts, args)??.parse()?, self.url.get_some(ts, args)??.parse()?);

        if let Some(map) = self.const_headers.get(ts, args)? {
            for (key, value) in map.map.iter() {
                ret.headers_mut().append(HeaderName::from_str(key)?, HeaderValue::from_str(value)?);
            }
        }

        for (key, value) in self.dynamic_headers.iter() {
            if let Some(value) = value.get(ts, args)? {
                ret.headers_mut().append(HeaderName::from_str(key)?, HeaderValue::from_str(&value)?);
            }
        }

        *ret.body_mut() = self.body.get(ts, args)?.map(|x| x.into_owned().into());

        Ok(ret)
    }
}
