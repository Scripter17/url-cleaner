//! The logic for how to modify a URL.

use serde::{Serialize, Deserialize};
use thiserror::Error;
use url::{Url, ParseError};
use std::str::Utf8Error;
use std::collections::hash_set::HashSet;

use std::borrow::Cow;

// Used internally by the `url` crate to handle query manipulation.
// Imported here for faster allow/remove query parts mappers.
use form_urlencoded;

#[cfg(feature = "http")]
use reqwest::{self, Error as ReqwestError};
#[cfg(not(feature = "http"))]
/// A dummy and empty [`reqwest::Error`].
/// Only exists when the `http` feature is disabled.
#[derive(Debug, Error)]
#[error("A dummy reqwest::Error; Only exists because URL Cleaner was compiled without the http feature.")]
pub struct ReqwestError;

#[cfg(feature = "cache-redirects")]
use std::{
    path::Path,
    io::{self, BufRead, Write, Error as IoError},
    fs::{OpenOptions, File}
};
#[cfg(not(feature = "cache-redirects"))]
use std::io::Error as IoError;

use crate::glue;
use crate::types;

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
    /// Prints debugging information about the contained mapper and its effect on the URL to STDERR.
    /// Intended primarily for debugging logic errors.
    /// *Can* be used in production as in both bash and batch `x|y` only pipes `x`'s STDOUT, but it'll look ugly.
    /// # Errors
    /// If the contained mapper returns an error, that error is returned after the debug info is printed.
    Debug(Box<Mapper>),

    // Error handling.

    /// Ignores any error the contained mapper may return.
    IgnoreError(Box<Mapper>),
    /// If the `try` mapper returns an error, the `else` mapper is used instead.
    /// # Errors
    /// If the `else` mapper returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::mappers::*;
    /// # use url::Url;
    /// assert!(Mapper::TryCatch {r#try: Box::new(Mapper::None ), catch: Box::new(Mapper::None )}.apply(&mut Url::parse("https://www.example.com").unwrap()).is_ok ());
    /// assert!(Mapper::TryCatch {r#try: Box::new(Mapper::None ), catch: Box::new(Mapper::Error)}.apply(&mut Url::parse("https://www.example.com").unwrap()).is_ok ());
    /// assert!(Mapper::TryCatch {r#try: Box::new(Mapper::Error), catch: Box::new(Mapper::None )}.apply(&mut Url::parse("https://www.example.com").unwrap()).is_ok ());
    /// assert!(Mapper::TryCatch {r#try: Box::new(Mapper::Error), catch: Box::new(Mapper::Error)}.apply(&mut Url::parse("https://www.example.com").unwrap()).is_err());
    /// ```
    TryCatch {
        /// The mapper to try first.
        r#try: Box<Mapper>,
        /// If the try mapper fails, instead return the result of this one.
        catch: Box<Mapper>
    },
    /// Applies the contained mappers in order.
    /// # Errors
    /// If one of the contained mappers returns an error, the URL is left unchanged and the error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::mappers::*;
    /// # use url::Url;
    /// let mut url=Url::parse("https://www.example.com").unwrap();
    /// assert!(Mapper::All(vec![Mapper::SetHost("2.com".to_string()), Mapper::Error]).apply(&mut url).is_err());
    /// assert_eq!(url.domain(), Some("www.example.com"));
    /// ```

    // Multiple.

    All(Vec<Mapper>),
    /// Applies the contained mappers in order. If an error occurs that error is returned and the URL is left unchanged.
    /// Technically the name is wrong as [`super::conditions::Condition::All`] only actually changes the URL after all the mappers pass, but this is conceptually simpler.
    /// # Errors
    /// If one of the contained mappers returns an error, the URL is left as whatever the previous contained mapper set it to and the error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::mappers::*;
    /// # use url::Url;
    /// let mut url=Url::parse("https://www.example.com").unwrap();
    /// assert!(Mapper::AllNoRevert(vec![Mapper::SetHost("3.com".to_string()), Mapper::Error, Mapper::SetHost("4.com".to_string())]).apply(&mut url).is_err());
    /// assert_eq!(url.domain(), Some("3.com"));
    /// ```
    AllNoRevert(Vec<Mapper>),
    /// If one of the contained mappers returns an error, that error is returned and subsequent mappers are still applied.
    /// This is equivalent to wrapping every contained mapper in a [`Mapper::IgnoreError`].
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::mappers::*;
    /// # use url::Url;
    /// let mut url=Url::parse("https://www.example.com").unwrap();
    /// assert!(Mapper::AllIgnoreError(vec![Mapper::SetHost("5.com".to_string()), Mapper::Error, Mapper::SetHost("6.com".to_string())]).apply(&mut url).is_ok());
    /// assert_eq!(url.domain(), Some("6.com"));
    /// ```
    AllIgnoreError(Vec<Mapper>),
    /// Effectively a [`Mapper::TryCatch`] chain but less ugly.
    /// Useful for when a mapper can be implemented under various different feature configurations.
    /// # Errors
    /// If every contained mapper errors, returns the last error.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::mappers::*;
    /// # use url::Url;
    /// let mut url=Url::parse("https://www.example.com").unwrap();
    /// assert!(Mapper::FirstNotError(vec![Mapper::SetHost("1.com".to_string()), Mapper::SetHost("2.com".to_string())]).apply(&mut url).is_ok());
    /// assert_eq!(url.domain(), Some("1.com"));
    /// assert!(Mapper::FirstNotError(vec![Mapper::SetHost("3.com".to_string()), Mapper::Error                       ]).apply(&mut url).is_ok());
    /// assert_eq!(url.domain(), Some("3.com"));
    /// assert!(Mapper::FirstNotError(vec![Mapper::Error                       , Mapper::SetHost("4.com".to_string())]).apply(&mut url).is_ok());
    /// assert_eq!(url.domain(), Some("4.com"));
    /// assert!(Mapper::FirstNotError(vec![Mapper::Error                       , Mapper::Error                       ]).apply(&mut url).is_err());
    /// assert_eq!(url.domain(), Some("4.com"));
    /// ```
    FirstNotError(Vec<Mapper>),

    // Query.

    /// Removes the URL's entire query.
    /// Useful for websites that only use the query for tracking.
    RemoveQuery,
    /// Removes query parameters whose name is in the specified names.
    /// Useful for websites that append random stuff to shared URLs so the website knows your friend got that link from you.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::mappers::*;
    /// # use url::Url;
    /// # use std::collections::hash_set::HashSet;
    /// let mut url=Url::parse("https://example.com?a=2&b=3&c=4&d=5").unwrap();
    /// assert!(Mapper::RemoveQueryParams(HashSet::from(["a".to_string()])).apply(&mut url).is_ok());
    /// assert_eq!(url.query(), Some("b=3&c=4&d=5"));
    /// assert!(Mapper::RemoveQueryParams(HashSet::from(["b".to_string(), "c".to_string()])).apply(&mut url).is_ok());
    /// assert_eq!(url.query(), Some("d=5"));
    /// assert!(Mapper::RemoveQueryParams(HashSet::from(["d".to_string()])).apply(&mut url).is_ok());
    /// assert_eq!(url.query(), None);
    /// ```
    RemoveQueryParams(HashSet<String>),
    /// Removes query parameters whose name isn't in the specified names.
    /// Useful for websites that keep changing their tracking parameters and you're sick of updating your rule set.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::mappers::*;
    /// # use url::Url;
    /// # use std::collections::hash_set::HashSet;
    /// let mut url=Url::parse("https://example.com?a=2&b=3&c=4&d=5").unwrap();
    /// assert!(Mapper::RemoveQueryParams(HashSet::from(["a".to_string()])).apply(&mut url).is_ok());
    /// ```
    AllowQueryParams(HashSet<String>),
    /// Removes query parameters whose name matches the specified regex.
    /// Useful for parsing AdGuard rules.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::mappers::*;
    /// # use url::Url;
    /// # use url_cleaner::glue::RegexParts;
    /// let mut url=Url::parse("https://example.com?a=2&b=3&c=4&d=5").unwrap();
    /// assert!(Mapper::AllowQueryParamsMatchingRegex(RegexParts::new("a|b|c").unwrap().into()).apply(&mut url).is_ok());
    /// assert_eq!(url.query(), Some("a=2&b=3&c=4"));
    /// assert!(Mapper::AllowQueryParamsMatchingRegex(RegexParts::new("d").unwrap().into()).apply(&mut url).is_ok());
    /// assert_eq!(url.query(), None);
    /// ```
    #[cfg(feature = "regex")]
    RemoveQueryParamsMatchingRegex(glue::RegexWrapper),
    /// Removes query parameters whose name doesn't match the specified regex.
    /// Useful for parsing AdGuard rules.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::mappers::*;
    /// # use url::Url;
    /// # use url_cleaner::glue::RegexParts;
    /// let mut url=Url::parse("https://example.com?a=2&b=3&c=4&d=5").unwrap();
    /// assert!(Mapper::RemoveQueryParamsMatchingRegex(RegexParts::new("a|b|c").unwrap().into()).apply(&mut url).is_ok());
    /// assert_eq!(url.query(), Some("d=5"));
    /// assert!(Mapper::RemoveQueryParamsMatchingRegex(RegexParts::new("d").unwrap().into()).apply(&mut url).is_ok());
    /// assert_eq!(url.query(), None);
    /// ```
    #[cfg(feature = "regex")]
    AllowQueryParamsMatchingRegex(glue::RegexWrapper),
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
    /// If the URL cannot be a base, returms the error [`MapperError::UrlCannotBeABase`].
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::mappers::*;
    /// # use url::Url;
    /// let mut url=Url::parse("https://example.com/0/1/2/3/4/5/6").unwrap();
    /// assert!(Mapper::RemovePathSegments(vec![1,3,5,6]).apply(&mut url).is_ok());
    /// assert_eq!(url.path(), "/0/2/4");
    /// ```
    RemovePathSegments(Vec<usize>),

    // Generic part handling.

    /// Sets the specified URL part to `to`.
    /// # Errors
    /// If `to` is `None` and `part` is [`types::UrlPart::Whole`], [`types::UrlPart::Scheme`], [`types::UrlPart::Username`], or [`types::UrlPart::Path`], returns the error [`types::PartError::PartCannotBeNone`].
    SetPart {
        /// The name of the part to replace.
        part: types::UrlPart,
        /// The value to set the part to.
        value: Option<String>
    },
    /// Modifies the specified part of the URL.
    /// # Errors
    /// If `how` is `types::StringModification::ReplaceAt` and the specified range is either out of bounds or not on UTF-8 boundaries, returns the error [`MapperError::StringError`].
    /// If the modification fails, returns the error [`MapperError::PartError`].
    ModifyPart {
        /// The name of the part to modify.
        part: types::UrlPart,
        /// If the relevant [`Url`] part getter returns [`None`], this decides whether to return a [`super::conditions::ConditionError::UrlPartNotFound`] or pretend it's just an empty string and check that.
        /// Defaults to `true`.
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        /// How exactly to modify the part.
        how: types::StringModification
    },
    /// Copies the part specified by `from` to the part specified by `to`.
    /// # Errors
    /// If the part specified by `from` is None, `none_to_empty_string` is `false`, and the part specified by `to` cannot be `None` (see [`Mapper::SetPart`]), returns the error [`types::PartError::PartCannotBeNone`].
    CopyPart {
        /// The part to get the value from.
        from: types::UrlPart,
        /// If the relevant [`Url`] part getter returns [`None`], this decides whether to return a [`super::conditions::ConditionError::UrlPartNotFound`] or pretend it's just an empty string and check that.
        /// Defaults to `true`.
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        /// The part to set to `from`'s value.
        to: types::UrlPart
    },
    /// Applies a regular expression substitution to the specified URL part.
    /// if `none_to_empty_string` is `false`, then getting the password, host, domain, port, query, or fragment may result in a [`super::conditions::ConditionError::UrlPartNotFound`] error.
    /// Also note that ports are strings because I can't be bothered to handle numbers for just ports.
    /// # Errors
    /// If chosen part's getter returns `None` and `none_to_empty_string` is set to `false`, returns the error [`MapperError::UrlPartNotFound`].
    #[cfg(feature = "regex")]
    RegexSubUrlPart {
        /// The name of the part to modify.
        part: types::UrlPart,
        /// If the relevant [`Url`] part getter returns [`None`], this decides whether to return a [`super::conditions::ConditionError::UrlPartNotFound`] or pretend it's just an empty string and check that.
        /// Defaults to `true`.
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        /// The [`glue::GlobWrapper`] that is used to match and extract parts of the selected part.
        regex: glue::RegexWrapper,
        /// The pattern the extracted parts are put into.
        /// See [`regex::Regex::replace`] for details.
        replace: String
    },

    // Miscelanious.

    /// Sends an HTTP request to the current URL and replaces it with the URL the website responds with.
    /// Useful for link shorteners like `bit.ly` and `t.co`.
    /// # Errors
    /// If URL Cleaner is compiled with the feature `cache-redirects`, the provided URL is found in the cache, but its cached result cannot be parsed as a URL, returns the error [`MapperError::UrlParseError`].
    /// If the [`reqwest::blocking::Client`] is not able to send the HTTP request, returns the error [`MapperError::ReqwestError`].
    /// All errors regarding caching the redirect to disk are ignored. This may change in the future.
    /// This is both because CORS makes this mapper useless and because `reqwest::blocking` does not work on WASM targets.
    /// See [reqwest#891](https://github.com/seanmonstar/reqwest/issues/891) and [reqwest#1068](https://github.com/seanmonstar/reqwest/issues/1068) for details.
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
    ExpandShortLink,
    /// Execute a command and sets the URL to its output. Any argument parameter with the value `"{}"` is replaced with the URL. If the command STDOUT ends in a newline it is stripped.
    /// Useful when what you want to do is really specific and niche.
    /// # Errors
    /// Returns the error [`glue::CommandError`] if the command fails.
    #[cfg(feature = "commands")]
    ReplaceWithCommandOutput(glue::CommandWrapper)
}

