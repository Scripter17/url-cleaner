//! Provides [`Config`] which controls all details of how URL Cleaner works.

use std::fs::read_to_string;
use std::path::Path;
use std::borrow::Cow;
use std::io;
#[cfg(feature = "default-config")]
use std::sync::OnceLock;

use serde::{Serialize, Deserialize};

use crate::types::*;
use crate::glue::*;
use crate::util::*;

mod params;
pub use params::*;
mod docs;
pub use docs::*;
mod common_call;
pub use common_call::*;
mod commons;
pub use commons::*;

/// The rules and rule parameters describing how to modify URLs.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Config {
    /// The documentation.
    #[serde(default, skip_serializing_if = "is_default")]
    pub docs: ConfigDocs,
    /// Restricts this [`Config`] to only allow stuff suitable for the default config.
    /// 
    /// The exact behavior from setting this to [`true`] is currently unspecified and subject to change.
    /// 
    /// Defaults to [`false`].
    #[serde(default = "get_false", skip_serializing_if = "is_false")]
    pub strict_mode: bool,
    /// The path to use for the [`Cache`].
    /// 
    /// Defaults to `:memory:` to store the cache in RAM and not read/write any files.
    #[cfg(feature = "cache")]
    #[serde(default, skip_serializing_if = "is_default")]
    pub cache_path: CachePath,
    /// The parameters passed into the rule's conditions and mappers.
    #[serde(default, skip_serializing_if = "is_default")]
    pub params: Params,
    /// The tests to make sure the config is working as intended.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tests: Vec<TestSet>,
    /// Various things that are used in multiple spots.
    #[serde(default, skip_serializing_if = "is_default")]
    pub commons: Commons,
    /// The [`Rule`]s that modify the URLS.
    pub rules: Rules
}

impl Config {
    /// Loads and parses the specified file.
    /// # Errors
    /// If the specified file can't be loaded, returns the error [`GetConfigError::CantLoadConfigFile`].
    /// 
    /// If the config contained in the specified file can't be parsed, returns the error [`GetConfigError::CantParseConfigFile`].
    pub fn load_from_file<T: AsRef<Path>>(path: T) -> Result<Self, GetConfigError> {
        serde_json::from_str(&read_to_string(path).map_err(GetConfigError::CantLoadConfigFile)?).map_err(GetConfigError::CantParseConfigFile)
    }

    /// Gets the config compiled into the URL Cleaner binary.
    /// 
    /// On the first call, it parses [`DEFAULT_CONFIG_STR`] and caches it in [`DEFAULT_CONFIG`]. On all future calls it simply returns the cached value.
    /// # Errors
    /// If the default config cannot be parsed, returns the error [`GetConfigError::CantParseDefaultConfig`].
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

    /// Useful for when you know you're only getting the config once and, if needed, caching it yourself.
    /// 
    /// Generally, [`Self::get_default`] should be used over calling this function multiple times.
    /// # Errors
    /// If the default config cannot be parsed, returns the error [`GetConfigError::CantParseDefaultConfig`].
    #[cfg(feature = "default-config")]
    pub fn get_default_no_cache() -> Result<Self, GetConfigError> {
        serde_json::from_str(DEFAULT_CONFIG_STR).map_err(GetConfigError::CantParseDefaultConfig)
    }

