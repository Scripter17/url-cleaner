//! Allows passing additional details into various types in URL Cleaner.

use std::collections::{HashMap, HashSet};
use std::path::Path;
#[cfg(feature = "cache")]
use std::io::{self, BufRead, Write};
use std::fs::{OpenOptions, File};

use serde::{Serialize, Deserialize};
use thiserror::Error;
use url::Url;

use super::*;

/// Configuration options to choose the behaviour of various URL Cleaner types.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
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

impl Default for Params {
    fn default() -> Self {
        Self {
            vars: HashMap::default(),
            flags: HashSet::default(),
            #[cfg(feature = "cache")] read_cache: true,
            #[cfg(feature = "cache")] write_cache: true,
            #[cfg(all(feature = "http", not(target_family = "wasm")))]
            http_client_config: HttpClientConfig::default()
        }
    }
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
    /// Returned when an [`std::io::Error`] is encountered.
    #[error(transparent)]
    IoError(#[from] std::io::Error)
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
        #[cfg(feature = "debug")]
        eprintln!("=== Params::http_client ===\nself: {self:?}\nhttp_client_config_diff: {http_client_config_diff:?}");
        match http_client_config_diff {
            Some(http_client_config_diff) => {
                let mut temp_http_client_config = self.http_client_config.clone();
                http_client_config_diff.apply(&mut temp_http_client_config);
                temp_http_client_config.apply(reqwest::blocking::ClientBuilder::new())
            },
            None => {self.http_client_config.apply(reqwest::blocking::ClientBuilder::new())}
        }?.build()
    }

    /// Read lines from `redirect-cache.txt`.
    /// 
    /// If a line that starts with `before` then a tab is found, returns that URL as `Ok(Some(_))`.
    /// 
    /// If no such line is found, returns `Ok(None)`.
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

    /// Writes a newline, `before`, a tab, and `after` to `redirect-cache.txt`.
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
    /// Adds to [`Params::vars`]. Defaults to an empty [`HashMap`].
    #[serde(default)] pub vars  : HashMap<String, String>,
    /// Removes from [`Params::vars`]. Defaults to an empty [`HashSet`].
    #[serde(default)] pub unvars: HashSet<String>,
    /// Adds to [`Params::flags`]. Defaults to an empty [`HashSet`].
    #[serde(default)] pub flags  : HashSet<String>,
    /// Removes from [`Params::flags`] Defaults to an empty [`HashSet`].
    #[serde(default)] pub unflags: HashSet<String>,
    /// If [`Some`], sets [`Params::read_cache`]. Defaults to [`None`].
    #[cfg(feature = "cache")]
    #[serde(default)] pub read_cache : Option<bool>,
    /// If [`Some`], sets [`Params::write_cache`]. Defaults to [`None`].
    #[cfg(feature = "cache")]
    #[serde(default)] pub write_cache: Option<bool>,
    /// If [`Some`], calls [`HttpClientConfigDiff::apply`] with `to`'s [`HttpClientConfig`]. Defaults to [`None`].
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
    #[serde(default)] pub http_client_config_diff: Option<HttpClientConfigDiff>
}

impl ParamsDiff {
    /// Applies the differences specified in `self` to `to`.
    /// In order:
    /// 1. Extends `to.vars` with [`Self::vars`], overwriting any keys found in both.
    /// 2. Removes all keys found in [`Self::unvars`] from `to.vars`.
    /// 3. Extends `to.flags` with [`Self::flags`].
    /// 4. Removes all flags found in [`Self::unflags`] from `to.flags`.
    /// 5. If [`Self::read_cache`] is [`Some`], sets `to.read_cache` to the contained value.
    /// 6. If [`Self::write_cache`] is [`Some`], sets `to.write_cache` to the contained value.
    /// 7. If [`Self::http_client_config_diff`] is [`Some`], calls [`HttpClientConfigDiff::apply`] with `to.http_client_config`.
    pub fn apply(&self, to: &mut Params) {
        #[cfg(feature = "debug")]
        {
            let old_to = to.clone();
            to.vars.extend(self.vars.clone());
            for var in &self.unvars {to.vars.remove(var);}
            to.flags.extend(self.flags.clone());
            for flag in &self.unflags {to.flags.remove(flag);}
            #[cfg(feature = "cache")] if let Some(read_cache ) = self.read_cache  {to.read_cache  = read_cache ;}
            #[cfg(feature = "cache")] if let Some(write_cache) = self.write_cache {to.write_cache = write_cache;}
            #[cfg(all(feature = "http", not(target_family = "wasm")))] if let Some(http_client_config_diff) = &self.http_client_config_diff {http_client_config_diff.apply(&mut to.http_client_config);}
            eprintln!("=== ParamsDiff::apply ===\nold: {old_to:?}\nDiff: {self:?}\nnew: {to:?}");
        }
        #[cfg(not(feature = "debug"))]
        {
            to.vars.extend(self.vars.clone());
            for var in &self.unvars {to.vars.remove(var);}
            to.flags.extend(self.flags.clone());
            for flag in &self.unflags {to.flags.remove(flag);}
            #[cfg(feature = "cache")] if let Some(read_cache ) = self.read_cache  {to.read_cache  = read_cache ;}
            #[cfg(feature = "cache")] if let Some(write_cache) = self.write_cache {to.write_cache = write_cache;}
            #[cfg(all(feature = "http", not(target_family = "wasm")))] if let Some(http_client_config_diff) = &self.http_client_config_diff {http_client_config_diff.apply(&mut to.http_client_config);}
        }
    }
}
