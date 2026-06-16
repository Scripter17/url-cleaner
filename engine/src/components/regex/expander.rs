//! [`RegexExpansion`].

use crate::prelude::*;

/// Expands a [`regex::Captures`] more powerfully.
/// # Examples
/// ```
/// use url_cleaner_engine::{task_state, prelude::*};
///
/// let task_state = task_state!("https://e.newsletters.cnn.com/click?EcmgzMjUyNzA1LmFkZGFAYmxvZ2dlci5jb20/CeyJtaWQiOiIxNzQ5OTkyNTE0NTA4OTEwYTZmOGM4OTljIiwiY3QiOiJjbm4tNmQwZWU3ZmNmNjRkZjIxY2VkZTg1OWJmMDhmYjA2NmMtMSIsInJkIjoiYmxvZ2dlci5jb20ifQ/VaHR0cHM6Ly93d3cuY25uLmNvbS8yMDI0LzA2LzE0L2hlYWx0aC9mYXRoZXJob29kLWdvb2QtZm9yLWRhZHMtd2VsbG5lc3M/SWkhfQ05OX2lfTmV3c19OREJBTjA2MTUyMDI1YzE2OTQwMDBiMQ/LY24x/qP3V0bV9zb3VyY2U9Y25uX0ZpdmUrVGhpbmdzK2ZvcitTdW5kYXklMkMrSnVuZSsxNSUyQysyMDI1JnV0bV9tZWRpdW09ZW1haWwmYnRfZWU9Mm5lSjJLZU9KZWllQ0JRVnBoTSUyRlBNQmp4S3FYeUc4aUhlY0NVNHljRiUyQiUyRm5YSEJkRSUyRiUyQmVhVnRJa2ZScnJESmMmYnRfdHM9MTc0OTk5MjUxNDUxMA/gaE7EWw/JMDYxNTIwMjVDMTY5NDAwMEIx/sdo767ff5f7");
///
/// let x = serde_json::from_str::<StringSource>(r#"
/// {"RegexExpansion": {
///   "value": {"Part": "Whole"},
///   "regex": "(?:/[VH]([^/]+)|/q([^/]+)|/[^/]+)+",
///   "expansion": {"Join": [
///     {"Modified": {
///       "value": {"Get": 1},
///       "modification": {"All": [{"Base64Decode": {}}, {"StripBefore": "http"}]}
///     }},
///     {"Modified": {
///       "value": {"Get": 2},
///       "modification": {"Base64Decode": {}}
///     }}
///   ]}
/// }}
/// "#).unwrap();
///
/// assert_eq!(x.get(&task_state, None).unwrap().unwrap(), "https://www.cnn.com/2024/06/14/health/fatherhood-good-for-dads-wellness?utm_source=cnn_Five+Things+for+Sunday%2C+June+15%2C+2025&utm_medium=email&bt_ee=2neJ2KeOJeieCBQVphM%2FPMBjxKqXyG8iHecCU4ycF%2B%2FnXHBdE%2F%2BeaVtIkfRrrDJc&bt_ts=1749992514510");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub enum RegexExpansion {
    /// [`StringSource::get`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    String(StringSource),
    /// Joins a list of [`Self`].
    ///
    /// Segments that evaluate to [`None`] are omitted.
    /// # Errors
    /// If any call to [`Self::expand`] returns an error, that error is returned.
    Join(Vec<Self>),
    /// [`regex::Captures::get`].
    Get(usize),
    /// [`regex::Captures::name`].
    Name(String),
    /// Apply [`Self::Modified::modification`] to [`Self::Modified::value`].
    /// # Errors
    /// If the call to [`StringModification::apply`] returns an error, that error is returned.
    Modified {
        /// The [`Self`] to modify.
        value: Box<Self>,
        /// The [`StringModification`].
        modification: StringModification
    }
}

/// The enum of errors [`RegexExpansion::expand`] can return.
#[derive(Debug, Error)]
pub enum RegexExpansionError {
    /// [`StringSourceError`].
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
    /// [`StringModificationError`].
    #[error(transparent)]
    StringModificationError(#[from] StringModificationError),
}

impl RegexExpansion {
    /// Expand `captures`.
    /// # Errors
    /// See each variant of [`Self`] for details.
    pub fn expand<'j: 't, 't: 'h, 'h>(&'j self, task_state: &'t TaskState<'j>, args: Option<&'j FunctionArgs>, captures: &regex::Captures<'h>) -> Result<Option<Cow<'h, str>>, RegexExpansionError> {
        Ok(match self {
            Self::String   (x) => get!(?x),
            Self::Join     (x) => Some(x.iter().filter_map(|y| y.expand(task_state, args, captures).transpose()).collect::<Result<String, RegexExpansionError>>()?.into()),
            Self::Get      (x) => captures.get (*x).map(|x| x.as_str().into()),
            Self::Name     (x) => captures.name( x).map(|x| x.as_str().into()),
            Self::Modified {value, modification} => {
                let mut ret = value.expand(task_state, args, captures)?;
                modification.apply(task_state, args, &mut ret)?;
                ret
            }
        })
    }
}
