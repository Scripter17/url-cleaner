

use crate::prelude::*;

/// How a [`HttpRequestSource`] should construct its body.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Suitability)]
#[serde(deny_unknown_fields)]
pub enum HttpBodyConfig {
    /// [`StringSource`].
    Text(StringSource),
    /// [`HttpFormBodySource`].
    Form(HttpFormBodySource),
    /// [`HttpJsonBodySource`].
    Json(HttpJsonBodySource),
}

/// The enum of errors [`HttpBodyConfig::apply`] can return.
#[derive(Debug, Error)]
pub enum HttpBodyConfigError {
    /// [`StringSourceError`].
    #[error(transparent)]
    StringSourceError(#[from] Box<StringSourceError>),
    /// [`StringNotFound`].
    #[error(transparent)]
    StringNotFound(#[from] StringNotFound),
    /// [`HttpFormBodySourceError`].
    #[error(transparent)]
    HttpFormBodySourceError(#[from] Box<HttpFormBodySourceError>),
    /// [`HttpJsonBodySourceError`].
    #[error(transparent)]
    HttpJsonBodySourceError(#[from] Box<HttpJsonBodySourceError>),
}

impl From<StringSourceError      > for HttpBodyConfigError {fn from(value: StringSourceError      ) -> Self {Box::new(value).into()}}
impl From<HttpFormBodySourceError> for HttpBodyConfigError {fn from(value: HttpFormBodySourceError) -> Self {Box::new(value).into()}}
impl From<HttpJsonBodySourceError> for HttpBodyConfigError {fn from(value: HttpJsonBodySourceError) -> Self {Box::new(value).into()}}

impl HttpBodyConfig {
    /// Inserts the specified body into a [`reqwest::blocking::RequestBuilder`].
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn apply<'j>(&'j self, request: reqwest::blocking::RequestBuilder, task_state: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<reqwest::blocking::RequestBuilder, HttpBodyConfigError> {
        Ok(match self {
            Self::Text(value) => request.body( get!(*value)),
            Self::Form(form ) => request.form(&get!(?form )),
            Self::Json(json ) => request.json(&get!(?json )),
        })
    }
}
