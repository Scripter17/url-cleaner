//! Provides [`StringSource`] which allows for getting strings from various parts of URL Cleaner's current state.

use std::str::FromStr;
use std::convert::Infallible;
use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use url::Url;
use thiserror::Error;

use crate::types::*;
use crate::glue::*;

/// Allows conditions and mappers to get strings from various sources without requiring different conditions and mappers for each source.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(remote = "Self")]
pub enum StringSource {
    /// Always returns the error [`StringSourceError::ExplicitError`].
    /// # Errors
    /// Always returns the error [`StringSourceError::ExplicitError`].
    Error,
    /// Prints debugging information about the contained [`Self`] and the details of its execution to STDERR.
    /// Intended primarily for debugging logic errors.
    /// 
    /// *Can* be used in production as in both bash and batch `x | y` only pipes `x`'s STDOUT, but you probably shouldn't.
    /// # Errors
    /// If the contained [`Self`] returns an error, that error is returned after the debug info is printed.
    Debug(Box<Self>),
    /// Just a string. The most common variant.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringSource;
    /// # use url::Url;
    /// # use url_cleaner::types::Params;
    /// # use std::borrow::Cow;
    /// let url = Url::parse("https://example.com").unwrap();
    /// assert_eq!(StringSource::String("abc".to_string()).get(&url, &Params::default()).unwrap(), Some(Cow::Borrowed("abc")));
    /// ```
    String(String),
    /// Gets the specified URL part.
    /// # Errors
    /// If the call to [`UrlPart::get`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringSource;
    /// # use url::Url;
    /// # use url_cleaner::types::Params;
    /// # use std::borrow::Cow;
    /// # use url_cleaner::types::UrlPart;
    /// let url = Url::parse("https://example.com").unwrap();
    /// let params = Params::default();
    /// assert_eq!(StringSource::Part(UrlPart::Domain).get(&url, &Params::default()).unwrap(), Some(Cow::Borrowed("example.com")));
    /// ```
    Part(UrlPart),
    /// Gets the specified variable's value.
    /// 
    /// Returns [`None`] (NOT an error) if the variable is not set.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringSource;
    /// # use url::Url;
    /// # use url_cleaner::types::Params;
    /// # use std::borrow::Cow;
    /// # use std::collections::HashMap;
    /// let url = Url::parse("https://example.com").unwrap();
    /// let params = Params {vars: HashMap::from_iter([("abc".to_string(), "xyz".to_string())]), ..Params::default()};
    /// assert_eq!(StringSource::Var("abc".to_string()).get(&url, &params).unwrap(), Some(Cow::Borrowed("xyz")));
    /// ```
    Var(String),
    /// If the flag specified by `flag` is set, return the result of `then`. Otherwise return the result of `r#else`.
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringSource;
    /// # use url::Url;
    /// # use url_cleaner::types::Params;
    /// # use std::borrow::Cow;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::collections::HashSet;
    /// let url = Url::parse("https://example.com").unwrap();
    /// let params_1 = Params::default();
    /// let params_2 = Params {flags: HashSet::from_iter(["abc".to_string()]), ..Params::default()};
    /// let x = StringSource::IfFlag {flag: "abc".to_string(), then: Box::new(StringSource::String("xyz".to_string())), r#else: Box::new(StringSource::Part(UrlPart::Domain))};
    /// assert_eq!(x.get(&url, &params_1).unwrap(), Some(Cow::Borrowed("example.com")));
    /// assert_eq!(x.get(&url, &params_2).unwrap(), Some(Cow::Borrowed("xyz")));
    /// ```
    IfFlag {
        /// The name of the flag to check.
        flag: String,
        /// If the flag is set, use this.
        then: Box<Self>,
        /// If the flag is not set, use this.
        r#else: Box<Self>
    },
    /// Gets a string with `source`, modifies it with `modification`, and returns the result.
    /// # Errors
    /// If the call to [`StringModification::apply`] errors, returns that error.
    #[cfg(feature = "string-modification")]
    Modified {
        /// The source to get the string from.
        source: Box<Self>,
        /// The modification to apply to the string.
        modification: StringModification
    },
    /// Joins a list of strings. Effectively a [`slice::join`].
    /// By default, `join` is `""` so the strings are concatenated.
    /// # Errors
    /// If any call to [`Self::get`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringSource;
    /// # use url_cleaner::types::Params;
    /// # use url_cleaner::types::UrlPart;
    /// # use url::Url;
    /// # use std::borrow::Cow;
    /// assert_eq!(
    ///     StringSource::Join {
    ///         sources: vec![
    ///             StringSource::String(".".to_string()),
    ///             StringSource::Part(UrlPart::NotSubdomain)
    ///         ],
    ///         join: "".to_string()
    ///     }.get(
    ///         &Url::parse("https://abc.example.com.example.com").unwrap(),
    ///         &Params::default()
    ///     ).unwrap(),
    ///     Some(Cow::Owned(".example.com".to_string()))
    /// );
    /// ```
    Join {
        /// The list of string sources to join.
        sources: Vec<Self>,
        /// The value to join `sources` with. Defaults to an empty string.
        #[serde(default)]
        join: String
    },
    /// Parses `source` as a URL and gets the specified value.
    /// Useful when used with [`Self::HttpRequest`].
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`Url::parse`] returns an error, that error is returned.
    /// 
    /// If the call to [`UrlPart::get`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringSource;
    /// # use url_cleaner::types::Params;
    /// # use url_cleaner::types::UrlPart;
    /// # use url::Url;
    /// # use std::borrow::Cow;
    /// assert_eq!(
    ///     StringSource::ExtractPart {
    ///         source: Box::new(StringSource::String("https://example.com".to_string())),
    ///         part: UrlPart::Scheme
    ///     }.get(
    ///         &Url::parse("https://not-relevant-at-all.com").unwrap(),
    ///         &Params::default()
    ///     ).unwrap(),
    ///     Some(Cow::Borrowed("https"))
    /// );
    /// ```
    ExtractPart {
        /// The string to parse and extract `part` from.
        source: Box<Self>,
        /// The part to extract from `source`.
        part: UrlPart
    },
    /// Sends an HTTP request and returns a string from the response determined by the specified [`ResponseHandler`].
    /// # Errors
    /// If the call to [`RequestConfig::response`] returns an error, that error is returned.
    #[cfg(all(feature = "advanced-requests", not(target_family = "wasm")))]
    HttpRequest(Box<RequestConfig>),
    /// If the contained [`Self`] returns `None`, instead return `Some(Cow::Borrowed(""))`
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    NoneToEmptyString(Box<Self>),
    /// Run a command and return its output.
    /// # Errors
    /// If the call to [`CommandConfig::output`] returns an error, that error is returned.
    #[cfg(feature = "commands")]
    CommandOutput {
        /// The command to run.
        command: CommandConfig,
        /// The STDIN to put into the command.
        #[serde(default)]
        stdin: Option<Box<Self>>
    }
}

