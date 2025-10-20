//! Details on how to call a [`Commons`] thing.

use std::str::FromStr;

use serde::{Serialize, Deserialize};

use crate::prelude::*;

/// Instructions on how to call a [`Commons`] thing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
#[serde(remote = "Self")]
pub struct CommonCall {
    /// The name of the [`Commons`] thing to call.
    pub name: Box<StringSource>,
    /// The [`CommonCallArgsConfig`].
    ///
    /// Defaults to [`CommonCallArgsConfig::default`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub args: Box<CommonCallArgsConfig>
}

impl FromStr for CommonCall {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl From<&str> for CommonCall {
    fn from(value: &str) -> Self {
        StringSource::from(value).into()
    }
}

impl From<String> for CommonCall {
    fn from(value: String) -> Self {
        StringSource::from(value).into()
    }
}

impl From<StringSource> for CommonCall {
    fn from(value: StringSource) -> Self {
        Box::new(value).into()
    }
}

impl From<Box<StringSource>> for CommonCall {
    fn from(value: Box<StringSource>) -> Self {
        Self {
            name: value,
            args: Default::default()
        }
    }
}
