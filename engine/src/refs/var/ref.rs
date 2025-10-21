//! [`VarRef`].

use std::borrow::Cow;
use std::str::FromStr;

use serde::{Serialize, Deserialize};

use crate::prelude::*;

/// A "reference" to a variable.
///
/// Used mainly to allow deserializing `{"type": "Params", "name": "..."}` as `"..."`.
/// # Examples
/// ```
/// use url_cleaner_engine::prelude::*;
/// assert_eq!(serde_json::from_str::<VarRef>("\"name\"").unwrap(), VarRef {r#type: Default::default(), name: "name".into()});
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(remote = "Self")]
pub struct VarRef {
    /// The type of the variable to get.
    ///
    /// Defaults to [`VarType::Params`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub r#type: VarType,
    /// The name of the variable to get.
    pub name: StringSource
}

impl VarRef {
    /// Get the var.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`GetVarError::StringSourceIsNone`].
    ///
    /// If the call to [`VarType::get`] returns an error, that error is returned.
    pub fn get<'a>(&self, task_state: &TaskStateView<'a>) -> Result<Option<Cow<'a, str>>, GetVarError> {
        debug!(VarRef::get, self);
        self.r#type.get(get_str!(self.name, task_state, GetVarError), task_state)
    }
}

impl FromStr for VarRef {
    type Err = std::convert::Infallible;

    fn from_str(name: &str) -> Result<VarRef, Self::Err> {
        Ok(name.into())
    }
}

impl From<StringSource> for VarRef {
    fn from(name: StringSource) -> Self {
        Self {
            r#type: Default::default(),
            name
        }
    }
}

impl From<String> for VarRef {
    fn from(name: String) -> Self {
        StringSource::String(name).into()
    }
}

impl From<&str> for VarRef {
    fn from(name: &str) -> Self {
        name.to_string().into()
    }
}

string_or_struct_magic!(VarRef);

impl Suitability for VarRef {
    fn assert_suitability(&self, config: &Cleaner) {
        match (&self.r#type, &self.name) {
            (VarType::Params     , StringSource::String(name)) => assert!(config.docs.vars             .contains_key(name), "Undocumented Var: {name}"),
            (VarType::JobContext , StringSource::String(name)) => assert!(config.docs.job_context.vars .contains_key(name), "Undocumented JobContext var: {name}"),
            (VarType::TaskContext, StringSource::String(name)) => assert!(config.docs.task_context.vars.contains_key(name), "Undocumented TaskContext var: {name}"),
            (VarType::Env        , StringSource::String(name)) => assert!(config.docs.environment_vars .contains_key(name), "Undocumented Env var: {name}"),
            (VarType::CommonArg | VarType::Scratchpad, StringSource::String(_)) => {},
            _ => panic!("Unsuitable VarRef: {self:?}")
        }
    }
}
