//! [`FlagSource`].

use crate::prelude::*;

/// Gets a flag from somewhere.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(remote = "Self")]
pub enum FlagSource {
    /// Get it from [`Params::flags`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, FlagSourceError))]
    Params(StringSource),
    /// Get it from [`TaskContext::flags`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, FlagSourceError))]
    TaskContext(StringSource),
    /// Get it from [`JobContext::flags`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, FlagSourceError))]
    JobContext(StringSource),
    /// Get it from [`FunctionArgs::flags`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, FlagSourceError))]
    FunctionArg(StringSource),
    /// Get it from the contained [`bool`].
    Literal(bool)
}

impl Default for FlagSource {
    fn default() -> Self {
        Self::Literal(false)
    }
}

impl FlagSource {
    /// Get the flag.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn get<'j>(&'j self, task_state: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<bool, FlagSourceError> {
        debug!(FlagSource::get, self; match self {
            Self::Params(StringSource::String(x)) => Ok(task_state.job.cleaner.params.flags.contains(x)),
            x => x._get(task_state, args)
        })
    }

    /// [`Self::get`].
    fn _get<'j>(&'j self, task_state: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<bool, FlagSourceError> {
        Ok(match self {
            Self::Params     (name) => task_state.job.cleaner.params              .flags.contains(get!(&name)),
            Self::TaskContext(name) => task_state.context                         .flags.contains(get!(&name)),
            Self::JobContext (name) => task_state.job.context                     .flags.contains(get!(&name)),
            Self::FunctionArg    (name) => args.ok_or(FlagSourceError::NotInFunction)?.flags.contains(get!(&name)),
            Self::Literal    (x   ) => *x
        })
    }
}

impl FromStr for FlagSource {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::Params(s.into()))
    }
}

impl From<&str> for FlagSource {
    fn from(value: &str) -> Self {
        Self::Params(value.into())
    }
}

impl From<String> for FlagSource {
    fn from(value: String) -> Self {
        Self::Params(value.into())
    }
}

impl From<StringSource> for FlagSource {
    fn from(value: StringSource) -> Self {
        Self::Params(value)
    }
}

/// The enum of errors [`FlagSource::get`] can return.
#[derive(Debug, Error)]
pub enum FlagSourceError {
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] Box<StringSourceError>),
    /// [`StringNotFound`].
    #[error(transparent)]
    StringNotFound(#[from] StringNotFound),

    /// Returned when attempting to use [`FunctionArgs`] outside a function.
    #[error("Attempted to use FunctionArgs outside a function.")]
    NotInFunction
}

impl From<StringSourceError> for FlagSourceError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

impl Serialize for FlagSource {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Ok(match self {
            Self::Literal(x) => serializer.serialize_bool(*x)?,
            Self::Params(StringSource::String(x)) => serializer.serialize_str(x)?,
            _ => Self::serialize(self, serializer)?
        })
    }
}

impl<'de> Deserialize<'de> for FlagSource {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(FlagSourceVisitor)
    }
}

/// [`Visitor`] for [`FlagSource`].
#[derive(Debug)]
struct FlagSourceVisitor;

impl<'de> Visitor<'de> for FlagSourceVisitor {
    type Value = FlagSource;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a list, null, or another variant written normally.")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(Self::Value::Params(v.into()))
    }

    fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
        Ok(Self::Value::Params(v.into()))
    }

    fn visit_bool<E: de::Error>(self, x: bool) -> Result<Self::Value, E> {
        Ok(Self::Value::Literal(x))
    }

    fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        Self::Value::deserialize(serde::de::value::MapAccessDeserializer::new(map))
    }
}
