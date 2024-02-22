//! The logic for how to modify a URL.

#[cfg(feature = "cache-redirects")]
use std::{
    path::Path,
    io::{self, BufRead, Write, Error as IoError},
    fs::{OpenOptions, File}
};

use serde::{Serialize, Deserialize};
use thiserror::Error;
use url::{Url, ParseError};
use std::str::Utf8Error;
use std::collections::hash_set::HashSet;
#[cfg(all(feature = "http", not(target_family = "wasm")))]
use reqwest::{self, Error as ReqwestError, header::HeaderMap};

use crate::glue::{self, string_or_struct, optional_string_or_struct};
use crate::types::*;

/// The part of a [`crate::rules::Rule`] that specifies how to modify a [`Url`] if the rule's condition passes.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum Mapper {

    // Testing.

    /// Does nothing.
    None,
    /// Always returns the error [`MapperError::ExplicitError`].
    /// # Errors
    /// Always returns the error [`MapperError::ExplicitError`].
    Error,
    /// Prints debugging information about the contained [`Self`] and the details of its application to STDERR.
    /// Intended primarily for debugging logic errors.
    /// *Can* be used in production as in both bash and batch `x | y` only pipes `x`'s STDOUT, but you probably shouldn't.
    /// # Errors
    /// If the contained [`Self`] returns an error, that error is returned after the debug info is printed.
    Debug(Box<Self>),

    // Error handling.

    /// Ignores any error the contained [`Self`] may return.
    IgnoreError(Box<Self>),
    /// If `try` returns an error, `else` is applied.
    /// If `try` does not return an error, `else` is not applied.
    /// # Errors
    /// If `else` returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::*;
    /// # use url_cleaner::config::Params;
    /// # use url::Url;
    /// assert!(Mapper::TryElse {r#try: Box::new(Mapper::None ), r#else: Box::new(Mapper::None )}.apply(&mut Url::parse("https://www.example.com").unwrap(), &Params::default()).is_ok ());
    /// assert!(Mapper::TryElse {r#try: Box::new(Mapper::None ), r#else: Box::new(Mapper::Error)}.apply(&mut Url::parse("https://www.example.com").unwrap(), &Params::default()).is_ok ());
    /// assert!(Mapper::TryElse {r#try: Box::new(Mapper::Error), r#else: Box::new(Mapper::None )}.apply(&mut Url::parse("https://www.example.com").unwrap(), &Params::default()).is_ok ());
    /// assert!(Mapper::TryElse {r#try: Box::new(Mapper::Error), r#else: Box::new(Mapper::Error)}.apply(&mut Url::parse("https://www.example.com").unwrap(), &Params::default()).is_err());
    /// ```
    TryElse {
        /// The [`Self`] to try first.
        r#try: Box<Self>,
        /// If `try` fails, instead return the result of this one.
        r#else: Box<Self>
    },

    // Multiple.

    /// Applies the contained [`Self`]s in order.
    /// # Errors
    /// If one of the contained [`Self`]s returns an error, the URL is left unchanged and the error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::*;
    /// # use url_cleaner::config::Params;
    /// # use url::Url;
    /// let mut url=Url::parse("https://www.example.com").unwrap();
    /// assert!(Mapper::All(vec![Mapper::SetHost("2.com".to_string()), Mapper::Error]).apply(&mut url, &Params::default()).is_err());
    /// assert_eq!(url.domain(), Some("www.example.com"));
    /// ```
    All(Vec<Self>),
    /// Applies the contained [`Self`]s in order. If an error occurs, the URL remains changed by the previous contained [`Self`]s and the error is returned.
    /// Technically the name is wrong as [`Self::All`] only actually applies the change after all the contained [`Self`] pass, but this is conceptually simpler.
    /// # Errors
    /// If one of the contained [`Self`]s returns an error, the URL is left as whatever the previous contained mapper set it to and the error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::*;
    /// # use url_cleaner::config::Params;
    /// # use url::Url;
    /// let mut url=Url::parse("https://www.example.com").unwrap();
    /// assert!(Mapper::AllNoRevert(vec![Mapper::SetHost("3.com".to_string()), Mapper::Error, Mapper::SetHost("4.com".to_string())]).apply(&mut url, &Params::default()).is_err());
    /// assert_eq!(url.domain(), Some("3.com"));
    /// ```
    AllNoRevert(Vec<Self>),
    /// If any of the contained [`Self`]s returns an error, the error is ignored and subsequent [`Self`]s are still applied.
    /// This is equivalent to wrapping every contained [`Self`] in a [`Self::IgnoreError`].
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::*;
    /// # use url_cleaner::config::Params;
    /// # use url::Url;
    /// let mut url=Url::parse("https://www.example.com").unwrap();
    /// assert!(Mapper::AllIgnoreError(vec![Mapper::SetHost("5.com".to_string()), Mapper::Error, Mapper::SetHost("6.com".to_string())]).apply(&mut url, &Params::default()).is_ok());
    /// assert_eq!(url.domain(), Some("6.com"));
    /// ```
    AllIgnoreError(Vec<Self>),
    /// Effectively a [`Self::TryElse`] chain but less ugly.
    /// # Errors
    /// If every contained [`Self`] errors, returns the last error.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::*;
    /// # use url_cleaner::config::Params;
    /// # use url::Url;
    /// let mut url=Url::parse("https://www.example.com").unwrap();
    /// assert!(Mapper::FirstNotError(vec![Mapper::SetHost("1.com".to_string()), Mapper::SetHost("2.com".to_string())]).apply(&mut url, &Params::default()).is_ok());
    /// assert_eq!(url.domain(), Some("1.com"));
    /// assert!(Mapper::FirstNotError(vec![Mapper::SetHost("3.com".to_string()), Mapper::Error                       ]).apply(&mut url, &Params::default()).is_ok());
    /// assert_eq!(url.domain(), Some("3.com"));
    /// assert!(Mapper::FirstNotError(vec![Mapper::Error                       , Mapper::SetHost("4.com".to_string())]).apply(&mut url, &Params::default()).is_ok());
    /// assert_eq!(url.domain(), Some("4.com"));
    /// assert!(Mapper::FirstNotError(vec![Mapper::Error                       , Mapper::Error                       ]).apply(&mut url, &Params::default()).is_err());
    /// assert_eq!(url.domain(), Some("4.com"));
    /// ```
    FirstNotError(Vec<Self>),

    // Query.

    /// Removes the URL's entire query.
    /// Useful for websites that only use the query for tracking.
    RemoveQuery,
    /// Removes all query parameters whose name exists in the specified [`std::collections::HashMap`].
    /// Useful for websites that append random stuff to shared URLs so the website knows your friend got that link from you.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::*;
    /// # use url_cleaner::config::Params;
    /// # use url::Url;
    /// # use std::collections::hash_set::HashSet;
    /// let mut url=Url::parse("https://example.com?a=2&b=3&c=4&d=5").unwrap();
    /// assert!(Mapper::RemoveQueryParams(HashSet::from(["a".to_string()])).apply(&mut url, &Params::default()).is_ok());
    /// assert_eq!(url.query(), Some("b=3&c=4&d=5"));
    /// assert!(Mapper::RemoveQueryParams(HashSet::from(["b".to_string(), "c".to_string()])).apply(&mut url, &Params::default()).is_ok());
    /// assert_eq!(url.query(), Some("d=5"));
    /// assert!(Mapper::RemoveQueryParams(HashSet::from(["d".to_string()])).apply(&mut url, &Params::default()).is_ok());
    /// assert_eq!(url.query(), None);
    /// ```
    RemoveQueryParams(HashSet<String>),
    /// Keeps only the query parameters whose name exists in the specified [`HashSet`].
    /// Useful for websites that keep changing their tracking parameters and you're sick of updating your rule set.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::*;
    /// # use url_cleaner::config::Params;
    /// # use url::Url;
    /// # use std::collections::hash_set::HashSet;
    /// let mut url=Url::parse("https://example.com?a=2&b=3&c=4&d=5").unwrap();
    /// assert!(Mapper::RemoveQueryParams(HashSet::from(["a".to_string()])).apply(&mut url, &Params::default()).is_ok());
    /// ```
    AllowQueryParams(HashSet<String>),
    /// Removes all query parameters whose name matches the specified [`StringMatcher`].
    /// # Errors
    /// If the call to [`StringMatcher::satisfied_by`] returns an error, that error is returned.
    #[cfg(feature = "string-matcher")]
    RemoveQueryParamsMatching(StringMatcher),
    /// Keeps only the query parameters whose name matches the specified [`StringMatcher`].
    /// # Errors
    /// If the call to [`StringMatcher::satisfied_by`] returns an error, that error is returned.
    #[cfg(feature = "string-matcher")]
    AllowQueryParamsMatching(StringMatcher),
    /// Replace the current URL with the value of the specified query parameter.
    /// Useful for websites for have a "are you sure you want to leave?" page with a URL like `https://example.com/outgoing?to=https://example.com`.
    /// # Errors
    /// If the specified query parameter cannot be found, returns the error [`MapperError::CannotFindQueryParam`].
    /// If the query parameter is found but its value cannot be parsed as a URL, returns the error [`MapperError::UrlParseError`].
    GetUrlFromQueryParam(String),
    /// Replace the current URL's path with the value of the specified query parameter.
    /// Useful for websites that have a "you must log in to see this page" page.
    /// # Errors
    /// If the specified query parameter cannot be found, returns the error [`MapperError::CannotFindQueryParam`].
    GetPathFromQueryParam(String),

    // Other parts.

    /// Replaces the URL's host to the provided host.
    /// Useful for websites that are just a wrapper around another website. For example, `vxtwitter.com`.
    /// # Errors
    /// If the resulting string cannot be parsed as a URL, returns the error [`MapperError::UrlParseError`].
    /// See [`Url::set_host`] for details.
    SetHost(String),
    /// Removes the path segments with an index in the specified list.
    /// See [`Url::path_segments`] for details.
    /// # Errors
    /// If the URL cannot be a base, returns the error [`MapperError::UrlCannotBeABase`].
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::*;
    /// # use url_cleaner::config::Params;
    /// # use url::Url;
    /// let mut url=Url::parse("https://example.com/0/1/2/3/4/5/6").unwrap();
    /// assert!(Mapper::RemovePathSegments(vec![1,3,5,6,8]).apply(&mut url, &Params::default()).is_ok());
    /// assert_eq!(url.path(), "/0/2/4");
    /// ```
    RemovePathSegments(Vec<usize>),
    /// [`Url::join`].
    #[cfg(feature = "string-source")]
    Join(#[serde(deserialize_with = "string_or_struct")] StringSource),
    #[cfg(not(feature = "string-source"))]
    Join(String),

    // Generic part handling.

    /// Sets the specified URL part to `to`.
    /// # Errors
    /// If `to` is `None` and `part` cannot be `None` (see [`UrlPart`] for details), returns the error [`types::PartError::PartCannotBeNone`].
    #[cfg(feature = "string-source")]
    SetPart {
        /// The name of the part to replace.
        part: UrlPart,
        /// If the relevant [`Url`] part getter returns [`None`], this decides whether to return a [`super::conditions::ConditionError::UrlPartNotFound`] or pretend it's just an empty string and check that.
        /// Defaults to `true`.
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        /// The value to set the part to.
        #[serde(deserialize_with = "optional_string_or_struct")]
        value: Option<StringSource>
    },
    #[cfg(not(feature = "string-source"))]
    SetPart {
        /// The name of the part to replace.
        part: UrlPart,
        /// If the relevant [`Url`] part getter returns [`None`], this decides whether to return a [`super::conditions::ConditionError::UrlPartNotFound`] or pretend it's just an empty string and check that.
        /// Defaults to `true`.
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        /// The value to set the part to.
        value: Option<String>
    },
    /// Modifies the specified part of the URL.
    /// # Errors
    /// If `how` is `StringModification::ReplaceAt` and the specified range is either out of bounds or not on UTF-8 boundaries, returns the error [`MapperError::StringError`].
    /// If the modification fails, returns the error [`MapperError::PartError`].
    #[cfg(feature = "string-modification")]
    ModifyPart {
        /// The name of the part to modify.
        part: UrlPart,
        /// If the relevant [`Url`] part getter returns [`None`], this decides whether to return a [`super::conditions::ConditionError::UrlPartNotFound`] or pretend it's just an empty string and check that.
        /// Defaults to `true`.
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        /// How exactly to modify the part.
        how: StringModification
    },
    /// Copies the part specified by `from` to the part specified by `to`.
    /// # Errors
    /// If the part specified by `from` is None, `none_to_empty_string` is `false`, and the part specified by `to` cannot be `None` (see [`Mapper::SetPart`]), returns the error [`types::PartError::PartCannotBeNone`].
    CopyPart {
        /// The part to get the value from.
        from: UrlPart,
        /// If the relevant [`Url`] part getter returns [`None`], this decides whether to return a [`super::conditions::ConditionError::UrlPartNotFound`] or pretend it's just an empty string and check that.
        /// Defaults to `true`.
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        /// The part to set to `from`'s value.
        to: UrlPart
    },   
    /// Applies a regular expression substitution to the specified URL part.
    /// if `none_to_empty_string` is `false`, then getting the password, host, domain, port, query, or fragment may result in a [`super::conditions::ConditionError::UrlPartNotFound`] error.
    /// Also note that ports are strings because I can't be bothered to handle numbers for just ports.
    /// # Errors
    /// If chosen part's getter returns `None` and `none_to_empty_string` is set to `false`, returns the error [`MapperError::UrlPartNotFound`].
    #[cfg(all(feature = "regex", feature = "string-source"))]
    RegexSubUrlPart {
        /// The name of the part to modify.
        part: UrlPart,
        /// If the relevant [`Url`] part getter returns [`None`], this decides whether to return a [`super::conditions::ConditionError::UrlPartNotFound`] or pretend it's just an empty string and check that.
        /// Defaults to `true`.
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        /// The regex that is used to match and extract parts of the selected part.
        #[serde(deserialize_with = "string_or_struct")]
        regex: glue::RegexWrapper,
        /// The pattern the extracted parts are put into.
        /// See [`regex::Regex::replace`] for details.
        #[serde(deserialize_with = "string_or_struct", default = "eufp_expand")]
        replace: StringSource
    },
    #[cfg(all(feature = "regex", not(feature = "string-source")))]
    RegexSubUrlPart {
        /// The name of the part to modify.
        part: UrlPart,
        /// If the relevant [`Url`] part getter returns [`None`], this decides whether to return a [`super::conditions::ConditionError::UrlPartNotFound`] or pretend it's just an empty string and check that.
        /// Defaults to `true`.
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        /// The regex that is used to match and extract parts of the selected part.
        #[serde(deserialize_with = "string_or_struct")]
        regex: glue::RegexWrapper,
        /// The pattern the extracted parts are put into.
        /// See [`regex::Regex::replace`] for details.
        #[serde(default = "efup_expand")]
        replace: String
    },

    // Miscellaneous.

    /// Sends an HTTP request to the current URL and replaces it with the URL the website responds with.
    /// Useful for link shorteners like `bit.ly` and `t.co`.
    /// # Errors
    /// If URL Cleaner is compiled with the feature `cache-redirects`, the provided URL is found in the cache, but its cached result cannot be parsed as a URL, returns the error [`MapperError::UrlParseError`].
    /// If the [`reqwest::blocking::Client`] is not able to send the HTTP request, returns the error [`MapperError::ReqwestError`].
    /// All errors regarding caching the redirect to disk are ignored. This may change in the future.
    /// This is both because CORS makes this mapper useless and because `reqwest::blocking` does not work on WASM targets.
    /// See [reqwest#891](https://github.com/seanmonstar/reqwest/issues/891) and [reqwest#1068](https://github.com/seanmonstar/reqwest/issues/1068) for details.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::Mapper;
    /// # use url_cleaner::config::Params;
    /// # use url::Url;
    /// # use reqwest::header::HeaderMap;
    /// let mut url = Url::parse("https://t.co/H8IF8DHSFL").unwrap();
    /// assert!(Mapper::ExpandShortLink{headers: HeaderMap::default()}.apply(&mut url, &Params::default()).is_ok());
    /// assert_eq!(url.as_str(), "https://www.eff.org/deeplinks/2024/01/eff-and-access-now-submission-un-expert-anti-lgbtq-repression");
    /// ```
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
    ExpandShortLink {
        /// The headers to send alongside the param's default headers.
        #[serde(default, with = "crate::glue::headermap")]
        headers: HeaderMap
    },
    /// Gets the URL as a webpage, uses `regex` to find a URL, and uses `expand` to join the regex capture's groups.
    #[cfg(all(feature = "http", feature = "regex", not(target_family = "wasm"), feature = "string-source"))]
    ExtractUrlFromPage {
        /// The headers to send alongside the param's default headers.
        #[serde(default, with = "crate::glue::headermap")]
        headers: HeaderMap,
        /// The pattern to search for in the page.
        #[serde(deserialize_with = "string_or_struct")]
        regex: glue::RegexWrapper,
        /// Used for [`regex::Captures::expand`].
        /// Defaults to `"$1"`.
        #[serde(deserialize_with = "string_or_struct", default = "eufp_expand")]
        expand: StringSource
    },
    #[cfg(all(feature = "http", feature = "regex", not(target_family = "wasm"), not(feature = "string-source")))]
    ExtractUrlFromPage {
        /// The headers to send alongside the param's default headers.
        #[serde(default, with = "crate::glue::headermap")]
        headers: HeaderMap,
        /// The pattern to search for in the page.
        #[serde(deserialize_with = "string_or_struct")]
        regex: glue::RegexWrapper,
        /// Used for [`regex::Captures::expand`].
        /// Defaults to `"$1"`.
        #[serde(default = "efup_expand")]
        expand: String
    },
    /// Execute a command and sets the URL to its output. Any argument parameter with the value `"{}"` is replaced with the URL. If the command STDOUT ends in a newline it is stripped.
    /// Useful when what you want to do is really specific and niche.
    /// # Errors
    /// Returns the error [`glue::CommandError`] if the command fails.
    #[cfg(feature = "commands")]
    ReplaceWithCommandOutput(glue::CommandWrapper)
}

