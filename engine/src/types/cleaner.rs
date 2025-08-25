//! The configuration for how a URL should be cleaned.

use std::fs::read_to_string;
use std::path::Path;
use std::borrow::Cow;
use std::io;
#[cfg(feature = "default-cleaner")]
use std::sync::OnceLock;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::types::*;
use crate::testing::*;
use crate::util::*;

pub mod params;
pub use params::*;
pub mod docs;
pub use docs::*;
pub mod common_call;
pub use common_call::*;
pub mod commons;
pub use commons::*;

/// The config that determines all behavior of how URLs are cleaned.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct Cleaner<'a> {
    /// The documentation.
    ///
    /// Defaults to an empty [`CleanerDocs`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub docs: Cow<'a, CleanerDocs>,
    /// Tuning shared between all [`Task`]s spawned with this [`Cleaner`].
    ///
    /// Defaults to an empty [`Params`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub params: Params<'a>,
    /// Basically functions.
    ///
    /// Defaults to an empty [`Commons`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub commons: Cow<'a, Commons>,
    /// The [`Action`]s to apply.
    ///
    /// Defaults to an empty [`Vec`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub actions: Cow<'a, [Action]>
}

impl<'a> Cleaner<'a> {
    /// Create a new [`Self`] that [`Cow::Borrowed`]s all fields.
    ///
    /// Basically a very cheap [`Clone`] that you can apply [`ParamsDiff`]s to.
    pub fn borrowed(&'a self) -> Self {
        Self {
            docs   : Cow::Borrowed(&*self.docs),
            params : self.params.borrowed(),
            commons: Cow::Borrowed(&*self.commons),
            actions: Cow::Borrowed(&*self.actions)
        }
    }

