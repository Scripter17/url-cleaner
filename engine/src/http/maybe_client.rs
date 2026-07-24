//! [`MaybeHttpClient`].

use crate::prelude::*;

/// An [`Option`] of an [`HttpClient`], allowing for blocking HTTP requests on a per-[`Job`] basis.
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct MaybeHttpClient(pub Option<HttpClient>);

impl MaybeHttpClient {
    /// Make a new [`Self`].
    pub fn new(handle: Option<tokio::runtime::Handle>) -> Self {
        Self(handle.map(HttpClient::new))
    }

    /// Get the inner [`HttpClient`].
    pub fn get(&self) -> Option<&HttpClient> {
        self.0.as_ref()
    }

    /// [`HttpClient::do`].
    /// # Errors
    /// If [`Self::get`] returns [`None`], returns the error [`NoHttpClient`].
    pub fn r#do<'j: 't, 't>(&'j self, request: &'j HttpRequestSource, response: &'j HttpResponseHandler, task_state: &'t TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Option<Cow<'t, str>>, DoHttpRequestError> {
        self.get().ok_or(NoHttpClient)?.r#do(request, response, task_state, args)
    }

    /// [`HttpClient::do_async`].
    /// # Errors
    /// If [`Self::get`] returns [`None`], returns the error [`NoHttpClient`].
    pub async fn do_async<'j: 't, 't>(&'j self, request: &'j HttpRequestSource, response: &'j HttpResponseHandler, task_state: &'t TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Option<Cow<'t, str>>, DoHttpRequestError> {
        self.get().ok_or(NoHttpClient)?.do_async(request, response, task_state, args).await
    }
}
