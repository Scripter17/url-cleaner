//! Provides [`Config`] which controls all details of how URL Cleaner works.

use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::path::Path;
use std::borrow::Cow;
use std::io;
#[cfg(feature = "default-config")]
use std::sync::OnceLock;
#[cfg(feature = "cache")]
use std::{
    io::{BufRead, Write},
    fs::{OpenOptions, File}
};

use serde::{Serialize, Deserialize};
use thiserror::Error;
use url::Url;
#[cfg(all(feature = "http", not(target_family = "wasm")))]
use reqwest::header::HeaderMap;

use crate::types::*;

/// The rules and rule parameters describing how to modify URLs.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Config {
    /// The parameters passed into the rule's conditions and mappers.
    #[serde(default)]
    pub params: Params,
    /// The tests to make sure the config is working as intended.
    #[serde(default)]
    pub tests: Vec<ConfigTest>,
    /// The conditions and mappers that modify the URLS.
    pub rules: Rules
}

impl Config {
    /// Loads and parses the specified file.
    /// # Errors
    /// If the specified file can't be loaded, returns the error [`GetConfigError::CantLoadConfigFile`].
    /// If the config contained in the specified file can't be parsed, returns the error [`GetConfigError::CantParseConfigFile`].
    pub fn load_from_file(path: &Path) -> Result<Self, GetConfigError> {
        serde_json::from_str(&read_to_string(path).map_err(GetConfigError::CantLoadConfigFile)?).map_err(GetConfigError::CantParseConfigFile)
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
    /// If the call to [`Rules::apply`] returns an error, that error is returned.
    #[allow(dead_code)]
    pub fn apply(&self, url: &mut Url) -> Result<(), RuleError> {
        self.rules.apply(url, &self.params)
    }

    /// Runs the tests specified in [`Self::tests`], panicking when any error happens.
    /// # Panics
    /// Panics if a call to [`Self::apply`] or a test fails.
    pub fn run_tests(mut self) {
        let original_params = self.params.clone();
        for test in self.tests.clone() {
            let serialized_test = serde_json::to_string(&test)
                .expect("The test to serialize without errors"); // Only applies when testing a config.
            test.params_diff.apply(&mut self.params);
            for [mut before, after] in test.pairs {
                self.apply(&mut before).expect("The URL to be modified without errors."); // Only applies when testing a config.
                assert_eq!(before, after, "Test: {serialized_test}");
            }
            self.params = original_params.clone();
        }
    }
}

/// Configuration options to choose the behaviour of a few select [`Condition`]s and [`Mapper`]s.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Params {
    /// Works with [`Condition::RuleVariableIs'`].
    #[serde(default)]
    pub vars: HashMap<String, String>,
    /// Works with [`Condition::FlagIsSet`].
    #[serde(default)]
    pub flags: HashSet<String>,
    /// If [`true`], enables reading from caches. Defaults to [`true`]
    #[cfg(feature = "cache")]
    #[serde(default = "get_true")]
    pub read_cache: bool,
    /// If [`true`], enables writing to caches. Defaults to [`true`]
    #[cfg(feature = "cache")]
    #[serde(default = "get_true")]
    pub write_cache: bool,
    /// The default headers to send in HTTP requests.
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
    #[serde(default)]
    pub http_client_config: HttpClientConfig
}

/// Serde helper function.
const fn get_true() -> bool {true}

/// The enum of all errors [`Params::get_redirect_from_cache`] can return.
#[cfg(feature = "cache")]
#[derive(Debug, Error)]
pub enum ReadCacheError {
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError)
}

/// The enum of all errors [`Params::write_redirect_to_cache`] can return.
#[cfg(feature = "cache")]
#[derive(Debug, Error)]
pub enum WriteCacheError {
    /// Returned when an [`io::Error`] is encountered.
    #[error(transparent)]
    IoError(#[from] io::Error)
}

/// Helper function used to read from the cache.
#[cfg(feature = "cache")]
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

impl Params {
    /// Gets an HTTP client with [`Self`]'s configuration pre-applied.
    /// # Errors
    /// Errors if [`reqwest::ClientBuilder::build`] errors.
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
    pub fn http_client(&self, http_client_config_diff: Option<&HttpClientConfigDiff>) -> reqwest::Result<reqwest::blocking::Client> {
        match http_client_config_diff {
            Some(http_client_config_diff) => {
                let mut temp_http_client_config = self.http_client_config.clone();
                http_client_config_diff.apply(&mut temp_http_client_config);
                temp_http_client_config.apply(reqwest::blocking::ClientBuilder::new())
            },
            None => {self.http_client_config.apply(reqwest::blocking::ClientBuilder::new())}
        }.build()
    }

