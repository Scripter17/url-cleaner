//! [`Task`] and co.

use std::str::FromStr;

use serde::{Serialize, Deserialize};
use url::Url;

use crate::prelude::*;

/// A task for a [`Job`] to [`Job::do`].
///
/// See [`TaskConfig`] for how to make these.
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
        Ok(match s.as_bytes().first() {
            Some(b'{' | b'"'                  ) => serde_json::from_str(s)?,
            Some(b'a' ..= b'z' | b'A' ..= b'Z') => Url::parse(s)?.into(),
            None => Err(MakeTaskError::IgnoreLineNotIgnored)?,
            _    => Err(MakeTaskError::OtherwiseInvalid)?
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
        Ok(match value.first() {
            Some(b'{' | b'"'                  ) => serde_json::from_slice(value)?,
            Some(b'a' ..= b'z' | b'A' ..= b'Z') => Url::parse(str::from_utf8(value)?)?.into(),
            None => Err(MakeTaskError::IgnoreLineNotIgnored)?,
            _    => Err(MakeTaskError::OtherwiseInvalid)?
        })
    }
}

impl TryFrom<serde_json::Value> for Task {
    type Error = serde_json::Error;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