    /// If `path` is `Some`, returns [`Self::load_from_file`].
    /// 
    /// If `path` is `None`, returns [`Self::get_default`].
    /// # Errors
    /// If `path` is `None` and the call to [`Self::get_default`] returns an error, that error is returned.
    /// 
    /// If `path` is `Some` and the call to [`Self::load_from_file`] returns an error, that error is returned.
    #[allow(dead_code, reason = "Public API.")]
    #[cfg(feature = "default-config")]
    pub fn get_default_or_load<T: AsRef<Path>>(path: Option<T>) -> Result<Cow<'static, Self>, GetConfigError> {
        Ok(match path {
            Some(path) => Cow::Owned(Self::load_from_file(path)?),
            None => Cow::Borrowed(Self::get_default()?)
        })
    }

    /// Useful for when you know you're only getting the config once and, if needed, caching it yourself.
    /// 
    /// Generally, [`Self::get_default_or_load`] should be used over calling this function with the same argument multiple times.
    /// # Errors
    /// If the default config cannot be parsed, returns the error [`GetConfigError::CantParseDefaultConfig`].
    #[cfg(feature = "default-config")]
    pub fn get_default_no_cache_or_load<T: AsRef<Path>>(path: Option<T>) -> Result<Self, GetConfigError> {
        Ok(match path {
            Some(path) => Self::load_from_file(path)?,
            None => Self::get_default_no_cache()?
        })
    }

    /// Basic wrapper around [`Self::rules`]'s [`Rules::apply`].
    /// # Errors
    /// If the call to [`Rules::apply`] returns an error, that error is returned.
    pub fn apply(&self, job_state: &mut JobState) -> Result<(), ApplyConfigError> {
        self.rules.apply(job_state).map_err(Into::into)
    }

    /// Basic wrapper around [`Self::rules`]'s [`Rules::apply_no_revert`].
    /// # Errors
    /// If the call to [`Rules::apply_no_revert`] returns an error, that error is returned.
    pub fn apply_no_revert(&self, job_state: &mut JobState) -> Result<(), ApplyConfigError> {
        self.rules.apply_no_revert(job_state).map_err(Into::into)
    }

    /// Runs the tests specified in [`Self::tests`], panicking when any error happens.
    /// # Panics
    /// Panics if a test fails.
    pub fn run_tests(&self) {
        // Changing the if's braces to parenthesis causes some really weird syntax errors. Including the `Ok(DEFAULT_CONFIG.get_or_init(|| config))` line above complaining about needing braces???
        if self.strict_mode {assert!(self.is_suitable_for_release());}
        for test in &self.tests {
            test.run(self);
        }
    }

    /// Internal method to make sure I don't accidentally commit Debug variants and other stuff unsuitable for the default config.
    pub(crate) fn is_suitable_for_release(&self) -> bool {
        assert!(self.commons.is_suitable_for_release(self) && self.params.is_suitable_for_release(self) && self.rules.is_suitable_for_release(self), "Unsuitable Config detected: {self:?}");
        true
    }
}

/// The enum of errors [`Config::apply`] can return.
/// 
/// Exists for future compatibility.
#[derive(Debug, Error)]
pub enum ApplyConfigError {
    /// Returned when a [`RuleError`] is encountered.
    #[error(transparent)]
    RuleError(#[from] RuleError)
}

/// The default [`Config`] as minified JSON.
#[cfg(feature = "default-config")]
pub const DEFAULT_CONFIG_STR: &str = include_str!(concat!(env!("OUT_DIR"), "/default-config.json.minified"));
/// The container for caching the parsed version of [`DEFAULT_CONFIG_STR`].
#[cfg(feature = "default-config")]
#[allow(dead_code, reason = "Public API.")]
pub static DEFAULT_CONFIG: OnceLock<Config> = OnceLock::new();

/// An enum containing all possible errors that can happen when loading/parsing a config.
#[derive(Debug, Error)]
pub enum GetConfigError {
    /// Could not load the specified config file.
    #[error(transparent)]
    CantLoadConfigFile(io::Error),
    /// The loaded config file did not contain valid JSON.
    #[error(transparent)]
    CantParseConfigFile(serde_json::Error),
    /// The default config compiled into URL Cleaner isn't valid JSON.
    #[error(transparent)]
    #[cfg(feature = "default-config")]
    CantParseDefaultConfig(serde_json::Error)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, reason = "Panicking tests are easier to write than erroring tests.")]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "default-config")]
    fn deserialize_default_config() {
        Config::get_default().unwrap();
    }

    #[test]
    #[cfg(feature = "default-config")]
    fn reserialize_default_config() {
        serde_json::to_string(&Config::get_default().unwrap()).unwrap();
    }

    /// Does not work when generic.
    /// 
    /// <'a, T: Serialize+Deserialize<'a>> throws nonsensical errors like `y.to_owned()` freed while still in use despite being an owned value.
    #[cfg(feature = "default-config")]
    fn de_ser(config: &Config) -> Config {
        serde_json::from_str(&serde_json::to_string(config).unwrap()).unwrap()
    }

    #[test]
    #[cfg(feature = "default-config")]
    fn default_config_de_ser_identity() {
        assert_eq!(Config::get_default().unwrap(),                 &de_ser(Config::get_default().unwrap())  );
        assert_eq!(Config::get_default().unwrap(),         &de_ser(&de_ser(Config::get_default().unwrap())) );
        assert_eq!(Config::get_default().unwrap(), &de_ser(&de_ser(&de_ser(Config::get_default().unwrap()))));
    }

    #[test]
    #[cfg(feature = "default-config")]
    fn test_default_config() {
        Config::get_default().unwrap().clone().run_tests();
    }
}
