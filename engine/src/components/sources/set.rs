//! [`SetSource`].

use crate::prelude::*;

/// Get a set.
///
/// Defauls to [`Self::None`].
///
/// Strings deserialize into [`Self::Params`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(remote = "Self")]
pub enum SetSource {
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
    /// Returns the contained set.
    ///
    /// Serializes and deserializes to and from a list.
    Literal(Set<String>),
    /// Gets a set from [`Params::sets`].
    ///
    /// If [`StringSource::String`], serializes to a string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, SetSourceError))]
    Params(#[suitable(assert = "set_is_documented")] StringSource),
    /// Get a set from [`FunctionArgs::sets`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, SetSourceError))]
    FunctionArg(StringSource)
}

impl SetSource {
    /// [`Self::get`], replacing [`None`] with the sub-error [`SetNotFound`].
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::get`] returns [`None`], returns the sub-error [`SetNotFound`].
    pub fn get_some<'j>(&'j self, task_state: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Result<&'j Set<String>, SetNotFound>, SetSourceError> {
        self.get(task_state, args).map(|x| x.ok_or(SetNotFound))
    }

    /// Get the set.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn get<'j>(&'j self, task_state: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Option<&'j Set<String>>, SetSourceError> {
        debug!(SetSource::get, self; match self {
            Self::Literal(x) => Ok(Some(x)),
            Self::Params(StringSource::String(x)) => Ok(task_state.job.cleaner.params.sets.get(x)),
            x => x._get(task_state, args)
        })
    }

    /// [`Self::get`].
    fn _get<'j>(&'j self, task_state: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Option<&'j Set<String>>, SetSourceError> {
        Ok(match self {
            Self::None => None,
            Self::If {r#if, r#then, r#else} => if r#if.check(task_state, args)? {get!(?r#then)} else {get!(?r#else)},
            Self::StringMap {value, map} => match map.get(get!(?&value)) {
                Some(source) => get!(?source),
                None => None
            },
            Self::PartMap {part, map} => match map.get(part.get(&task_state.url)) {
                Some(source) => get!(?source),
                None => None
            },
            Self::Literal(x) => Some(x),
            Self::Params(x) => task_state.job.cleaner.params.sets.get(get!(&x)),
            Self::FunctionArg(name) => args.ok_or(SetSourceError::NotInFunction)?.sets.get(get!(&name)),
        })
    }
}

/// The enum of errors [`ListSource::get`] can return.
#[derive(Debug, Error)]
pub enum SetSourceError {
    /// [`StringNotFound`].
    #[error(transparent)]
    StringNotFound(#[from] StringNotFound),
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),

    /// Returned when a [`ConditionError`] is encountered.
    #[error(transparent)]
    ConditionError(#[from] Box<ConditionError>),

    /// Returned when attempting to use [`FunctionArgs`] outside a function.
    #[error("Attempted to use FunctionArgs outside a function.")]
    NotInFunction
}

impl From<ConditionError> for SetSourceError {
    fn from(value: ConditionError) -> Self {
        Self::ConditionError(value.into())
    }
}

impl Serialize for SetSource {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Ok(match self {
            Self::None => serializer.serialize_none()?,
            Self::Literal(set) => {
                let mut serializer = serializer.serialize_seq(Some(set.set.len() + (set.if_none as usize)))?;
                if set.if_none {
                    serializer.serialize_element(&None::<&String>)?;
                }
                for x in &set.set {
                    serializer.serialize_element(&Some(x))?;
                }
                serializer.end()?
            },
            Self::Params(StringSource::String(x)) => serializer.serialize_str(x)?,
            _ => Self::serialize(self, serializer)?
        })
    }
}

impl<'de> Deserialize<'de> for SetSource {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(SetSourceVisitor)
    }
}

/// [`Visitor`] for [`SetSource`].
#[derive(Debug)]
struct SetSourceVisitor;

impl<'de> Visitor<'de> for SetSourceVisitor {
    type Value = SetSource;

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
        let mut ret = Set::with_capacity(seq.size_hint().unwrap_or(8));
        while let Some(x) = seq.next_element()? {
            ret.insert(x);
        }
        Ok(Self::Value::Literal(ret))
    }

    fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        Self::Value::deserialize(serde::de::value::MapAccessDeserializer::new(map))
    }
}
