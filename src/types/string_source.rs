use std::str::FromStr;
use std::convert::Infallible;
use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use url::{Url, ParseError};
use thiserror::Error;
#[cfg(all(feature = "http", not(target_family = "wasm")))]
use reqwest::{Error as ReqwestError, header::{HeaderMap, ToStrError}};

use super::*;
use crate::config::Params;
use crate::glue::*;

/// Allows conditions and mappers to get strings from various sources without requiring different conditions and mappers for each source.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum StringSource {
    /// Always returns the error [`StringSourceError::ExplicitError`].
    /// # Errors
    /// Always returns the error [`StringSourceError::ExplicitError`].
    Error,
    /// Prints debugging information about the contained [`Self`] and the details of its execution to STDERR.
    /// Intended primarily for debugging logic errors.
    /// *Can* be used in production as in both bash and batch `x | y` only pipes `x`'s STDOUT, but you probably shouldn't.
    /// # Errors
    /// If the contained [`Self`] returns an error, that error is returned after the debug info is printed.
    Debug(Box<Self>),
    /// Just a string. The most common variant.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringSource;
    /// # use url::Url;
    /// # use url_cleaner::config::Params;
    /// # use std::borrow::Cow;
    /// let url = Url::parse("https://example.com").unwrap();
    /// assert!(StringSource::String("abc".to_string()).get(&url, &Params::default(), false).is_ok_and(|x| x==Some(Cow::Borrowed("abc"))));
    /// ```
    String(String),
    /// Gets the specified URL part.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringSource;
    /// # use url::Url;
    /// # use url_cleaner::config::Params;
    /// # use std::borrow::Cow;
    /// # use url_cleaner::types::UrlPart;
    /// let url = Url::parse("https://example.com").unwrap();
    /// let params = Params::default();
    /// assert!(StringSource::Part(UrlPart::Domain).get(&url, &Params::default(), false).is_ok_and(|x| x==Some(Cow::Borrowed("example.com"))));
    /// ```
    Part(UrlPart),
    /// Gets the specified variable's value.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringSource;
    /// # use url::Url;
    /// # use url_cleaner::config::Params;
    /// # use std::borrow::Cow;
    /// # use std::collections::HashMap;
    /// let url = Url::parse("https://example.com").unwrap();
    /// let params = Params {vars: HashMap::from_iter([("abc".to_string(), "xyz".to_string())]), ..Params::default()};
    /// assert!(StringSource::Var("abc".to_string()).get(&url, &params, false).is_ok_and(|x| x==Some(Cow::Borrowed("xyz"))));
    /// ```
    Var(String),
    /// If the flag specified by `flag` is set, return the result of `then`. Otherwise return the result of `r#else`.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringSource;
    /// # use url::Url;
    /// # use url_cleaner::config::Params;
    /// # use std::borrow::Cow;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::collections::HashSet;
    /// let url = Url::parse("https://example.com").unwrap();
    /// let params_1 = Params::default();
    /// let params_2 = Params {flags: HashSet::from_iter(["abc".to_string()]), ..Params::default()};
    /// let x = StringSource::IfFlag {flag: "abc".to_string(), then: Box::new(StringSource::String("xyz".to_string())), r#else: Box::new(StringSource::Part(UrlPart::Domain))};
    /// assert!(x.get(&url, &params_1, false).is_ok_and(|x| x==Some(Cow::Borrowed("example.com"))));
    /// assert!(x.get(&url, &params_2, false).is_ok_and(|x| x==Some(Cow::Borrowed("xyz"))));
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
        #[serde(deserialize_with = "box_string_or_struct")]
        source: Box<Self>,
        /// The modification to apply to the string.
        modification: StringModification
    },
    Join {
        sources: Vec<Self>,
        #[serde(default)]
        join: String
    },
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
    HeaderValue {
        name: String,
        #[serde(default, with = "crate::glue::headermap")]
        headers: HeaderMap
    },
    ExtractPart {
        source: Box<Self>,
        part: UrlPart
    },
    #[cfg(all(feature = "http", feature = "regex", not(target_family = "wasm")))]
    ExtractFromPage {
        #[serde(default, with = "crate::glue::headermap")]
        headers: HeaderMap,
        #[serde(deserialize_with = "string_or_struct")]
        regex: RegexWrapper,
        #[serde(deserialize_with = "box_string_or_struct")]
        expand: Box<Self>
    }
}

impl FromStr for StringSource {
    type Err=Infallible;

