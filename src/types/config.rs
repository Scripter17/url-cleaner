//! Provides [`Config`] which controls all details of how URL Cleaner works.

use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use std::borrow::Cow;
use std::io;
#[cfg(feature = "default-config")]
use std::sync::OnceLock;

use serde::{Serialize, Deserialize};

use crate::types::*;
use crate::util::is_default;

mod params;
pub use params::*;
#[cfg(all(feature = "http", not(target_family = "wasm")))] mod http_client_config;
#[cfg(all(feature = "http", not(target_family = "wasm")))] pub use http_client_config::*;

/// The rules and rule parameters describing how to modify URLs.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Config {
    /// The parameters passed into the rule's conditions and mappers.
    #[serde(default, skip_serializing_if = "is_default")]
    pub params: Params,
    /// The path of the sqlite cache to use.
    #[cfg(feature = "cache")]
    pub cache_path: PathBuf,
    /// The tests to make sure the config is working as intended.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tests: Vec<TestSet>,
    /// The conditions and mappers that modify the URLS.
    pub rules: Rules
}

impl Config {
    /// Loads and parses the specified file.
    /// # Errors
    /// If the specified file can't be loaded, returns the error [`GetConfigError::CantLoadConfigFile`].
    /// 
    /// If the config contained in the specified file can't be parsed, returns the error [`GetConfigError::CantParseConfigFile`].
    pub fn load_from_file(path: &Path) -> Result<Self, GetConfigError> {
        serde_json::from_str(&read_to_string(path).map_err(GetConfigError::CantLoadConfigFile)?).map_err(GetConfigError::CantParseConfigFile)
    }

    /// Gets the config compiled into the URL Cleaner binary.
    /// On the first call, it parses [`DEFAULT_CONFIG_STR`] and caches it in [`DEFAULT_CONFIG`]. On all future calls it simply returns the cached value.
    /// # Errors
    /// If the default config cannot be parsed, returns the error [`GetConfigError::CantParseDefaultConfig`].
    /// 
    /// If URL Cleaner was compiled without a default config, returns the error [`GetConfigError::NoDefaultConfig`].
    pub fn get_default() -> Result<&'static Self, GetConfigError> {
        #[cfg(feature = "default-config")]
        {
            if let Some(config) = DEFAULT_CONFIG.get() {
                Ok(config)
            } else {
                let config=serde_json::from_str(DEFAULT_CONFIG_STR).map_err(GetConfigError::CantParseDefaultConfig)?;
                Ok(DEFAULT_CONFIG.get_or_init(|| config))
            }
        }
        #[cfg(not(feature = "default-config"))]
        Err(GetConfigError::NoDefaultConfig)
    }

    /// If `path` is `Some`, returns [`Self::load_from_file`].
    /// 
    /// If `path` is `None`, returns [`Self::get_default`].
    /// # Errors
    /// If `path` is `None` and the call to [`Self::get_default`] returns an error, that error is returned.
    /// 
    /// If `path` is `Some` and the call to [`Self::load_from_file`] returns an error, that error is returned.
    pub fn get_default_or_load(path: Option<&Path>) -> Result<Cow<'static, Self>, GetConfigError> {
        Ok(match path {
            Some(path) => Cow::Owned(Self::load_from_file(path)?),
            None => Cow::Borrowed(Self::get_default()?)
        })
    }

    /// Runs the tests specified in [`Self::tests`], panicking when any error happens.
    /// # Panics
    /// Panics if a call to [`Job::do`] or a test fails.
    pub fn run_tests(&self) {
        for test in &self.tests {
            test.run(self.clone());
        }
    }
}

/// The config loaded into URL Cleaner at compile time.
/// 
/// When the `minify-included-strings` is enabled, all whitespace is replaced with a single space.
/// If there are any spaces in a string, this compression will alter how the config works.
/// 
/// `{"x":     "y"}` is compressed but functionally unchanged, but `{"x   y": "z"}` will be converted to `{"x y": "z"}`, which could alter the functionality of the rule.
/// 
/// If you cannot avoid multiple spaces in a string, turn off the `minify-default-strings` feature to disable this compression.
#[cfg(all(feature = "default-config", feature = "minify-included-strings"))]
pub static DEFAULT_CONFIG_STR: &str=const_str::squish!(include_str!("../../default-config.json"));
/// The non-minified config loaded into URL Cleaner at compile time.
#[cfg(all(feature = "default-config", not(feature = "minify-included-strings")))]
pub static DEFAULT_CONFIG_STR: &str=include_str!("../../default-config.json");
/// The container for caching the parsed version of [`DEFAULT_CONFIG_STR`].
#[cfg(feature = "default-config")]
pub static DEFAULT_CONFIG: OnceLock<Config>=OnceLock::new();

/// An enum containing all possible errors that can happen when loading/parsing a rules into a [`Rules`]
#[derive(Debug, Error)]
pub enum GetConfigError {
    /// Could not load the specified config file.
    #[error(transparent)]
    CantLoadConfigFile(io::Error),
    /// The loaded config file did not contain valid JSON.
    #[error(transparent)]
    CantParseConfigFile(serde_json::Error),
    /// URL Cleaner was compiled without default config.
    #[allow(dead_code)]
    #[error("URL Cleaner was compiled without default config.")]
    NoDefaultConfig,
    /// The default config compiled into URL Cleaner isn't valid JSON.
    #[allow(dead_code)]
    #[error(transparent)]
    CantParseDefaultConfig(serde_json::Error)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_default_config() {
        Config::get_default().unwrap();
    }

    #[test]
    fn reserialize_default_config() {
        serde_json::to_string(&Config::get_default().unwrap()).unwrap();
    }

    /// Does not work when generic.
    /// <'a, T: Serialize+Deserialize<'a>> throws nonsensical errors like `y.to_owned()` freed while still in use despite being an owned value.
    fn de_ser(config: &Config) -> Config {
        serde_json::from_str(&serde_json::to_string(config).unwrap()).unwrap()
    }

    #[test]
    fn default_config_de_ser_identity() {
        assert_eq!(Config::get_default().unwrap(), &de_ser(                Config::get_default().unwrap()  ));
        assert_eq!(Config::get_default().unwrap(), &de_ser(&de_ser(        Config::get_default().unwrap() )));
        assert_eq!(Config::get_default().unwrap(), &de_ser(&de_ser(&de_ser(Config::get_default().unwrap()))));
    }

    #[test]
    fn test_default_config() {
        Config::get_default().unwrap().clone().run_tests();
    }
}
