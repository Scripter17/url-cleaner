//! [`ListSource`].

use serde::{Serialize, ser::{Serializer, SerializeSeq}, Deserialize, de::{self, Deserializer, Visitor, SeqAccess, MapAccess}};
use thiserror::Error;

use crate::prelude::*;

/// Get a list.
///
/// Defauls to [`Self::None`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(remote = "Self")]
pub enum ListSource {
    /// [`None`].
    ///
    /// Serializes to and deserializes from `null`.
    ///
    /// The default.
    #[default]
    None,
    /// If [`Self::If::if`] is satisfied, [`Self::If::then`]. Otherwise [`Self::If::else`].
    /// # Errors
    #[doc = edoc!(checkerr(Condition), geterr(Self))]
    If {
        /// The [`Condition`] to decide between [`Self::If::then`] and [`Self::If::else`].
        r#if: Box<Condition>,
        /// The [`Self`] to use if [`Self::If::if`] is satisfied.
        r#then: Box<Self>,
        /// The [`Self`] to use if [`Self::If::if`] is unsatisfied.
        ///
        /// Defaults to [`Self::None`].
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Box<Self>
    },
    /// Indexes [`Self::StringMap::map`] with [`Self::StringMap::value`] and uses that [`Self`].
    ///
    /// If the call to [`Map::get`] returns [`None`], returns [`None`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    StringMap {
        /// The value to index [`Self::StringMap::map`] with.
        value: StringSource,
        /// The value to index with [`Self::StringMap::value`].
        #[serde(flatten)]
        map: Box<Map<Self>>
    },
    /// Indexes [`Self::PartMap::map`] with [`Self::PartMap::part`] and uses that [`Self`].
    ///
    /// If the call to [`Map::get`] returns [`None`], returns [`None`].
    PartMap {
        /// The value to index [`Self::PartMap::map`] with.
        part: UrlPart,
        /// The value to index with [`Self::PartMap::part`].
        #[serde(flatten)]
        map: Box<Map<Self>>
    },
    /// Returns the contained list.
    ///
    /// Serializes and deserializes to and from a list.
    Literal(Vec<String>),
    /// Gets a list from [`Params::lists`].
    ///
    /// If [`StringSource::String`], serializes and deserializes to and from a string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, GetListError))]
    Params(StringSource)
}

impl ListSource {
    /// Get the list.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn get<'a>(&'a self, task_state: &TaskStateView<'a>) -> Result<Option<&'a Vec<String>>, GetListError> {
        Ok(match self {
            Self::None => None,
            Self::If {r#if, r#then, r#else} => if r#if.check(task_state)? {r#then} else {r#else}.get(task_state)?,
            Self::StringMap {value, map} => match map.get(get_option_str!(value, task_state)) {
                Some(source) => source.get(task_state)?,
                None => None
            },
            Self::PartMap {part, map} => match map.get(part.get(task_state.url)) {
                Some(source) => source.get(task_state)?,
                None => None
            },
            Self::Literal(x) => Some(x),
            Self::Params(x) => task_state.params.lists.get(get_str!(x, task_state, GetListError))
        })
    }
}

/// The enum of errors [`ListSource::get`] can return.
#[derive(Debug, Error)]
pub enum GetListError {
    /// Returned when a [`StringSource`] returned [`None`] where it has to return [`Some`].
    #[error("A StringSource returned None where it had to return Some.")]
    StringSourceIsNone,
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),

    /// Returned when a [`ConditionError`] is encountered.
    #[error(transparent)]
    ConditionError(#[from] Box<ConditionError>)
}

impl From<ConditionError> for GetListError {
    fn from(value: ConditionError) -> Self {
        Self::ConditionError(value.into())
    }
}

impl Serialize for ListSource {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Ok(match self {
            Self::None => serializer.serialize_none()?,
            Self::Literal(list) => {
                let mut serializer = serializer.serialize_seq(Some(list.len()))?;
                for x in list {
                    serializer.serialize_element(x)?;
                }
                serializer.end()?
            },
            Self::Params(StringSource::String(x)) => serializer.serialize_str(x)?,
            _ => Self::serialize(self, serializer)?
        })
    }
}

impl<'de> Deserialize<'de> for ListSource {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(ListSourceVisitor)
    }
}

/// [`Visitor`] for [`ListSource`].
#[derive(Debug)]
struct ListSourceVisitor;

impl<'de> Visitor<'de> for ListSourceVisitor {
    type Value = ListSource;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a list, null, or another variant written normally.")
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

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut ret = Vec::with_capacity(seq.size_hint().unwrap_or(8));
        while let Some(x) = seq.next_element()? {
            ret.push(x);
        }
        Ok(Self::Value::Literal(ret))
    }

    fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        Self::Value::deserialize(serde::de::value::MapAccessDeserializer::new(map))
    }
}
