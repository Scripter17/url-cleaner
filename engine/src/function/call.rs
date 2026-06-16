//! [`FunctionCall`]

use crate::prelude::*;

/// Instructions on how to call a [`Functions`] thing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
#[serde(remote = "Self")]
pub struct FunctionCall {
    /// The name of the [`Functions`] thing to call.
    pub name: String,
    /// The [`FunctionArgs`].
    ///
    /// Defaults to [`FunctionArgs::default`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub args: FunctionArgs
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