    /// # Errors
    /// If a cache line starting with `url` is found but the map isn't parseable as a URL, returns the error [`ReadCacheError::UrlParseError`].
    #[cfg(feature = "cache-redirects")]
    pub fn get_redirect_from_cache(&self, before: &Url) -> Result<Option<Url>, ReadCacheError> {
        if self.read_cache {
            if let Ok(lines) = read_lines("redirect-cache.txt") {
                for line in lines.map_while(Result::ok) {
                    if let Some((short, long)) = line.split_once('\t') {
                        if before.as_str()==short {
                            return Ok(Some(Url::parse(long)?));
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    /// # Errors
    /// If the cache line cannot be written, returns [`WriteCacheError::IoError`].
    #[cfg(feature = "cache-redirects")]
    pub fn write_redirect_to_cache(&self, before: &Url, after: &Url) -> Result<(), WriteCacheError> {
        if self.write_cache {
            if let Ok(mut x) = OpenOptions::new().create(true).append(true).open("redirect-cache.txt") {
                x.write_all(format!("\n{}\t{}", before.as_str(), after.as_str()).as_bytes())?;
            }
        }
        Ok(())
    }
}

/// Allows changing [`Config::params`].
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct ParamsDiff {
    /// Adds to [`Params::vars`].
    #[serde(default)] pub vars  : HashMap<String, String>,
    /// Removes from [`Params::vars`].
    #[serde(default)] pub unvars: HashSet<String>,
    /// Adds to [`Params::flags`].
    #[serde(default)] pub flags  : HashSet<String>,
    /// Removes from [`Params::flags`]
    #[serde(default)] pub unflags: HashSet<String>,
    /// If [`Some`], sets [`Params::read_cache`].
    #[cfg(feature = "cache")]
    #[serde(default)] pub read_cache : Option<bool>,
    /// If [`Some`], sets [`Params::write_cache`].
    #[cfg(feature = "cache")]
    #[serde(default)] pub write_cache: Option<bool>
}

impl ParamsDiff {
    /// Applies the differences specified in `self` to `to`.
    /// In order:
    /// 1. Extends `to.vars` with `self.vars`, overwriting any keys found in both.
    /// 2. Removes all keys found in `self.unvars` from `to.vars`.
    /// 3. Extends `to.flags` with `self.flags`.
    /// 4. Removes all flags found in `self.unflags` from `to.flags`.
    /// 5. If `self.read_cache` is [`Some`], sets `to.read_cache` to the contained value.
    /// 6. If `self.write_cache` is [`Some`], sets `to.write_cache` to the contained value.
    pub fn apply(&self, to: &mut Params) {
        to.vars.extend(self.vars.clone());
        for var in &self.unvars {to.vars.remove(var);}
        to.flags.extend(self.flags.clone());
        for flag in &self.unflags {to.flags.remove(flag);}
        #[cfg(feature = "cache")] if let Some(read_cache ) = self.read_cache  {to.read_cache  = read_cache ;}
        #[cfg(feature = "cache")] if let Some(write_cache) = self.write_cache {to.write_cache = write_cache;}
    }
}

/// Used by [`Params`] to detail how a [`reqwest::blocking::Client`] should be made.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HttpClientConfig {
    /// [`reqwest::blocking::ClientBuilder::default_headers`].
    #[serde(default, with = "crate::glue::headermap")]
    pub default_headers: HeaderMap,
    /// Roughly corresponds to [`reqwest::redirect::Policy`].
    #[serde(default)]
    pub redirect_policy: RedirectPolicy
}

/// Bandaid fix until [`reqwest::redirect::Policy`] stops sucking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RedirectPolicy {
    /// [`reqwest::redirect::Policy::limited`].
    Limited(usize),
    /// [`reqwest::redirect::Policy::none`].
    None
}

impl Default for RedirectPolicy {
    /// Defaults to `Self::Limited(10)` because that's what reqwest does.
    fn default() -> Self {
        Self::Limited(10)
    }
}

impl From<RedirectPolicy> for reqwest::redirect::Policy {
    fn from(value: RedirectPolicy) -> Self {
        match value {
            RedirectPolicy::Limited(x) => Self::limited(x),
            RedirectPolicy::None => Self::none()
        }
    }
}

impl HttpClientConfig {
    /// Unfortunately has to consume `client` due to [`reqwest::blocking::ClientBuilder`]'s API sucking.
    pub fn apply(&self, client: reqwest::blocking::ClientBuilder) -> reqwest::blocking::ClientBuilder {
        client.default_headers(self.default_headers.clone())
            .redirect(self.redirect_policy.clone().into())
    }
}

/// Allows changing [`HttpClientConfig`].
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HttpClientConfigDiff {
    /// Sets [`HttpClientConfig::redirect_policy`].
    #[serde(default)]
    pub redirect_policy: Option<RedirectPolicy>,
    /// Appends headers to [`HttpClientConfig::default_headers`].
    #[serde(default, with = "crate::glue::headermap")]
    pub default_headers: HeaderMap
}

impl HttpClientConfigDiff {
    /// Applies the differences specified in `self` to `to`.
    /// In order:
    /// 1. If [`Self::redirect_policy`] is [`Some`], overwrite `to`'s [`HttpClientConfig::redirect_policy`].
    /// 2. Append [`Self::default_headers`] to `to`'s [`HttpClientConfig::default_headers`].
    pub fn apply(&self, to: &mut HttpClientConfig) {
        if let Some(new_redirect_policy) = &self.redirect_policy {to.redirect_policy=new_redirect_policy.clone();}
        to.default_headers.extend(self.default_headers.clone());
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
#[derive(Error, Debug)]
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

/// Tests to make sure a [`Config`] is working as intended.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConfigTest {
    /// The [`ParamsDiff`] to apply to the [`Config::params`] for this test.
    #[serde(default)]
    pub params_diff: ParamsDiff,
    /// A list of URLs to test and the expected results.
    pub pairs: Vec<[Url; 2]>
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
