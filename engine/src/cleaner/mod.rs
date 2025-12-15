//! [`Cleaner`] and co.

use std::fs::read_to_string;
use std::path::Path;
use std::borrow::Cow;
use std::io;
#[cfg(feature = "bundled-cleaner")]
use std::sync::OnceLock;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::prelude::*;

pub mod profiled_cleaner;
pub mod params;
pub mod params_diff;
pub mod docs;
pub mod functions;
pub mod function_call;
pub mod call_args;
pub mod components;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::profiled_cleaner::prelude::*;
    pub use super::params::*;
    pub use super::params_diff::*;
    pub use super::docs::*;
    pub use super::functions::*;
    pub use super::function_call::*;
    pub use super::call_args::*;
    pub use super::components::prelude::*;

    pub use super::{Cleaner, GetCleanerError, ApplyCleanerError};

    #[cfg(feature = "bundled-cleaner")]
    pub use super::BUNDLED_CLEANER_STR;
}

/// The main unit describing how to clean URLs.
///
/// See the documentation for [`Params`] and [`ProfiledCleaner`] for why everything's a [`Cow`].
///
/// I promise it's not just me liking the funny name.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct Cleaner<'a> {
    /// The [`Docs`].
    ///
    /// Defaults to an empty [`Docs`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub docs: Cow<'a, Docs>,
    /// The [`Params`].
    ///
    /// Defaults to an empty [`Params`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub params: Params<'a>,
    /// The [`Functions`].
    ///
    /// Defaults to an empty [`Functions`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub functions: Cow<'a, Functions>,
    /// The [`Action`]s.
    ///
    /// Defaults to an empty [`Vec`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub action: Cow<'a, Action>
}

/// The JSON text of the bundled cleaner.
///
/// Please see [`Cleaner::get_bundled`] and co.
#[cfg(all(feature = "bundled-cleaner", not(test)))]
pub const BUNDLED_CLEANER_STR: &str = include_str!(concat!(env!("OUT_DIR"), "/bundled-cleaner.json.minified"));
/// The JSON text of the bundled cleaner.
///
/// Please see [`Cleaner::get_bundled`] and co.
#[cfg(all(feature = "bundled-cleaner", test))]
pub const BUNDLED_CLEANER_STR: &str = include_str!("bundled-cleaner.json");
/// The cached deserialization of the bundled cleaner.
#[cfg(feature = "bundled-cleaner")]
static BUNDLED_CLEANER: OnceLock<Cleaner<'static>> = OnceLock::new();

impl<'a> Cleaner<'a> {
    /// Create a new [`Self`] that [`Cow::Borrowed`]s all fields (and [`Params::borrowed`]s [`Self::params`]).
    ///
    /// Used to enable both [`ProfiledCleaner`] and [`ParamsDiff`] to be much more memory efficient tha otherwise possible.
    pub fn borrowed(&'a self) -> Cleaner<'a> {
        Self {
            docs     : Cow::Borrowed(&*self.docs),
            params   : self.params.borrowed(),
            functions: Cow::Borrowed(&*self.functions),
            action   : Cow::Borrowed(&*self.action)
        }
    }

    /// Become an owned [`Self`], cloning only what's needed.
    pub fn into_owned(self) -> Cleaner<'static> {
        Cleaner {
            docs     : Cow::Owned(self.docs.into_owned()),
            params   : self.params.into_owned(),
            functions: Cow::Owned(self.functions.into_owned()),
            action   : Cow::Owned(self.action.into_owned())
        }
    }