impl FromStr for StringSource {
    type Err = Infallible;

    /// Returns a [`Self::String`].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::String(s.to_string()))
    }
}

impl From<&str> for StringSource {
    /// Returns a [`Self::String`].
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

crate::util::string_or_struct_magic!(StringSource);

/// The enum of all possible errors [`StringSource::get`] can return.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum StringSourceError {
    /// Returned when [`StringSource::Error`] is used.
    #[error("StringSource::Error was used.")]
    ExplicitError,
    /// Returned when a [`StringModificationError`] is encountered.
    #[cfg(feature = "string-modification")]
    #[error(transparent)]
    StringModificationError(#[from] StringModificationError),
    /// Returned when [`reqwest::Error`] is encountered.
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    /// Returned when a requested HTTP response header is not found.
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
    #[error("The HTTP request response did not contain the requested header.")]
    HeaderNotFound,
    /// Returned when a [`reqwest::header::ToStrError`] is encountered.
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
    #[error(transparent)]
    HeaderToStrError(#[from] reqwest::header::ToStrError),
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when a regex does not find any matches.
    #[error("A regex pattern did not find any matches.")]
    #[cfg(feature = "regex")]
    NoRegexMatchesFound,
    /// Returned when a call to [`StringSource::get`] returns `None` where it has to be `Some`.
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,
    /// Returned when a [`RequestConfigError`] is encountered.
    #[cfg(all(feature = "advanced-requests", not(target_family = "wasm")))]
    #[error(transparent)]
    RequestConfigError(#[from] RequestConfigError),
    /// Returned when a [`ResponseHandlerError`] is encountered.
    #[cfg(all(feature = "advanced-requests", not(target_family = "wasm")))]
    #[error(transparent)]
    ReponseHandlerError(#[from] ResponseHandlerError),
    /// Returned when a [`CommandError`] is encountered.
    #[cfg(feature = "commands")]
    #[error(transparent)]
    CommandError(#[from] CommandError)
}

impl StringSource {
    /// Gets the string from the source.
    /// # Errors
    /// See each of [`Self`]'s variant's documentation for details.
    pub fn get<'a>(&'a self, url: &'a Url, params: &'a Params) -> Result<Option<Cow<'a, str>>, StringSourceError> {
        #[cfg(feature = "debug")]
        println!("Source: {self:?}");
        Ok(match self {
            Self::String(x) => Some(Cow::Borrowed(x.as_str())),
            Self::Part(x) => x.get(url),
            Self::Var(x) => params.vars.get(x).map(|x| Cow::Borrowed(x.as_str())),
            Self::IfFlag {flag, then, r#else} => if params.flags.contains(flag) {then} else {r#else}.get(url, params)?,
            #[cfg(feature = "string-modification")]
            Self::Modified {source, modification} => {
                match source.as_ref().get(url, params)? {
                    Some(x) => {
                        let mut x = x.into_owned();
                        modification.apply(&mut x, params)?;
                        Some(Cow::Owned(x))
                    },
                    None => None
                }
            },
            // I love that [`Result`] and [`Option`] implement [`FromIterator`].
            // It's so silly but it works SO well.
            Self::Join {sources, join} => sources.iter().map(|source| source.get(url, params)).collect::<Result<Option<Vec<_>>, _>>()?.map(|x| Cow::Owned(x.join(join))),
            // Transpose wouldn't need to exist in a world where `.map(|x| x?)` worked.
            Self::ExtractPart{source, part} => source.get(url, params)?.map(|x| Url::parse(&x)).transpose()?.and_then(|x| part.get(&x).map(|x| Cow::Owned(x.into_owned()))),
            #[cfg(all(feature = "advanced-requests", not(target_family = "wasm")))]
            Self::HttpRequest(config) => Some(Cow::Owned(config.response(url, params)?)),
            Self::NoneToEmptyString(source) => source.get(url, params)?.or(Some(Cow::Borrowed(""))),
            Self::Debug(source) => {
                let ret=source.get(url, params);
                eprintln!("=== StringSource::Debug ===\nSource: {source:?}\nURL: {url:?}\nParams: {params:?}\nret: {ret:?}");
                ret?
            },
            #[cfg(feature = "commands")]
            Self::CommandOutput {command, stdin} => match stdin {
                Some(stdin) => match stdin.get(url, params)? {
                    Some(stdin) => Some(Cow::Owned(command.output(Some(url), Some(stdin.as_bytes()))?)),
                    None => Some(Cow::Owned(command.output(Some(url), None)?))
                },
                None => Some(Cow::Owned(command.output(Some(url), None)?))
            },
            Self::Error => Err(StringSourceError::ExplicitError)?
        })
    }
}
