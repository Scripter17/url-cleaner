//! [`VarSource`].

use crate::prelude::*;

/// Get a var.
///
/// Defaults to [`Self::None`].
///
/// Null deserializes/serializes from/into [`Self::None`].
///
/// Strings deserialize/serialize from/into [`Self::Params`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(remote = "Self")]
pub enum VarSource {
    /// [`None`].
    ///
    /// The default.
    #[default]
    None,
    /** [`Params::vars`].       */ Params     (#[suitable(assert = var_source_params      )] StringSource),
    /** [`TaskContext::vars`].  */ TaskContext(#[suitable(assert = var_source_task_context)] StringSource),
    /** [`JobContext::vars`].   */ JobContext (#[suitable(assert = var_source_job_context )] StringSource),
    /** [`FunctionArgs::vars`]. */ FunctionArg(                                              StringSource),
    /** [`Secrets::vars`].      */ Secret     (#[suitable(assert = var_source_secret      )] StringSource),
}

impl VarSource {
    /// Get the var.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn get<'j: 't, 't>(&'j self, ts: &'t TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Option<Cow<'t, str>>, VarSourceError> {
        Ok(match self {
            Self::None                               => None,
            Self::Params(StringSource::String(name)) => ts.job.cleaner.params.vars.get(name).map(Into::into),
            _ => self._get(ts, args)?,
        })
    }

    /// [`Self::get`] without fast paths.
    #[cold]
    fn _get<'j: 't, 't>(&'j self, ts: &'t TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Option<Cow<'t, str>>, VarSourceError> {
        Ok(match self {
            Self::None              => None,
            Self::Params     (name) => ts.job.cleaner.params     .vars.get(&*name.get_some(ts, args)??).map(Into::into),
            Self::TaskContext(name) => ts.context                .vars.get(&*name.get_some(ts, args)??).map(Into::into),
            Self::JobContext (name) => ts.job.context            .vars.get(&*name.get_some(ts, args)??).map(Into::into),
            Self::FunctionArg(name) => args.ok_or(NotInFunction)?.vars.get(&*name.get_some(ts, args)??).map(Into::into),
            Self::Secret     (name) => ts.job.secrets            .vars.get(&*name.get_some(ts, args)??).map(Into::into),
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
        struct V;

        impl<'de> Visitor<'de> for V {
            type Value = VarSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a none, str, string, or map.")
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

        deserializer.deserialize_any(V)
    }
}