    /// Simply encase the provided string in a [`StringSource::String`] since it's the most common variant.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::String(s.to_string()))
    }
}

impl TryFrom<&str> for StringSource {
    type Error = <Self as FromStr>::Err;

    /// Why doesn't the standard library do `implt<T: FromStr> TryFrom<&str> for T`?
    fn try_from(s: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        Self::from_str(s)
    }
}

/// An enum of all possible errors a [`StringSource`] can return.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum StringSourceError {
    /// A generic string error.
    #[error(transparent)]
    StringError(#[from] StringError),
    /// Returned by [`StringSource::Modified`].
    #[cfg(feature = "string-modification")]
    #[error(transparent)]
    StringModificationError(#[from] StringModificationError),
    /// Always returned by [`StringSource::Error`].
    #[error("StringSource::Error was used.")]
    ExplicitError,
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
    #[error(transparent)]
    ReqwestError(#[from] ReqwestError),
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
    #[error("The HTTP request response did not contain the requested header.")]
    HeaderNotFound,
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
    #[error(transparent)]
    ToStrError(#[from] ToStrError),
    #[error(transparent)]
    ParseError(#[from] ParseError),
    #[error("The regex pattern did not find any matches.")]
    #[cfg(feature = "regex")]
    PatternNotFound,
    #[error("...")]
    StringSourceIsNone
}

impl StringSource {
    /// Gets the string from the source.
    /// # Errors
    /// See the documentation for [`Self`]'s variants for details.
    pub fn get<'a>(&'a self, url: &'a Url, params: &'a Params, none_to_empty_string: bool) -> Result<Option<Cow<'a, str>>, StringSourceError> {
        #[cfg(feature = "debug")]
        println!("Source: {self:?}");
        let ret = Ok(match self {
            Self::String(x) => Some(Cow::Borrowed(x.as_str())),
            Self::Part(x) => x.get(url, none_to_empty_string),
            Self::Var(x) => params.vars.get(x).map(|x| Cow::Borrowed(x.as_str())),
            Self::IfFlag {flag, then, r#else} => if params.flags.contains(flag) {then.get(url, params, none_to_empty_string)?} else {r#else.get(url, params, none_to_empty_string)?},
            #[cfg(feature = "string-modification")]
            Self::Modified {source, modification} => {
                let x=source.as_ref().get(url, params, none_to_empty_string)?.map(Cow::into_owned);
                if let Some(mut x) = x {
                    modification.apply(&mut x, params)?;
                    Some(Cow::Owned(x))
                } else {
                    x.map(Cow::Owned)
                }
            },
            Self::Join {sources, join} => sources.iter().map(|source| source.get(url, params, none_to_empty_string)).collect::<Result<Option<Vec<_>>, _>>()?.map(|x| Cow::Owned(x.join(join))),
            #[cfg(all(feature = "http", not(target_family = "wasm")))]
            Self::HeaderValue{name, headers} => Some(Cow::Owned(params.http_client()?.get(url.as_str()).headers(headers.clone()).send()?.headers().get(name).ok_or(StringSourceError::HeaderNotFound)?.to_str()?.to_string())),
            Self::ExtractPart{source, part} => source.get(url, params, false)?.map(|x| Url::parse(&x)).transpose()?.and_then(|x| part.get(&x, none_to_empty_string).map(|x| Cow::Owned(x.into_owned()))),
            #[cfg(all(feature = "http", feature = "regex", not(target_family = "wasm")))]
            Self::ExtractFromPage{headers, regex, expand} => if let Some(expand) = expand.get(url, params, false)? {
                let mut ret=String::new();
                regex.captures(&params.http_client()?.get(url.as_str()).headers(headers.clone()).send()?.text()?).ok_or(StringSourceError::PatternNotFound)?.expand(&expand, &mut ret);
                Some(Cow::Owned(ret))
            } else {
                Err(StringSourceError::StringSourceIsNone)?
            },
            Self::Debug(source) => {
                let ret=source.get(url, params, none_to_empty_string);
                eprintln!("=== StringSource::Debug ===\nSource: {source:?}\nURL: {url:?}\nParams: {params:?}\nnone_to_empty_string: {none_to_empty_string:?}\nret: {ret:?}");
                ret?
            },
            Self::Error => Err(StringSourceError::ExplicitError)?
        });
        if none_to_empty_string {
            ret.map(|x| x.or(Some(Cow::Borrowed(""))))
        } else {
            ret
        }
    }
}
