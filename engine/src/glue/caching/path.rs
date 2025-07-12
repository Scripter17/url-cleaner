//! The home of [`CachePath`].

use std::str::FromStr;

use serde::{Serialize, Deserialize};

use crate::util::*;

/// The path of a cache database.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(remote = "Self")]
pub enum CachePath {
    /// Stores the database in memory, wiping it on program exit.
    ///
    /// Has the string representation of `:memory:`.
    #[default]
    Memory,
    /// A filesystem/network/whatever path.
    Path(String)
}

crate::util::string_or_struct_magic!(CachePath);

impl CachePath {
    /// The cache's path as a [`str`].
    ///
    /// If `self` is [`Self::Memory`], returns `:memory:`.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::glue::*;
    ///
    /// assert_eq!(CachePath::Memory                          .as_str(), ":memory:");
    /// assert_eq!(CachePath::Path(       "abc.sqlite".into()).as_str(), "abc.sqlite");
    /// assert_eq!(CachePath::Path("file://abc.sqlite".into()).as_str(), "file://abc.sqlite");
    /// ```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Memory => ":memory:",
            Self::Path(x) => x
        }
    }
}

impl AsRef<str> for CachePath {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for CachePath {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl From<&str> for CachePath {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<String> for CachePath {
    fn from(value: String) -> Self {
        match &*value {
            ":memory:" => Self::Memory,
            _ => Self::Path(value)
        }
    }
}
