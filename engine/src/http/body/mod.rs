//! [`HttpBodyConfig`] and co.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::prelude::*;

pub mod json;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::json::*;

    pub use super::{HttpBodyConfig, ApplyHttpBodyError};
}
use prelude::*;

/// How a [`HttpRequestConfig`] should construct its body.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Suitability)]
#[serde(deny_unknown_fields)]
pub enum HttpBodyConfig {
    /// Send the specified text.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`ApplyHttpBodyError::StringSourceIsNone`].
    Text(StringSource),
    /// Sends the HTML form.
    ///
    /// If a call to [`StringSource::get`] returns [`None`], the field it came from isn't sent. This can be useful for API keys.
    /// # Errors
    /// If a call to [`StringSource::get`] returns an error, that error is returned.
    Form(HashMap<String, StringSource>),
    /// Sends JSON.
    /// # Errors
    /// If the call to [`JsonSource::make`] returns an error, that error is returned.
    Json(JsonSource)
}

/// The enum of errors [`HttpBodyConfig::apply`] can return.
#[derive(Debug, Error)]
pub enum ApplyHttpBodyError {
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(Box<StringSourceError>),
    /// Returned when a call to [`StringSource::get`] returns [`None`] where it must return [`Some`].
    #[error("A StringSource was None where it has to be Some.")]
    StringSourceIsNone
}

impl From<StringSourceError> for ApplyHttpBodyError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

impl HttpBodyConfig {
    /// Inserts the specified body into a [`reqwest::blocking::RequestBuilder`].
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn apply<'j>(&'j self, request: reqwest::blocking::RequestBuilder, task_state: &TaskState<'j>) -> Result<reqwest::blocking::RequestBuilder, ApplyHttpBodyError> {
        Ok(match self {
            Self::Text(StringSource::String(value)) => request.body(value.clone()),
            Self::Text(value) => request.body(get_string!(value, task_state, ApplyHttpBodyError)),
            Self::Form(map) => {
                let mut ret = HashMap::new();
                for (k, v) in map.iter() {
                    if let Some(v) = v.get(task_state)? {
                        ret.insert(k, v);
                    }
                }
                request.form(&ret)
            },
            Self::Json(json) => request.json(&json.make(task_state)?)
        })
    }
}
