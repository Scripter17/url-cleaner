//! Unified logic for calling commons.

use std::str::FromStr;
use std::collections::{HashSet, HashMap};
use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::types::*;
use crate::util::*;
use crate::glue::*;

/// The name of the common to call and the arguments to call it with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(remote = "Self")]
pub struct CommonCall {
    /// The name of the common to call.
    pub name: Box<StringSource>,
    /// The arguments to call it with.
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

/// The rules used to make a [`CommonCallArgs`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub struct CommonCallArgsSource {
    /// The flags for a common call.
    pub flags: HashSet<String>,
    /// The vars for a common call.
    pub vars: HashMap<String, StringSource>,
    /// The [`HttpClientConfigDiff`] to use for the duration of a common call.
    #[cfg(feature = "http")]
    pub http_client_config_diff: Option<HttpClientConfigDiff>
}

/// The enum of errors that [`CommonCallArgsSource::make`] can return.
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
    /// Makes a [`CommonCallArgs`].
    /// # Errors
    /// If a call to [`StringSource::get`] returns an error, that error is returned.
    pub fn make<'a>(&'a self, job_state: &JobStateView) -> Result<CommonCallArgs<'a>, CommonCallArgsError> {
        Ok(CommonCallArgs {
            flags: self.flags.iter().map(|x| Cow::Borrowed(&**x)).collect(),
            vars: self.vars.iter().map(|(k, v)| Ok((Cow::Borrowed(&**k), get_string!(v, job_state, StringSourceError)))).collect::<Result<HashMap<_, _>, StringSourceError>>()?,
            #[cfg(feature = "http")]
            http_client_config_diff: self.http_client_config_diff.as_ref().map(Cow::Borrowed)
        })
    }
}

/// The arguments for a common.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub struct CommonCallArgs<'a> {
    /// The flags for a common call.
    pub flags: HashSet<Cow<'a, str>>,
    /// The vars for a common call.
    pub vars: HashMap<Cow<'a, str>, String>,
    /// The [`HttpClientConfigDiff`] to use for the duration of a common call.
    #[cfg(feature = "http")]
    pub http_client_config_diff: Option<Cow<'a, HttpClientConfigDiff>>
}
