//! [`HttpBodyConfig`].

use crate::prelude::*;

/// How a [`HttpRequestSource`] should construct its body.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Suitability)]
#[serde(deny_unknown_fields)]
pub enum HttpBodyConfig {
    /// [`HttpFormBodySource`].
    Text(HttpTextBodySource),
    /// [`HttpFormBodySource`].
    Form(HttpFormBodySource),
    /// [`HttpJsonBodySource`].
    Json(HttpJsonBodySource),
}

impl HttpBodyConfig {
    /// Inserts the specified body into a [`reqwest::blocking::RequestBuilder`].
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn apply<'j>(&'j self, request: reqwest::blocking::RequestBuilder, task_state: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<reqwest::blocking::RequestBuilder, HttpBodyConfigError> {
        Ok(match self {
            Self::Text(value) => request.body(value.get(task_state, args)?.into_owned()),
            Self::Form(form ) => request.form(&get!(?form )),
            Self::Json(json ) => request.json(&get!(?json )),
        })
    }
}