const fn get_true() -> bool {true}
#[cfg(all(feature = "regex", feature = "string-source"))]
fn eufp_expand() -> StringSource {StringSource::String("$1".to_string())}
#[cfg(all(feature = "regex", not(feature = "string-source")))]
fn eufp_expand() -> String {"$1".to_string()}

/// An enum of all possible errors a [`Mapper`] can return.
#[derive(Error, Debug)]
pub enum MapperError {
    /// The [`Mapper::Error`] mapper always returns this error.
    #[error("Mapper::Error was used.")]
    ExplicitError,
    /// Returned when the mapper has `none_to_empty_string` set to `false` and the requested part of the provided URL is `None`.
    #[error("The provided URL does not have the requested part.")]
    UrlPartNotFound,
    /// Returned when the provided URL's query does not contain a query parameter with the requested name.
    #[error("The URL provided does not contain the query parameter required.")]
    CannotFindQueryParam,
    /// Returned when the would-be new URL could not be parsed by [`url::Url`].
    #[error(transparent)]
    UrlParseError(#[from] ParseError),
    /// Returned when an HTTP request fails. Currently only applies to the Expand301 mapper.
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
    #[error(transparent)]
    ReqwestError(#[from] ReqwestError),
    /// Returned when an I/O error occurs. Currently only applies when Expand301 is set to cache redirects.
    #[cfg(feature = "cache-redirects")]
    #[error(transparent)]
    IoError(#[from] IoError),
    /// UTF-8 error.
    #[error(transparent)]
    Utf8Error(#[from] Utf8Error),
    /// The command failed.
    #[cfg(feature = "commands")]
    #[error(transparent)]
    CommandError(#[from] glue::CommandError),
    /// A string operation failed.
    #[error(transparent)]
    StringError(#[from] StringError),
    /// The part modification failed.
    #[cfg(feature = "string-modification")]
    #[error(transparent)]
    PartModificationError(#[from] PartModificationError),
    #[error(transparent)]
    SetPartError(#[from] SetPartError),
    /// The URL cannot be a base.
    #[error("The URL cannot be a base.")]
    UrlCannotBeABase,
    /// An instance of the regex pattern could not be found in the page returned by the URL.
    #[error("An instance of the regex pattern could not be found in the page returned by the URL.")]
    PatternNotFound,
    /// A variable was requested but not found at runtime.
    #[error("A variable was requested but not found at runtime.")]
    VarNotFound,
    /// The specified [`StringSource`] returned `None`.
    #[error("The specified StringSource returned None.")]
    StringSourceIsNone,
    /// The call to [`StringMatcher::satisfied_by`] returned an error.
    #[cfg(feature = "string-matcher")]
    #[error(transparent)]
    StringMatcherError(#[from] StringMatcherError),
    /// The call to [`StringSource::get`] returned an error.
    #[cfg(feature = "string-source")]
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
    /// The call to [`StringModification::apply`] returned an error.
    #[cfg(feature = "string-modification")]
    #[error(transparent)]
    StringModificationError(#[from] StringModificationError)
}

#[cfg(feature = "cache-redirects")]
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

impl Mapper {
    /// Applies the mapper to the provided URL.
    /// Does not check with a [`crate::rules::conditions::Condition`]. You should do that yourself or use [`crate::rules::Rule`].
    /// # Errors
    /// If the mapper has an error, that error is returned.
    /// See [`Mapper`]'s documentation for details.
    pub fn apply(&self, url: &mut Url, params: &crate::config::Params) -> Result<(), MapperError> {
        #[cfg(feature = "debug")]
        println!("Mapper: {self:?}");
        match self {

            // Boolean

            Self::All(mappers) => {
                let mut temp_url=url.clone();
                for mapper in mappers {
                    mapper.apply(&mut temp_url, params)?;
                }
                *url=temp_url;
            },
            Self::AllNoRevert(mappers) => {
                for mapper in mappers {
                    mapper.apply(url, params)?;
                }
            },
            Self::AllIgnoreError(mappers) => {
                for mapper in mappers {
                    let _=mapper.apply(url, params);
                }
            },
            Self::FirstNotError(mappers) => {
                let mut error=Ok(());
                for mapper in mappers {
                    error=mapper.apply(url, params);
                    if error.is_ok() {break}
                }
                error?
            },

            // Query

            Self::RemoveQuery => url.set_query(None),
            Self::RemoveQueryParams(names) => {
                let new_query=form_urlencoded::Serializer::new(String::new()).extend_pairs(url.query_pairs().filter(|(name, _)| !names.contains(name.as_ref()))).finish();
                url.set_query((!new_query.is_empty()).then_some(&new_query));
            },
            Self::AllowQueryParams(names) => {
                let new_query=form_urlencoded::Serializer::new(String::new()).extend_pairs(url.query_pairs().filter(|(name, _)|  names.contains(name.as_ref()))).finish();
                url.set_query((!new_query.is_empty()).then_some(&new_query));
            },
            #[cfg(feature = "string-matcher")]
            Self::RemoveQueryParamsMatching(matcher) => {
                let mut new_query=form_urlencoded::Serializer::new(String::new());
                for (name, value) in url.query_pairs() {
                    if !matcher.satisfied_by(&name, params)? {
                        new_query.append_pair(&name, &value);
                    }
                }
                let x = new_query.finish();
                url.set_query((!x.is_empty()).then_some(&x));
            },
            #[cfg(feature = "string-matcher")]
            Self::AllowQueryParamsMatching(matcher) => {
                let mut new_query=form_urlencoded::Serializer::new(String::new());
                for (name, value) in url.query_pairs() {
                    if matcher.satisfied_by(&name, params)? {
                        new_query.append_pair(&name, &value);
                    }
                }
                let x = new_query.finish();
                url.set_query((!x.is_empty()).then_some(&x));
            },
            Self::GetUrlFromQueryParam(name) => {
                match url.query_pairs().find(|(param_name, _)| param_name==name) {
                    Some((_, new_url)) => {*url=Url::parse(&new_url)?},
                    None => Err(MapperError::CannotFindQueryParam)?
                }
            },
            Self::GetPathFromQueryParam(name) => {
                match url.query_pairs().find(|(param_name, _)| param_name==name) {
                    Some((_, new_path)) => {#[allow(clippy::unnecessary_to_owned)] url.set_path(&new_path.into_owned());},
                    None => Err(MapperError::CannotFindQueryParam)?
                }
            },

            // Other parts

            Self::SetHost(new_host) => url.set_host(Some(new_host))?,
            Self::RemovePathSegments(indices) => url.set_path(&url.path_segments().ok_or(MapperError::UrlCannotBeABase)?.enumerate().filter_map(|(i, x)| (!indices.contains(&i)).then_some(x)).collect::<Vec<_>>().join("/")),
            #[cfg(feature = "string-source")]
            Self::Join(with) => if let Some(value) = with.get(url, params, false)? {
                *url=url.join(&value)?;
            } else {
                Err(MapperError::StringSourceIsNone)?
            },
            #[cfg(not(feature = "string-source"))]
            Self::Join(with) => *url=url.join(with)?,

            // Generic part handling

            #[cfg(feature = "string-source")]
            Self::SetPart{part, value, none_to_empty_string} => match value.as_ref() {
                Some(source) => {
                    let temp=source.get(url, params, *none_to_empty_string)?.map(|x| x.into_owned());
                    part.set(url, temp.as_deref())
                },
                None => part.set(url, None)
            }?,
            #[cfg(not(feature = "string-source"))]
            Self::SetPart{part, value, none_to_empty_string} => part.set(url, value.as_deref())?,
            #[cfg(feature = "string-modification")]
            Self::ModifyPart{part, none_to_empty_string, how} => part.modify(url, *none_to_empty_string, how, params)?,
            Self::CopyPart{from, none_to_empty_string, to} => to.set(url, from.get(url, *none_to_empty_string).map(|x| x.into_owned()).as_deref())?,
            #[cfg(feature = "regex")]
            Self::RegexSubUrlPart {part, none_to_empty_string, regex, replace} => {
                let old_part_value=part.get(url, *none_to_empty_string).ok_or(MapperError::UrlPartNotFound)?;
                #[allow(clippy::unnecessary_to_owned)]
                part.set(url, Some(&regex.replace(&old_part_value, replace.get(url, params, false)?.ok_or(MapperError::StringSourceIsNone)?).into_owned()))?;
            },

            // Error handling

            Self::IgnoreError(mapper) => {let _=mapper.apply(url, params);},
            Self::TryElse{r#try, r#else} => r#try.apply(url, params).or_else(|_| r#else.apply(url, params))?,

            // Miscellaneous

            #[cfg(all(feature = "http", not(target_family = "wasm")))]
            Self::ExpandShortLink{headers} => {
                #[cfg(feature = "cache-redirects")]
                if let Ok(lines) = read_lines("redirect-cache.txt") {
                    for line in lines.map_while(Result::ok) {
                        if let Some((short, long)) = line.split_once('\t') {
                            if url.as_str()==short {
                                *url=Url::parse(long)?;
                                return Ok(());
                            }
                        }
                    }
                }
                let new_url=params.http_client()?.get(url.as_str()).headers(headers.clone()).send()?.url().clone();
                // Intentionally ignore any and all file writing errors.
                #[cfg(feature = "cache-redirects")]
                if let Ok(mut x) = OpenOptions::new().create(true).append(true).open("redirect-cache.txt") {
                    let _=x.write(format!("\n{}\t{}", url.as_str(), new_url.as_str()).as_bytes());
                }
                *url=new_url.clone();
            },
            #[cfg(all(feature = "http", feature = "regex", not(target_family = "wasm")))]
            Self::ExtractUrlFromPage{headers, regex, expand} => if let Some(expand) = expand.get(url, params, false)? {
                let mut ret = String::new();
                regex.captures(&params.http_client()?.get(url.as_str()).headers(headers.clone()).send()?.text()?).ok_or(MapperError::PatternNotFound)?.expand(&expand, &mut ret);
                *url=Url::parse(&ret)?;
            } else {
                Err(MapperError::StringSourceIsNone)?
            },
            #[cfg(feature = "commands")]
            Self::ReplaceWithCommandOutput(command) => {*url=command.get_url(Some(url))?;},

            // Testing

            Self::None => {},
            Self::Error => Err(MapperError::ExplicitError)?,
            Self::Debug(mapper) => {
                let url_before_mapper=url.clone();
                let mapper_result=mapper.apply(url, params);
                eprintln!("=== Mapper::Debug ===\nMapper: {mapper:?}\nParams: {params:?}\nURL before mapper: {url_before_mapper:?}\nMapper return value: {mapper_result:?}\nURL after mapper: {url:?}");
                mapper_result?;
            }
        };
        Ok(())
    }
}
