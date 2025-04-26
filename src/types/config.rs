//! The configuration for how a URL should be cleaned.

use std::fs::read_to_string;
use std::path::Path;
use std::borrow::Cow;
use std::io;
#[cfg(feature = "default-config")]
use std::sync::OnceLock;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::types::*;
use crate::glue::*;
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
pub struct Config {
    /// The documentation.
    ///
    /// Defaults to an empty [`ConfigDocs`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub docs: ConfigDocs,
    /// The location of the cache.
    ///
    /// Defaults to being stored in memory and destroyed on program exit.
    #[cfg(feature = "cache")]
    #[serde(default, skip_serializing_if = "is_default")]
    pub cache_path: CachePath,
    /// Fine tuning shared between all [`Task`]s of a [`Job`] and maybe multiple [`Job`]s.
    ///
    /// Defaults to an empty [`Params`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub params: Params,
    /// Basically functions.
    ///
    /// Defaults to an empty [`Commons`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub commons: Commons,
    /// The [`Action`]s to apply.
    ///
    /// Defaults to an empty [`Vec`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub actions: Vec<Action>
}

impl Config {
    /// Load [`Self`] from a JSON file.
    /// # Errors
    /// If the call to [`std::fs::read_to_string`] returns an error, that error is returned.
    ///
    /// If the call to [`serde_json::from_str`] returns an error, that error is returned.
    pub fn load_from_file<T: AsRef<Path>>(path: T) -> Result<Self, GetConfigError> {
        serde_json::from_str(&read_to_string(path)?).map_err(Into::into)
    }

    /// Gets the default [`Self`] compiled into the binary itself.
    ///
    /// Caching is done by putting the [`Self`] in [`DEFAULT_CONFIG`] and only returning references to it.
    ///
    /// If you know you're only going to get the default config once, [`Self::get_default_no_cache`] is better because you can apply [`ParamsDiff`]s to it without [`Clone::clone`]ing.
    /// # Errors
    /// If the call to [`Self::get_default_no_cache`] returns an error, that error is returned.
    #[allow(dead_code, reason = "Public API.")]
    #[cfg(feature = "default-config")]
    pub fn get_default() -> Result<&'static Self, GetConfigError> {
        if let Some(config) = DEFAULT_CONFIG.get() {
            Ok(config)
        } else {
            let config = Self::get_default_no_cache()?;
            Ok(DEFAULT_CONFIG.get_or_init(|| config))
        }
    }

    /// Deserializes [`DEFAULT_CONFIG_STR`] and returns it without caching in [`DEFAULT_CONFIG`]
    ///
    /// If you're getting the default config often and rarely using [`ParamsDiff`]s, [`Self::get_default`] may be better due to it only deserializing the config once.
    /// # Errors
    /// If the call to [`serde_json::from_str`] returns an error, that error is returned.
    #[cfg(feature = "default-config")]
    pub fn get_default_no_cache() -> Result<Self, GetConfigError> {
        serde_json::from_str(DEFAULT_CONFIG_STR).map_err(Into::into)
    }

    /// If `path` is [`Some`], returns the result of [`Self::load_from_file`] in a [`Cow::Owned`].
    ///
    /// If `path` is [`None`], returns the result of [`Self::get_default`] in a [`Cow::Borrowed`].
    /// # Errors
    /// If the call to [`Self::load_from_file`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::get_default`] returns an error, that error is returned.
    #[allow(dead_code, reason = "Public API.")]
    #[cfg(feature = "default-config")]
    pub fn load_or_get_default<T: AsRef<Path>>(path: Option<T>) -> Result<Cow<'static, Self>, GetConfigError> {
        Ok(match path {
            Some(path) => Cow::Owned(Self::load_from_file(path)?),
            None => Cow::Borrowed(Self::get_default()?)
        })
    }

    /// If `path` is [`Some`], returns the result of [`Self::load_from_file`].
    ///
    /// If `path` is [`None`], returns the result of [`Self::get_default_no_cache`].
    /// # Errors
    /// If the call to [`Self::load_from_file`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::get_default`] returns an error, that error is returned.
    #[cfg(feature = "default-config")]
    pub fn load_or_get_default_no_cache<T: AsRef<Path>>(path: Option<T>) -> Result<Self, GetConfigError> {
        Ok(match path {
            Some(path) => Self::load_from_file(path)?,
            None => Self::get_default_no_cache()?
        })
    }

    /// Applies each [`Action`] in [`Self::actions`] in order to the provided [`TaskState`].
    ///
    /// If an error is returned, `job_state` may be left in a partially modified state.
    /// # Errors
    /// If any call to [`Action::apply`] returns an error, that error is returned.
    #[allow(dead_code, reason = "Public API.")]
    pub fn apply(&self, job_state: &mut TaskState) -> Result<(), ApplyConfigError> {
        for action in &self.actions {
            action.apply(job_state)?;
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
    #[cfg_attr(feature = "default-config", doc = "# Examples")]
    #[cfg_attr(feature = "default-config", doc = "```")]
    #[cfg_attr(feature = "default-config", doc = "# use url_cleaner::types::*;")]
    #[cfg_attr(feature = "default-config", doc = "Config::get_default().unwrap().assert_suitability();")]
    #[cfg_attr(feature = "default-config", doc = "```")]
    pub fn assert_suitability(&self) {
        Suitability::assert_suitability(self, self)
    }
}

/// The enum of errors [`Config::apply`] can return.
#[derive(Debug, Error)]
pub enum ApplyConfigError {
    /// Returned when a [`ActionError`] is encountered.
    #[error(transparent)]
    ActionError(#[from] ActionError)
}

/// The JSON text of the default config.
#[cfg(all(feature = "default-config", not(test)))]
pub const DEFAULT_CONFIG_STR: &str = include_str!(concat!(env!("OUT_DIR"), "/default-config.json.minified"));
/// The JSON text of the default config.
#[cfg(all(feature = "default-config", test))]
pub const DEFAULT_CONFIG_STR: &str = include_str!("../../default-config.json");
/// The cached deserialization of the default config.
#[cfg(feature = "default-config")]
#[allow(dead_code, reason = "Public API.")]
pub static DEFAULT_CONFIG: OnceLock<Config> = OnceLock::new();

/// The enum of errors that can happen when loading a [`Config`].
#[derive(Debug, Error)]
pub enum GetConfigError {
    /// Returned when loading a [`Config`] fails.
    #[error(transparent)]
    CantLoadConfig(#[from] io::Error),
    /// Returned when deserializing a [`Config`] fails.
    #[error(transparent)]
    CantParseConfig(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "default-config")]
    fn default_config_is_json() {
        serde_json::from_str::<serde_json::Value>(DEFAULT_CONFIG_STR).unwrap();
    }

    #[test]
    #[cfg(feature = "default-config")]
    fn default_config_is_valid() {
        Config::get_default().unwrap();
    }

    #[test]
    #[cfg(feature = "default-config")]
    fn serde_roundtrip_equality() {
        let default_config = Config::get_default().unwrap();

        assert_eq!(&serde_json::from_str::<Config>(&serde_json::to_string(default_config).unwrap()).unwrap(), default_config);
    }
}
