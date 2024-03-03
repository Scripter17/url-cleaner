use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::path::Path;
use std::borrow::Cow;
#[cfg(feature = "default-config")]
use std::sync::OnceLock;
#[cfg(feature = "cache-redirects")]
use std::{
    io::{self, BufRead, Write, Error as IoError},
    fs::{OpenOptions, File}
};

use serde::{Serialize, Deserialize};
use thiserror::Error;
use url::{Url, ParseError};
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
    pub fn apply(&self, url: &mut Url) -> Result<(), crate::rules::RuleError> {
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
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
    #[serde(default, with = "crate::glue::headermap")]
    pub default_http_headers: HeaderMap,
    /// If `true`, disables all form of logging to disk.
    /// Currently just caching HTTP redirects.
    #[serde(default)]
    pub amnesia: bool
}

#[derive(Debug, Error)]
pub enum ReadCacheError {
    #[error(transparent)]
    UrlParseError(#[from] ParseError)
}

#[derive(Debug, Error)]
pub enum WriteCacheError {
    #[cfg(feature = "cache-redirects")]
    #[error(transparent)]
    IoError(#[from] IoError)
}

#[cfg(feature = "cache-redirects")]
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


impl Params {
    /// Overwrites part of `self` with `from`.
    pub fn merge(&mut self, from: Self) {
        self.vars.extend(from.vars);
        self.flags.extend(from.flags);
        #[cfg(all(feature = "http", not(target_family = "wasm")))]
        self.default_http_headers.extend(from.default_http_headers);
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
    pub fn get_url_from_cache(&self, before: &Url) -> Result<Option<Url>, ReadCacheError> {
        #[cfg(feature = "cache-redirects")]
        if let Ok(lines) = read_lines("redirect-cache.txt") {
            for line in lines.map_while(Result::ok) {
                if let Some((short, long)) = line.split_once('\t') {
                    if before.as_str()==short {
                        return Ok(Some(Url::parse(long)?));
                    }
                }
            }
        }
        Ok(None)
    }

    /// # Errors
    /// If the cache line cannot be written, returns [`WriteCacheError::IoError`].
    pub fn write_url_map_to_cache(&self, before: &Url, after: &Url) -> Result<(), WriteCacheError> {
        #[cfg(feature = "cache-redirects")]
        if !self.amnesia {
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
    fn parse_default_config() {
        Config::get_default().unwrap();
    }

    macro_rules! test_config {
        ($c:expr, $f:expr, $t:expr) => {{
            let mut url = Url::parse($f).unwrap();
            $c.apply(&mut url).unwrap();
            assert_eq!(url.as_str(), $t);
        }}
    }

    macro_rules! set_var    {($c:expr, $n:expr, $v:expr) => {$c.params.vars .insert($n.to_string(), $v.to_string());}}
    macro_rules! unset_var  {($c:expr, $n:expr         ) => {$c.params.vars .remove($n                            );}}
    macro_rules! set_flag   {($c:expr, $n:expr         ) => {$c.params.flags.insert($n.to_string()                );}}
    macro_rules! unset_flag {($c:expr, $n:expr         ) => {$c.params.flags.remove($n                            );}}

    #[test]
    fn test_default_config() {
        let mut config = Config::get_default().unwrap().clone();

        test_config!(config, "https://x.com?t=a&s=b", "https://twitter.com/");

        set_var!    (config, "tor2web-suffix", "example");
        test_config!(config, "https://example.onion", "https://example.onion/");
        set_flag!   (config, "tor2web");
        test_config!(config, "https://example.onion", "https://example.onion.example/");
        test_config!(config, "https://example.onion.example2", "https://example.onion.example/");
        unset_flag! (config, "tor2web");
        set_flag!   (config, "tor2web2tor");
        test_config!(config, "https://example.onion.example", "https://example.onion/");
        unset_var!  (config, "tor2web-suffix");

        test_config!(config, "https://x.com?a=2", "https://twitter.com/");
        test_config!(config, "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id", "https://example.com/");
        test_config!(config, "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8", "https://www.amazon.ca/dp/B0C6DX66TN");

        set_flag!   (config, "unbreezewiki");
        test_config!(config, "https://antifandom.com/tardis/wiki/Genocide", "https://tardis.fandom.com/wiki/Genocide");
        unset_flag! (config, "unbreezewiki");
        set_flag!   (config, "breezewiki");
        test_config!(config, "https://antifandom.com/tardis/wiki/Genocide", "https://breezewiki.com/tardis/wiki/Genocide");
        test_config!(config, "https://tardis.fandom.com/wiki/Genocide"    , "https://breezewiki.com/tardis/wiki/Genocide");
        unset_flag! (config, "breezewiki");

        set_flag!   (config, "unmobile");
        test_config!(config, "https://en.m.wikipedia.org/wiki/Self-immolation_of_Aaron_Bushnell", "https://en.wikipedia.org/wiki/Self-immolation_of_Aaron_Bushnell");
        unset_flag! (config, "unmobile");

        config.apply(&mut Url::parse("https://127.0.0.1").unwrap()).unwrap();
    }
}