    /// Load [`Self`] from a JSON file.
    /// # Errors
    #[doc = edoc!(callerr(std::fs::read_to_string), callerr(serde_json::from_str))]
    pub fn load_from_file<T: AsRef<Path>>(path: T) -> Result<Cleaner<'static>, GetCleanerError> {
        serde_json::from_str(&read_to_string(path)?).map_err(Into::into)
    }

    /// Gets the default [`Self`] compiled into the binary itself.
    ///
    /// If you know you're only going to get the default config once, [`Self::get_default_no_cache`] is better because you can apply [`ParamsDiff`]s to it without [`Clone::clone`]ing.
    /// # Errors
    #[doc = edoc!(callerr(Self::get_default_no_cache))]
    /// If the call to [`Self::get_default_no_cache`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// Cleaner::get_default().unwrap();
    /// ```
    #[cfg(feature = "default-cleaner")]
    pub fn get_default() -> Result<&'static Cleaner<'static>, GetCleanerError> {
        if let Some(config) = DEFAULT_CLEANER.get() {
            Ok(config)
        } else {
            let config = Self::get_default_no_cache()?;
            Ok(DEFAULT_CLEANER.get_or_init(|| config))
        }
    }

    /// Deserializes [`DEFAULT_CLEANER_STR`] and returns it without caching.
    ///
    /// If you're getting the default config often and rarely using [`ParamsDiff`]s, [`Self::get_default`] may be better due to it only deserializing the config once.
    /// # Errors
    #[doc = edoc!(callerr(serde_json::from_str))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// Cleaner::get_default_no_cache().unwrap();
    /// ```
    #[cfg(feature = "default-cleaner")]
    pub fn get_default_no_cache() -> Result<Cleaner<'static>, GetCleanerError> {
        serde_json::from_str(DEFAULT_CLEANER_STR).map_err(Into::into)
    }

    /// If `path` is [`Some`], returns the result of [`Self::load_from_file`] in a [`Cow::Owned`].
    ///
    /// If `path` is [`None`], returns the result of [`Self::get_default`] in a [`Cow::Borrowed`].
    /// # Errors
    #[doc = edoc!(callerr(Self::load_from_file), callerr(Self::get_default))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert_eq!(
    ///     Cleaner::get_default().unwrap(),
    ///     &*Cleaner::load_or_get_default(None::<&str>).unwrap()
    /// );
    ///
    /// assert_eq!(
    ///     Cleaner::get_default().unwrap(),
    ///     &*Cleaner::load_or_get_default(Some("default-cleaner.json")).unwrap()
    /// );
    /// ```
    #[cfg(feature = "default-cleaner")]
    pub fn load_or_get_default<T: AsRef<Path>>(path: Option<T>) -> Result<Cow<'static, Self>, GetCleanerError> {
        Ok(match path {
            Some(path) => Cow::Owned(Self::load_from_file(path)?),
            None => Cow::Borrowed(Self::get_default()?)
        })
    }

    /// If `path` is [`Some`], returns the result of [`Self::load_from_file`].
    ///
    /// If `path` is [`None`], returns the result of [`Self::get_default_no_cache`].
    /// # Errors
    #[doc = edoc!(callerr(Self::load_from_file), callerr(Self::get_default_no_cache))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert_eq!(
    ///     Cleaner::get_default_no_cache().unwrap(),
    ///     Cleaner::load_or_get_default_no_cache(None::<&str>).unwrap()
    /// );
    ///
    /// assert_eq!(
    ///     Cleaner::get_default_no_cache().unwrap(),
    ///     Cleaner::load_or_get_default_no_cache(Some("default-cleaner.json")).unwrap()
    /// );
    /// ```
    #[cfg(feature = "default-cleaner")]
    pub fn load_or_get_default_no_cache<T: AsRef<Path>>(path: Option<T>) -> Result<Self, GetCleanerError> {
        Ok(match path {
            Some(path) => Self::load_from_file(path)?,
            None => Self::get_default_no_cache()?
        })
    }

    /// Applies each [`Action`] in [`Self::actions`] in order to the provided [`TaskState`].
    ///
    /// If an error is returned, `task_state` may be left in a partially modified state.
    /// # Errors
    #[doc = edoc!(applyerr(Action, 3))]
    pub fn apply(&self, task_state: &mut TaskState) -> Result<(), ApplyCleanerError> {
        for action in &*self.actions {
            action.apply(task_state)?;
        }
        Ok(())
    }

    /// Runs the provided [`Tests`], panicking if any of them fail.
    /// # Panics
    /// If any [`Test`] fails, panics.
    pub fn run_tests(&self, tests: Tests) {
        tests.r#do(self);
    }

    /// Asserts the suitability of `self` to be URL Cleaner's default config.
    ///
    /// Exact behavior is unspecified and changes are not considered breaking.
    /// # Panics
    /// If `self` is deemed unsuitable to be URL Cleaner's default config, panics.
    #[cfg_attr(feature = "default-cleaner", doc = "# Examples")]
    #[cfg_attr(feature = "default-cleaner", doc = "```")]
    #[cfg_attr(feature = "default-cleaner", doc = "use url_cleaner_engine::types::*;")]
    #[cfg_attr(feature = "default-cleaner", doc = "")]
    #[cfg_attr(feature = "default-cleaner", doc = "Cleaner::get_default().unwrap().assert_suitability();")]
    #[cfg_attr(feature = "default-cleaner", doc = "```")]
    pub fn assert_suitability(&self) {
        Suitability::assert_suitability(self, self)
    }
}

/// The enum of errors [`Cleaner::apply`] can return.
#[derive(Debug, Error)]
pub enum ApplyCleanerError {
    /// Returned when a [`ActionError`] is encountered.
    #[error(transparent)]
    ActionError(#[from] ActionError)
}

/// The JSON text of the default config.
#[cfg(all(feature = "default-cleaner", not(test)))]
pub const DEFAULT_CLEANER_STR: &str = include_str!(concat!(env!("OUT_DIR"), "/default-cleaner.json.minified"));
/// The JSON text of the default config.
#[cfg(all(feature = "default-cleaner", test))]
pub const DEFAULT_CLEANER_STR: &str = include_str!("../../default-cleaner.json");
/// The cached deserialization of the default config.
#[cfg(feature = "default-cleaner")]
static DEFAULT_CLEANER: OnceLock<Cleaner> = OnceLock::new();

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
