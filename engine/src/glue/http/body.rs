//! [`HttpRequestBodyConfig`].

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::prelude::*;

/// How a [`HttpRequestConfig`] should construct its body.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Suitability)]
#[serde(deny_unknown_fields)]
pub enum HttpRequestBodyConfig {
    /// Send the specified text.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`ApplyRequestBodyError::StringSourceIsNone`].
    Text(StringSource),
    /// Sends the HTML form.
    ///
    /// If a call to [`StringSource::get`] returns [`None`], the field it came from isn't sent. This can be useful for API keys.
    /// # Errors
    /// If a call to [`StringSource::get`] returns an error, that error is returned.
    Form(HashMap<String, StringSource>),
    /// Sends JSON.
    /// # Errors
    /// If the call to [`StringSourceJsonValue::make`] returns an error, that error is returned.
    Json(StringSourceJsonValue)
}

/// The enum of errors [`HttpRequestBodyConfig::apply`] can return.
#[derive(Debug, Error)]
pub enum ApplyRequestBodyError {
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(Box<StringSourceError>),
    /// Returned when a call to [`StringSource::get`] returns [`None`] where it must return [`Some`].
    #[error("A StringSource was None where it has to be Some.")]
    StringSourceIsNone
}

impl From<StringSourceError> for ApplyRequestBodyError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

impl HttpRequestBodyConfig {
    /// Inserts the specified body into a [`reqwest::blocking::RequestBuilder`].
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn apply(&self, request: reqwest::blocking::RequestBuilder, task_state: &TaskStateView) -> Result<reqwest::blocking::RequestBuilder, ApplyRequestBodyError> {
        Ok(match self {
            Self::Text(StringSource::String(value)) => request.body(value.clone()),
            Self::Text(value) => request.body(get_string!(value, task_state, ApplyRequestBodyError)),
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