const fn get_true() -> bool {true}

/// An enum of all possible errors a [`Mapper`] can return.
#[derive(Error, Debug)]
pub enum MapperError {
    /// The [`Mapper::Error`] mapper always returns this error.
    #[error("The \"Error\" mapper always returns this error.")]
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
    #[error(transparent)]
    ReqwestError(#[from] ReqwestError),
    /// Returned when an I/O error occurs. Currently only applies when Expand301 is set to cache redirects.
    #[error(transparent)]
    IoError(#[from] IoError),
    /// Returned when a part replacement fails.
    #[error(transparent)]
    PartError(#[from] types::PartError),
    /// UTF-8 error.
    #[error(transparent)]
    Utf8Error(#[from] Utf8Error),
    /// The command failed.
    #[cfg(feature = "commands")]
    #[error(transparent)]
    CommandError(#[from] glue::CommandError),
    /// A string operation failed.
    #[error(transparent)]
    StringError(#[from] types::StringError),
    /// The part modification failed.
    #[error(transparent)]
    PartModificationError(#[from] types::PartModificationError),
    /// The URL cannot be a base.
    #[error("The URL cannot be a base.")]
    UrlCannotBeABase
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
    pub fn apply(&self, url: &mut Url) -> Result<(), MapperError> {
        match self {

            // Boolean
            
            Self::All(mappers) => {
                let mut temp_url=url.clone();
                for mapper in mappers {
                    mapper.apply(&mut temp_url)?;
                }
                *url=temp_url;
            },
            Self::AllNoRevert(mappers) => {
                for mapper in mappers {
                    mapper.apply(url)?;
                }
            },
            Self::AllIgnoreError(mappers) => {
                for mapper in mappers {
                    let _=mapper.apply(url);
                }
            },
            Self::FirstNotError(mappers) => {
                let mut error=Ok(());
                for mapper in mappers {
                    error=mapper.apply(url);
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
            #[cfg(feature = "regex")]
            Self::RemoveQueryParamsMatchingRegex(regex) => {
                let new_query=form_urlencoded::Serializer::new(String::new()).extend_pairs(url.query_pairs().filter(|(name, _)| !regex.is_match(name))).finish();
                url.set_query((!new_query.is_empty()).then_some(&new_query));
            },
            #[cfg(feature = "regex")]
            Self::AllowQueryParamsMatchingRegex(regex) => {
                let new_query=form_urlencoded::Serializer::new(String::new()).extend_pairs(url.query_pairs().filter(|(name, _)|  regex.is_match(name))).finish();
                url.set_query((!new_query.is_empty()).then_some(&new_query));
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

            // Generic part handling

            Self::SetPart{part, value} => part.set(url, value.as_deref())?,
            Self::ModifyPart{part, none_to_empty_string, how} => part.modify(url, *none_to_empty_string, how)?,
            Self::CopyPart{from, none_to_empty_string, to} => if *none_to_empty_string {
                #[allow(clippy::unnecessary_to_owned)] // It is necessary.
                to.set(url, Some(&from.get(url).unwrap_or(Cow::Borrowed("")).into_owned()))
            } else {
                to.set(url, from.get(url).map(Cow::into_owned).as_deref())
            }?,
            #[cfg(feature = "regex")]
            Self::RegexSubUrlPart {part, none_to_empty_string, regex, replace} => {
                let old_part_value=part
                    .get(url)
                    .or_else(|| none_to_empty_string.then_some(Cow::Borrowed("")))
                    .ok_or(MapperError::UrlPartNotFound)?;
                #[allow(clippy::unnecessary_to_owned)]
                part.set(url, Some(&regex.replace(&old_part_value, replace).into_owned()))?;
            },

            // Error handling
            
            Self::IgnoreError(mapper) => {let _=mapper.apply(url);},
            Self::TryCatch{r#try, catch} => r#try.apply(url).or_else(|_| catch.apply(url))?,

            // Miscelanious

            #[cfg(all(feature = "http", not(target_family = "wasm")))]
            Self::ExpandShortLink => {
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
                let new_url=reqwest::blocking::Client::new().get(url.to_string()).send()?.url().clone();
                *url=new_url.clone();
                // Intentionally ignore any and all file writing errors.
                #[cfg(feature = "cache-redirects")]
                if let Ok(mut x) = OpenOptions::new().create(true).append(true).open("redirect-cache.txt") {
                    let _=x.write(format!("\n{}\t{}", url.as_str(), new_url.as_str()).as_bytes());
                }
            },
            #[cfg(feature = "commands")]
            Self::ReplaceWithCommandOutput(command) => {*url=command.get_url(url)?;},

            // Testing

            Self::None => {},
            Self::Error => Err(MapperError::ExplicitError)?,
            Self::Debug(mapper) => {
                let url_before_mapper=url.clone();
                let mapper_result=mapper.apply(url);
                eprintln!("=== Debug mapper ===\nMapper: {mapper:?}\nURL before mapper: {url_before_mapper:?}\nMapper return value: {mapper_result:?}\nURL after mapper: {url:?}");
                mapper_result?;
            }
        };
        Ok(())
    }
}
