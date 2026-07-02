//! [`Task`] and co.

use crate::prelude::*;

/// A task for a [`Job`] to [`Job::do`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(remote = "Self")]
pub struct Task {
    /// The [`BetterUrl`] to modify.
    pub url: BetterUrl,
    /// The context.
    ///
    /// Defaults to [`TaskContext::default`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub context: TaskContext
}

string_or_struct_magic!(Task);

impl Task {
    /// Make a new [`Self`].
    /// # Errors
    /// If the call to [`TryInto::try_into`] returns an error, that error is returned.
    pub fn new<T: TryInto<Self>>(task: T) -> Result<Self, T::Error> {
        task.try_into()
    }
}

impl From<BetterUrl> for Task {fn from(url: BetterUrl) -> Self {Self {url, context: Default::default()}}}
impl From<url::Url > for Task {fn from(url: url::Url ) -> Self {BetterUrl::from(url).into()}}

impl FromStr for Task {
    type Err = MakeTaskError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.as_bytes() {
            [b'{' | b'"'                  , ..] => serde_json::from_str(s)?,
            [b'a' ..= b'z' | b'A' ..= b'Z', ..] => BetterUrl::parse(s)?.into(),
            [] => Err(MakeTaskError::IgnoreLineNotIgnored)?,
            _  => Err(MakeTaskError::OtherwiseInvalid)?
        })
    }
}

impl TryFrom<&str> for Task {
    type Error = MakeTaskError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<&[u8]> for Task {
    type Error = MakeTaskError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        str::from_utf8(value)?.parse()
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.context.is_empty() {
            true  => write!(formatter, "{}", self.url),
            // TODO: This is dumb.
            false => write!(formatter, "{}", serde_json::to_string(self).expect("To always work."))
        }
    }
}
