//! [`Task`] and co.

use std::str::FromStr;

use serde::{Serialize, Deserialize};
use url::Url;

use crate::prelude::*;

/// A task for a [`Job`] to [`Job::do`].
///
/// String and byte types are first attempted to be parsed as JSON.
///
/// That is, if a string given to [`TaskConfig::make_task`] begins with a `{` or `"`, it is parsed as JSON.
///
/// Otherwise, they are parsed as plain URLs.
///
/// [`FromStr`], [`Deserialize`], and the various [`TryFrom`] implementations all agree on this.
/// # Tests
/// ```
/// use url_cleaner_engine::prelude::*;
///
///         r#"https://example.com"#   .make_task().unwrap();
///        r#""https://example.com""#  .make_task().unwrap();
/// r#"{"url":"https://example.com"}"# .make_task().unwrap();
///
///         br#"https://example.com"#  .make_task().unwrap();
///        br#""https://example.com""# .make_task().unwrap();
/// br#"{"url":"https://example.com"}"#.make_task().unwrap();
///
/// serde_json::json!{        r#"https://example.com"#  }.make_task().unwrap();
/// serde_json::json!{       r#""https://example.com""# }.make_task().unwrap();
/// serde_json::json!{r#"{"url":"https://example.com"}"#}.make_task().unwrap();
/// serde_json::json!{   {"url":"https://example.com"}  }.make_task().unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(remote = "Self")]
pub struct Task {
    /// The [`BetterUrl`] to modify.
    pub url: BetterUrl,
    /// The context.
    ///
    /// Defaults to [`TaskContext::default`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub context: TaskContext
}

string_or_struct_magic!(Task);

impl From<Url> for Task {
    fn from(url: Url) -> Self {
        BetterUrl::from(url).into()
    }
}

impl From<BetterUrl> for Task {
    fn from(url: BetterUrl) -> Self {
        Self {
            url,
            context: Default::default()
        }
    }
}

impl FromStr for Task {
    type Err = MakeTaskError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.starts_with(['{', '"']) {
            serde_json::from_str(s)?
        } else {
            Url::parse(s)?.into()
        })
    }
}

impl TryFrom<&str> for Task {
    type Error = <Self as FromStr>::Err;

    /// [`Self::from_str`].
    /// # Errors
    #[doc = edoc!(callerr(Self::from_str))]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<&[u8]> for Task {
    type Error = MakeTaskError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(if value.starts_with(b"{") || value.starts_with(b"\"") {
            serde_json::from_slice(value)?
        } else {
            Url::parse(str::from_utf8(value)?)?.into()
        })
    }
}

impl TryFrom<serde_json::Value> for Task {
    type Error = serde_json::Error;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
