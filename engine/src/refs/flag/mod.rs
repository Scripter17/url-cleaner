//! [`FlagRef`] and co.

use std::str::FromStr;

use serde::{Serialize, Deserialize};

use crate::prelude::*;

pub mod r#type;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::r#type::*;

    pub use super::FlagRef;
}

/// A "reference" to a flag.
///
/// Used mainly to allow deserializing `{"type": "Params", "name": "..."}` as `"..."`.
/// # Examples
/// ```
/// use url_cleaner_engine::prelude::*;
/// assert_eq!(serde_json::from_str::<FlagRef>("\"name\"").unwrap(), FlagRef {r#type: Default::default(), name: "name".into()});
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(remote = "Self")]
pub struct FlagRef {
    /// The type of the flag to get.
    ///
    /// Defaults to [`FlagType::Params`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub r#type: FlagType,
    /// The name of the flag to get.
    pub name: StringSource
}

impl Suitability for FlagRef {
    fn assert_suitability(&self, config: &Cleaner) {
        match (&self.r#type, &self.name) {
            (FlagType::Params, StringSource::String(name)) => assert!(config.docs.flags.contains_key(name), "Undocumented Flag: {name}"),
            (FlagType::CommonArg | FlagType::Scratchpad, StringSource::String(_)) => {},
            _ => panic!("Unsuitable FlagRef: {self:?}")
        }
    }
}

impl FlagRef {
    /// Get the flag.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`GetFlagError::StringSourceIsNone`].
    ///
    /// If the call to [`FlagType::get`] returns an error, that error is returned.
    pub fn get(&self, task_state: &TaskStateView) -> Result<bool, GetFlagError> {
        debug!(FlagRef::get, self);
        self.r#type.get(get_str!(self.name, task_state, GetFlagError), task_state)
    }
}

impl FromStr for FlagRef {
    type Err = std::convert::Infallible;

    fn from_str(name: &str) -> Result<FlagRef, Self::Err> {
        Ok(name.into())
    }
}

impl From<StringSource> for FlagRef {
    fn from(name: StringSource) -> Self {
        Self {
            r#type: Default::default(),
            name
        }
    }
}

impl From<String> for FlagRef {
    fn from(name: String) -> Self {
        StringSource::String(name).into()
    }
}

impl From<&str> for FlagRef {
    fn from(name: &str) -> Self {
        name.to_string().into()
    }
}

string_or_struct_magic!(FlagRef);

