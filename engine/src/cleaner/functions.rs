//! [`Functions`]

use std::collections::HashMap;
use std::str::FromStr;

use serde::{Serialize, Deserialize};

use crate::prelude::*;

/// Common snippets used throughout a [`Cleaner::actions`].
///
/// For example, an [`Action`] for removing universal tracking parameters before both expanding redirects and returning the final URL.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct Functions {
    /// [`Condition`]s.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub conditions: HashMap<String, Condition>,
    /// [`Action`]s.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub actions: HashMap<String, Action>,
    /// [`StringSource`]s.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_sources: HashMap<String, StringSource>,
    /// [`StringModification`]s.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_modifications: HashMap<String, StringModification>,
    /// [`StringMatcher`]s.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_matchers: HashMap<String, StringMatcher>
}

/// Instructions on how to call a [`Functions`] thing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
#[serde(remote = "Self")]
pub struct FunctionCall {
    /// The name of the [`Functions`] thing to call.
    pub name: String,
    /// The [`CallArgs`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub args: CallArgs
}

string_or_struct_magic!(FunctionCall);

impl FromStr for FunctionCall {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl From<&str> for FunctionCall {
    fn from(name: &str) -> Self {
        name.to_string().into()
    }
}

impl From<String> for FunctionCall {
    fn from(name: String) -> Self {
        Self {
            name,
            args: Default::default()
        }
    }
}
