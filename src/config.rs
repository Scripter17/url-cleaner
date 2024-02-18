use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::path::Path;
use std::borrow::Cow;
#[cfg(feature = "default-config")]
use std::sync::OnceLock;

use serde::{Serialize, Deserialize};
use thiserror::Error;
use url::Url;
#[cfg(all(feature = "http", not(target_family = "wasm")))]
use reqwest::header::HeaderMap;

use crate::rules::Rules;

/// The rules and rule parameters describing how to modify URLs.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// The parameters passed into the rule's conditions and mappers.
    #[serde(default)]
    pub params: Params,
    /// The conditions and mappers that modify the URLS.
    pub rules: Rules
}

impl Config {
    /// Loads and parses the specified file.
    /// # Errors
    /// If the specified file can't be loaded, returns the error [`GetConfigError::CantLoadFile`].
    /// If the config contained in the specified file can't be parsed, returns the error [`GetConfigError::CantParseFile`].
    pub fn load_from_file(path: &Path) -> Result<Self, GetConfigError> {
        Ok(serde_json::from_str(&read_to_string(path).or(Err(GetConfigError::CantLoadFile))?)?)
    }

    /// Gets the config compiled into the URL Cleaner binary.
    /// On the first call, it parses [`DEFAULT_CONFIG_STR`] and caches it in [`DEFAULT_CONFIG`]. On all future calls it simply returns the cached value.
    /// # Errors
    /// If the default config cannot be parsed, returns the error [`GetConfigError::CantParseDefaultConfig`].
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
    /// If `path` is `None`, returns [`Self::get_default`].
    /// # Errors
    /// If `path` is `None` and the call to [`Self::get_default`] returns an error, that error is returned.
    /// If `path` is `Some` and the call to [`Self::load_from_file`] returns an error, that error is returned.
    pub fn get_default_or_load(path: Option<&Path>) -> Result<Cow<'static, Self>, GetConfigError> {
        Ok(match path {
            Some(path) => Cow::Owned(Self::load_from_file(path)?),
            None => Cow::Borrowed(Self::get_default()?)
        })
    }

    /// Applies the rules to the provided URL using the parameters contained in [`Self::params`].
    /// # Errors
    /// If the call to `Rules::apply` returns an error, that error is returned.
    #[allow(dead_code)]
    pub fn apply(&self, url: &mut Url,) -> Result<(), crate::rules::RuleError> {
        self.rules.apply(url, &self.params)
    }
}

/// Configuration options to choose the behaviour of a few select [`crate::rules::Condition`]s and [`crate::rules::Mapper`]s.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Params {
    /// Works with [`crate::rules::Condition::RuleVariableIs'`].
    #[serde(default)]
    pub vars: HashMap<String, String>,
    /// Works with [`crate::rules::Condition::FlagIsSet`].
    #[serde(default)]
    pub flags: HashSet<String>,
    /// The default headers to send in HTTP requests.
    #[cfg(all(feature = "http", feature = "regex", not(target_family = "wasm")))]
    #[serde(default, with = "crate::glue::headermap")]
    pub default_http_headers: HeaderMap
}

#[cfg(all(feature = "http", not(target_family = "wasm")))]
impl Params {
    /// Gets an HTTP client with [`self`]'s configuration pre-applied.
    /// # Errors
    /// Errors if [`reqwest::blocking::ClientBuilder::build`] errors.
    pub fn http_client(&self) -> reqwest::Result<reqwest::blocking::Client> {
        reqwest::blocking::ClientBuilder::new()
            .default_headers(self.default_http_headers.clone())
            .build()
    }
}

impl Params {
    /// Overwrites part of `self` with `from`.
    pub fn merge(&mut self, from: Self) {
        self.vars.extend(from.vars);
        self.flags.extend(from.flags);
        #[cfg(all(feature = "http", not(target_family = "wasm")))]
        self.default_http_headers.extend(from.default_http_headers);
    }
}

/// The config loaded into URL Cleaner at compile time.
/// When the `minify-included-strings` is enabled, all whitespace is replaced with a single space.
/// If there are any spaces in a string, this compression will alter how the config works.
/// `{"x":     "y"}` is compressed but functionally unchanged, but `{"x   y": "z"}` will be converted to `{"x y": "z"}`, which could alter the functionality of the rule.
/// If you cannot avoid multiple spaces in a string, turn off the `minify-default-strings` feature to disable this compression.
#[cfg(all(feature = "default-config", feature = "minify-included-strings"))]
pub static DEFAULT_CONFIG_STR: &str=const_str::squish!(include_str!("../default-config.json"));
/// The non-minified config loaded into URL Cleaner at compile time.
#[cfg(all(feature = "default-config", not(feature = "minify-included-strings")))]
pub static DEFAULT_CONFIG_STR: &str=include_str!("../default-config.json");
/// The container for caching the parsed version of [`DEFAULT_CONFIG_STR`].
#[cfg(feature = "default-config")]
pub static DEFAULT_CONFIG: OnceLock<Config>=OnceLock::new();

/// An enum containing all possible errors that can happen when loading/parsing a rules into a [`Rules`]
#[derive(Error, Debug)]
pub enum GetConfigError {
    /// Could not load the specified rules file.
    #[error("Could not load the specified rules file.")]
    CantLoadFile,
    /// The loaded file did not contain valid JSON.
    #[error("Can't parse config file: `{0}`.")]
    CantParseFile(#[from] serde_json::Error),
    /// URL Cleaner was compiled without default rules.
    #[allow(dead_code)]
    #[error("URL Cleaner was compiled without default config.")]
    NoDefaultConfig,
    /// The default rules compiled into URL Cleaner aren't valid JSON.
    #[allow(dead_code)]
    #[error("Can't parse default config: `{0}`.")]
    CantParseDefaultConfig(serde_json::Error)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn parse_default_rules() {
        Config::get_default().unwrap();
    }
}
