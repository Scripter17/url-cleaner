//! [`FlagSource`].

use crate::prelude::*;

/// Get a flag.
///
/// Defaults to [`Self::False`].
///
/// Strings deserialize/serialize from/into [`Self::Params`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(remote = "Self")]
pub enum FlagSource {
    /// [`false`].
    ///
    /// The default.
    #[default]
    False,
    /// [`true`].
    True,
    /** [`Params::flags`].       */ Params     (#[suitable(assert = flag_source_params      )] StringSource),
    /** [`TaskContext::flags`].  */ TaskContext(#[suitable(assert = flag_source_task_context)] StringSource),
    /** [`JobContext::flags`].   */ JobContext (#[suitable(assert = flag_source_job_context )] StringSource),
    /** [`FunctionArgs::flags`]. */ FunctionArg(                                               StringSource),
}

impl FlagSource {
    /// Get the flag.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn get<'j>(&'j self, ts: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<bool, FlagSourceError> {
        Ok(match self {
            Self::False                              => false,
            Self::True                               => true,
            Self::Params(StringSource::String(name)) => ts.job.cleaner.params.flags.contains(name),
            _ => self._get(ts, args)?,
        })
    }

    /// [`Self::get`] without fast paths.
    #[cold]
    fn _get<'j>(&'j self, ts: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<bool, FlagSourceError> {
        Ok(match self {
            Self::False             => false,
            Self::True              => true,
            Self::Params     (name) => ts.job.cleaner.params     .flags.contains(&*name.get_some(ts, args)??),
            Self::TaskContext(name) => ts.context                .flags.contains(&*name.get_some(ts, args)??),
            Self::JobContext (name) => ts.job.context            .flags.contains(&*name.get_some(ts, args)??),
            Self::FunctionArg(name) => args.ok_or(NotInFunction)?.flags.contains(&*name.get_some(ts, args)??),
        })
    }
}



impl FromStr for FlagSource {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::Params(s.into()))
    }
}

impl From<&str        > for FlagSource {fn from(value: &str        ) -> Self {Self::Params(value.into())}}
impl From<String      > for FlagSource {fn from(value: String      ) -> Self {Self::Params(value.into())}}
impl From<StringSource> for FlagSource {fn from(value: StringSource) -> Self {Self::Params(value)}}



impl Serialize for FlagSource {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Ok(match self {
            Self::False                           => serializer.serialize_bool(false)?,
            Self::True                            => serializer.serialize_bool(true )?,
            Self::Params(StringSource::String(x)) => serializer.serialize_str(x)?,
            _                                     => Self::serialize(self, serializer)?
        })
    }
}

impl<'de> Deserialize<'de> for FlagSource {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct V;

        impl<'de> Visitor<'de> for V {
            type Value = FlagSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a str, string, bool, or map.")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(Self::Value::Params(v.into()))
            }

            fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(Self::Value::Params(v.into()))
            }

            fn visit_bool<E: de::Error>(self, x: bool) -> Result<Self::Value, E> {
                Ok(match x {
                    false => Self::Value::False,
                    true  => Self::Value::True ,
                })
            }

            fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
                Self::Value::deserialize(serde::de::value::MapAccessDeserializer::new(map))
            }
        }

        deserializer.deserialize_any(V)
    }
}
