//! Details on how to call a [`Commons`] thing.

use std::str::FromStr;
use std::collections::{HashSet, HashMap};
use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::types::*;
use crate::util::*;
use crate::glue::*;

/// Instructions on how to call a [`Commons`] thing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(remote = "Self")]
pub struct CommonCall {
    /// The name of the [`Commons`] thing to call.
    pub name: Box<StringSource>,
    /// The args to call the [`Commons`] thing with.
    ///
    /// Defaults to [`CommonCallArgsSource::default`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub args: CommonCallArgsSource
}

impl FromStr for CommonCall {
    type Err = <StringSource as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            name: Box::new(FromStr::from_str(s)?),
            args: Default::default()
        })
    }
}

string_or_struct_magic!(CommonCall);

/// Instructions on how to make the args to call a [`Commons`] thing.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub struct CommonCallArgsSource {
    /// The flags to set.
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: HashSet<String>,
    /// The vars to set.
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, StringSource>,
    /// The [`HttpClientConfigDiff`] to apply.
    ///
    /// Yes this is a questionable design choice. It's just the least questionable of the choices I knew I could make.
    #[cfg(feature = "http")]
    #[serde(default, skip_serializing_if = "is_default")]
    pub http_client_config_diff: Option<HttpClientConfigDiff>
}

/// The enum of errors [`CommonCallArgsSource::build`] can return.
#[derive(Debug, Error)]
pub enum CommonCallArgsError {
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] Box<StringSourceError>)
}

impl From<StringSourceError> for CommonCallArgsError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

impl CommonCallArgsSource {
    /// Builds the [`CommonCallArgs`].
    /// # Errors
    /// If a call to [`StringSource::get`] returns an error, that error is returned.
    pub fn build<'a>(&'a self, job_state: &JobStateView) -> Result<CommonCallArgs<'a>, CommonCallArgsError> {
        Ok(CommonCallArgs {
            flags: self.flags.iter().map(|x| Cow::Borrowed(&**x)).collect(),
            vars: self.vars.iter().map(|(k, v)| Ok((Cow::Borrowed(&**k), get_string!(v, job_state, StringSourceError)))).collect::<Result<HashMap<_, _>, StringSourceError>>()?,
            #[cfg(feature = "http")]
            http_client_config_diff: self.http_client_config_diff.as_ref().map(Cow::Borrowed)
        })
    }
}

/// The args a [`Commons`] thing is called with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub struct CommonCallArgs<'a> {
    /// The flags that are set.
    pub flags: HashSet<Cow<'a, str>>,
    /// The vars that are set.
    pub vars: HashMap<Cow<'a, str>, String>,
    /// The [`HttpClientConfigDiff`] to apply.
    #[cfg(feature = "http")]
    pub http_client_config_diff: Option<Cow<'a, HttpClientConfigDiff>>
}
