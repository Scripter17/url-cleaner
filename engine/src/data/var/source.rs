//! [`VarSource`].

use crate::prelude::*;

/// Get a var.
///
/// Defaults to [`Self::None`].
///
/// Null deserializes/serializes from/into [`Self::None`].
///
/// Strings deserialize/serialize from/into [`Self::Params`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(remote = "Self")]
pub enum VarSource {
    /// [`None`].
    #[default]
    None,
    /// [`Params::vars`].
    Params(StringSource),
    /// [`TaskContext::vars`].
    TaskContext(StringSource),
    /// [`JobContext::vars`].
    JobContext(StringSource),
    /// [`FunctionArgs::vars`].
    FunctionArg(StringSource),
    /// [`Secrets::vars`].
    Secret(StringSource),
}

impl VarSource {
    /// Get the var.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn get<'j: 't, 't>(&'j self, task_state: &'t TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Option<Cow<'t, str>>, VarSourceError> {
        debug!(VarSource::get, self; match self {
            Self::Params(StringSource::String(x)) => Ok(task_state.job.cleaner.params.vars.get(&**x).map(Into::into)),
            x => x._get(task_state, args)
        })
    }

    /// [`Self::get`].
    fn _get<'j: 't, 't>(&'j self, task_state: &'t TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Option<Cow<'t, str>>, VarSourceError> {
        Ok(match self {
            Self::None              => None,
            Self::Params     (name) => task_state.job.cleaner.params.vars.get(get!(&name)).map(Into::into),
            Self::TaskContext(name) => task_state.context           .vars.get(get!(&name)).map(Into::into),
            Self::JobContext (name) => task_state.job.context       .vars.get(get!(&name)).map(Into::into),
            Self::FunctionArg(name) => args.ok_or(NotInFunction)?   .vars.get(get!(&name)).map(Into::into),
            Self::Secret     (name) => task_state.job.secrets       .vars.get(get!(&name)).map(Into::into),
        })
    }
}



impl FromStr for VarSource {
    type Err = std::convert::Infallible;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        Ok(name.into())
    }
}

impl From<&str        > for VarSource {fn from(name: &str        ) -> Self {Self::Params(name.into())}}
impl From<String      > for VarSource {fn from(name: String      ) -> Self {Self::Params(name.into())}}
impl From<StringSource> for VarSource {fn from(name: StringSource) -> Self {Self::Params(name       )}}



impl Suitability for VarSource {
    fn assert_suitability(&self, config: &Cleaner) {
        match self {
            Self::Params     (StringSource::String(name)) => assert!(config.docs.params      .vars.contains_key(name), "Undocumented Params Var: {name}"     ),
            Self::TaskContext(StringSource::String(name)) => assert!(config.docs.task_context.vars.contains_key(name), "Undocumented TaskContext var: {name}"),
            Self::JobContext (StringSource::String(name)) => assert!(config.docs.job_context .vars.contains_key(name), "Undocumented JobContext var: {name}" ),
            Self::Secret     (StringSource::String(name)) => assert!(config.docs.secrets     .vars.contains_key(name), "Undocumented Secret var: {name}"     ),
            _ => {}
        }
    }
}



impl Serialize for VarSource {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Ok(match self {
            Self::None                            => serializer.serialize_none()?,
            Self::Params(StringSource::String(x)) => serializer.serialize_str(x)?,
            _                                     => Self::serialize(self, serializer)?
        })
    }
}

impl<'de> Deserialize<'de> for VarSource {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(VarSourceVisitor)
    }
}

/// [`Visitor`] for [`VarSource`].
#[derive(Debug)]
struct VarSourceVisitor;

impl<'de> Visitor<'de> for VarSourceVisitor {
    type Value = VarSource;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string, null, or another variant written normally.")
    }

    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(Self::Value::None)
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(Self::Value::Params(v.into()))
    }

    fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
        Ok(Self::Value::Params(v.into()))
    }

    fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        Self::Value::deserialize(serde::de::value::MapAccessDeserializer::new(map))
    }
}