    /// Load [`Self`] from a JSON file.
    /// # Errors
    #[doc = edoc!(callerr(std::fs::read_to_string), callerr(serde_json::from_str))]
    pub fn load_from_file<T: AsRef<Path>>(path: T) -> Result<Cleaner<'static>, GetCleanerError> {
        serde_json::from_str(&read_to_string(path)?).map_err(Into::into)
    }

    /// Gets the cached bundled cleaner, parsing it if not already cached.
    /// # Errors
    #[doc = edoc!(callerr(Self::get_bundled_no_cache))]
    /// If the call to [`Self::get_bundled_no_cache`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// Cleaner::get_bundled().unwrap();
    /// ```
    #[cfg(feature = "bundled-cleaner")]
    pub fn get_bundled() -> Result<&'static Cleaner<'static>, GetCleanerError> {
        if let Some(cleaner) = BUNDLED_CLEANER.get() {
            Ok(cleaner)
        } else {
            let cleaner = Self::get_bundled_no_cache()?;
            Ok(BUNDLED_CLEANER.get_or_init(|| cleaner))
        }
    }

    /// Parses a new copy the bundled cleaner.
    /// # Errors
    #[doc = edoc!(callerr(serde_json::from_str))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// Cleaner::get_bundled_no_cache().unwrap();
    /// ```
    #[cfg(feature = "bundled-cleaner")]
    pub fn get_bundled_no_cache() -> Result<Cleaner<'static>, GetCleanerError> {
        serde_json::from_str(BUNDLED_CLEANER_STR).map_err(Into::into)
    }

    /// Either [`Self::load_from_file`] or [`Self::get_bundled`].
    /// # Errors
    #[doc = edoc!(callerr(Self::load_from_file), callerr(Self::get_bundled))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// assert_eq!(
    ///     Cleaner::get_bundled().unwrap(),
    ///     &*Cleaner::load_or_get_bundled(None::<&str>).unwrap()
    /// );
    ///
    /// assert_eq!(
    ///     Cleaner::get_bundled().unwrap(),
    ///     &*Cleaner::load_or_get_bundled(Some("src/cleaner/bundled-cleaner.json")).unwrap()
    /// );
    /// ```
    #[cfg(feature = "bundled-cleaner")]
    pub fn load_or_get_bundled<T: AsRef<Path>>(path: Option<T>) -> Result<Cow<'static, Cleaner<'static>>, GetCleanerError> {
        Ok(match path {
            Some(path) => Cow::Owned(Self::load_from_file(path)?),
            None => Cow::Borrowed(Self::get_bundled()?)
        })
    }

    /// Either [`Self::load_from_file`] or [`Self::get_bundled_no_cache`].
    /// # Errors
    #[doc = edoc!(callerr(Self::load_from_file), callerr(Self::get_bundled_no_cache))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// assert_eq!(
    ///     Cleaner::get_bundled_no_cache().unwrap(),
    ///     Cleaner::load_or_get_bundled_no_cache(None::<&str>).unwrap()
    /// );
    ///
    /// assert_eq!(
    ///     Cleaner::get_bundled_no_cache().unwrap(),
    ///     Cleaner::load_or_get_bundled_no_cache(Some("src/cleaner/bundled-cleaner.json")).unwrap()
    /// );
    /// ```
    #[cfg(feature = "bundled-cleaner")]
    pub fn load_or_get_bundled_no_cache<T: AsRef<Path>>(path: Option<T>) -> Result<Cleaner<'static>, GetCleanerError> {
        Ok(match path {
            Some(path) => Self::load_from_file(path)?,
            None => Self::get_bundled_no_cache()?
        })
    }

    /// Asserts the suitability of `self` to be URL Cleaner's bundled cleaner.
    ///
    /// Exact behavior is unspecified and changes are not considered breaking.
    /// # Panics
    /// If `self` is deemed unsuitable to be URL Cleaner's bundled cleaner, panics.
    #[cfg_attr(feature = "bundled-cleaner", doc = "# Examples")]
    #[cfg_attr(feature = "bundled-cleaner", doc = "```")]
    #[cfg_attr(feature = "bundled-cleaner", doc = "use url_cleaner_engine::prelude::*;")]
    #[cfg_attr(feature = "bundled-cleaner", doc = "")]
    #[cfg_attr(feature = "bundled-cleaner", doc = "Cleaner::get_bundled().unwrap().assert_suitability();")]
    #[cfg_attr(feature = "bundled-cleaner", doc = "```")]
    pub fn assert_suitability(&self) {
        Suitability::assert_suitability(self, self)
    }
}

impl<'j> Cleaner<'j> {
    /// [`Action::apply`]s [`Self::action`].
    /// # Errors
    #[doc = edoc!(applyerr(Action, 3))]
    pub fn apply(&'j self, task_state: &mut TaskState<'j>) -> Result<(), ApplyCleanerError> {
        self.action.apply(task_state)?;
        Ok(())
    }
}

/// The enum of errors that can happen when loading a [`Cleaner`].
#[derive(Debug, Error)]
pub enum GetCleanerError {
    /// Returned when loading a [`Cleaner`] fails.
    #[error(transparent)]
    CantLoadCleaner(#[from] io::Error),
    /// Returned when deserializing a [`Cleaner`] fails.
    #[error(transparent)]
    CantParseCleaner(#[from] serde_json::Error),
}

/// The enum of errors [`Cleaner::apply`] can return.
#[derive(Debug, Error)]
pub enum ApplyCleanerError {
    /// Returned when a [`ActionError`] is encountered.
    #[error(transparent)]
    ActionError(#[from] ActionError)
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "bundled-cleaner")]
    fn minificiation_test() {
        use super::*;

        let minified   = include_str!(concat!(env!("OUT_DIR"), "/bundled-cleaner.json.minified"));
        let unminified = include_str!("bundled-cleaner.json");

        assert_eq!(
            serde_json::from_str::<Cleaner>(minified  ).expect("Deserializing the minified bundled cleaner to work"),
            serde_json::from_str::<Cleaner>(unminified).expect("Deserializing the unminified bundled cleaner to work"),
            "The minified version was improperly made."
        );

        assert!(minified.len() < unminified.len(), "The minified version isn't minified???");
    }
}
