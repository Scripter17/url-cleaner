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
    /// Defaults to [`StringSource::Part`]`(`[`UrlPart::Whole`]`)`.
    #[serde(default = "get_string_source_part_whole", skip_serializing_if = "is_string_source_part_whole")]
    pub url: StringSource,
    /// The method to use.
    ///
    /// Defaults to `"GET"`.
    #[serde(default = "get_string_source_get", skip_serializing_if = "is_string_source_get")]
    pub method: StringSource,
    /// The headers to send that never change.
    ///
    /// If [`None`], does nothing.
    ///
    /// [`Map::if_none`] and [`Map::else`] are ignored.
    ///
    /// Defaults to [`MapSource::None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub const_headers: MapSource,
    /// The headers to send that may change.
    ///
    /// If a call to [`StringSource::get`] returns [`None`], the header it came from isn't sent.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub dynamic_headers: HashMap<String, StringSource>,
    /// The body to send.
    ///
    /// Defaults to [`None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub body: Option<HttpBodyConfig>
}

impl Default for HttpRequestSource {
    fn default() -> Self {
        Self {
            url            : UrlPart::Whole.into(),
            method         : "GET".into(),
            const_headers  : Default::default(),
            dynamic_headers: Default::default(),
            body           : None
        }
    }
}

/// The enum of errors [`HttpRequestSource::get`] can return.
#[derive(Debug, Error)]
pub enum HttpRequestSourceError {
    /// [`reqwest::Error`].
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    /// [`StringSourceError`].
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
    /// [`StringNotFound`].
    #[error(transparent)]
    StringNotFound(#[from] StringNotFound),
    /// [`MapSourceError`].
    #[error(transparent)]
    MapSourceError(#[from] MapSourceError),
    /// [`http::method::InvalidMethod`].
    #[error(transparent)]
    HttpInvalidMethod(#[from] http::method::InvalidMethod),
    /// [`HttpBodyConfigError`].
    #[error(transparent)]
    HttpBodyConfigError(#[from] HttpBodyConfigError),
}

impl HttpRequestSource {
    /// Get a [`reqwest::blocking::RequestBuilder`].
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

/// Serde helper function for [`HttpRequestSource::url`].
fn get_string_source_part_whole() -> StringSource {StringSource::Part(UrlPart::Whole)}
/// Serde helper function for [`HttpRequestSource::url`].
fn is_string_source_part_whole(value: &StringSource) -> bool {value == &get_string_source_part_whole()}

/// Serde helper function for [`HttpRequestSource::method`].
fn get_string_source_get() -> StringSource {"GET".into()}
/// Serde helper function for [`HttpRequestSource::method`].
fn is_string_source_get(value: &StringSource) -> bool {value == &get_string_source_get()}
