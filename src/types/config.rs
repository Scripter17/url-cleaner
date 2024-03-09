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

use crate::rules::Rules;

/// The rules and rule parameters describing how to modify URLs.
#[derive(Debug, Clone, Deserialize, Serialize)]
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
    /// If the call to `Rules::apply` returns an error, that error is returned.
    #[allow(dead_code)]
    pub fn apply(&self, url: &mut Url) -> Result<(), crate::rules::RuleError> {
        self.rules.apply(url, &self.params)
    }

    /// # Panics
    /// Panics if a call to [`Self::apply`] or a test fails.
    pub fn run_tests(mut self) {
        let original_params = self.params.clone();
        for test in self.tests.clone() {
            self.params.apply_diff(test.params_diff);
            for [mut before, after] in test.pairs {
                self.apply(&mut before).expect("The URL to be modified without errors.");
                assert_eq!(before, after);
            }
            self.params = original_params.clone();
        }
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
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
    #[serde(default, with = "crate::glue::headermap")]
    pub default_http_headers: HeaderMap,
    /// If [`true`], enables reading from caches. Defaults to [`true`]
    #[cfg(feature = "cache")]
    #[serde(default = "get_true")]
    pub read_cache: bool,
    /// If [`true`], enables writing to caches. Defaults to [`true`]
    #[cfg(feature = "cache")]
    #[serde(default = "get_true")]
    pub write_cache: bool
}

const fn get_true() -> bool {true}

/// Allows changing [`Config::params`].
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
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

#[cfg(feature = "cache")]
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

impl Params {
    /// Overwrites part of `self` with `from`.
    pub fn apply_diff(&mut self, diff: ParamsDiff) {
        self.vars.extend(diff.vars);
        for var in diff.unvars {self.vars.remove(&var);}
        self.flags.extend(diff.flags);
        for flag in diff.unflags {self.flags.remove(&flag);}
        #[cfg(feature = "cache")] if let Some(read_cache ) = diff.read_cache  {self.read_cache  = read_cache ;}
        #[cfg(feature = "cache")] if let Some(write_cache) = diff.write_cache {self.write_cache = write_cache;}
    }

    /// Gets an HTTP client with [`Self`]'s configuration pre-applied.
    /// # Errors
    /// Errors if [`reqwest::blocking::ClientBuilder::build`] errors.
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
    pub fn http_client(&self) -> reqwest::Result<reqwest::blocking::Client> {
        reqwest::blocking::ClientBuilder::new()
            .default_headers(self.default_http_headers.clone())
            .build()
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

/// The config loaded into URL Cleaner at compile time.
/// When the `minify-included-strings` is enabled, all whitespace is replaced with a single space.
/// If there are any spaces in a string, this compression will alter how the config works.
/// `{"x":     "y"}` is compressed but functionally unchanged, but `{"x   y": "z"}` will be converted to `{"x y": "z"}`, which could alter the functionality of the rule.
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
    /// The default cpnfig compiled into URL Cleaner isn't valid JSON.
    #[allow(dead_code)]
    #[error(transparent)]
    CantParseDefaultConfig(serde_json::Error)
}

/// Tests to make sure a [`Config`] is working as intended.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

    #[test]
    fn test_default_config() {
        Config::get_default().unwrap().clone().run_tests();
    }
}
