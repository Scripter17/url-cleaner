//! [`CommonCallConfig`].

use std::str::FromStr;

use serde::{Serialize, Deserialize};

use crate::prelude::*;

/// Instructions on how to call a [`Commons`] thing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
#[serde(remote = "Self")]
pub struct CommonCallConfig {
    /// The name of the [`Commons`] thing to call.
    pub name: Box<StringSource>,
    /// The [`CommonArgsConfig`].
    ///
    /// Defaults to [`CommonArgsConfig::default`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub args: Box<CommonArgsConfig>
}

string_or_struct_magic!(CommonCallConfig);

impl FromStr for CommonCallConfig {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl From<&str> for CommonCallConfig {
    fn from(value: &str) -> Self {
        StringSource::from(value).into()
    }
}

impl From<String> for CommonCallConfig {
    fn from(value: String) -> Self {
        StringSource::from(value).into()
    }
}

impl From<StringSource> for CommonCallConfig {
    fn from(value: StringSource) -> Self {
        Box::new(value).into()
    }
}

impl From<Box<StringSource>> for CommonCallConfig {
    fn from(value: Box<StringSource>) -> Self {
        Self {
            name: value,
            args: Default::default()
        }
    }
}
