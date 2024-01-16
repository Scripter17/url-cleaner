//! The logic for how to modify a URL.

use serde::{Serialize, Deserialize};
use thiserror::Error;
use url::{Url, ParseError};

// Used only for [`RegexWrapper::replace`].
#[cfg(feature = "regex")]
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
/// A dummy [`std::io::Error`].
/// Only exists when the `cache-redirects` feature is disabled.
#[derive(Debug, Error)]
#[error("A dummy std::io::Error; Only exists because URL Cleaner was compiled without the cache-redirects feature.")]
pub struct IoError;

#[cfg(feature = "commands")]
use std::str::Utf8Error;
#[cfg(not(feature = "commands"))]
/// A dummy [`std::str::Utf8Error`].
/// Only exists when the `commands` feature is disabled.
#[derive(Debug, Error)]
#[error("A dummy std::str::Utf8Error; Only exists because URL Cleaner was compiled without the commands feature.")]
pub struct Utf8Error;

use crate::glue;
use crate::types;

/// The part of a [`crate::rules::Rule`] that specifies how to modify a [`Url`] if the rule's condition passes.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Mapper {
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
    /// Ignores any error the contained mapper may return.
    IgnoreError(Box<Mapper>),
    /// If the `try` mapper reuterns an error, the `else` mapper is used instead.
    /// # Errors
    /// If the `else` mapper returns an error, that error is returned.
    TryCatch {
        /// The mapper to try first.
        r#try: Box<Mapper>,
        /// If the try mapper fails, instead return the result of this one.
        catch: Box<Mapper>
    },
    /// Applies the contained mappers in order.
    /// # Errors
    /// If one of the contained mappers returns an error, the URL is left unchanged and the error is returned.
    All(Vec<Mapper>),
    /// Applies the contained mappers in order. If an error occurs that error is returned and the URL is left unchanged.
    /// Technically the name is wrong as [`super::conditions::Condition::All`] only actually changes the URL after all the mappers pass, but this is conceptually simpler.
    /// # Errors
    /// If one of the contained mappers returns an error, the URL is left as whatever the previous contained mapper set it to and the error is returned.
    AllNoRevert(Vec<Mapper>),
    /// If one of the contained mappers returns an error, that error is returned and sebsequent mappers are still applied.
    /// This is equivalent to wrapping every contained mapper in a [`Mapper::IgnoreError`].
    AllIgnoreError(Vec<Mapper>),
    /// Removes the URL's entire query.
    /// Useful for webites that only use the query for tracking.
    RemoveQuery,
    /// Removes query paramaters whose name is in the specified names.
    /// Useful for websites that append random stuff to shared URLs so the website knows your friend got that link from you.
    RemoveQueryParams(Vec<String>),
    /// Removes query paramaters whose name isn't in the specified names.
    /// Useful for websites that keep changing their tracking paramaters and you're sick of updating your rule set.
    AllowQueryParams(Vec<String>),
    /// Removes query paramaters whose name matches the specified regex.
    /// Useful for parsing AdGuard rules.
    /// # Errors
    /// Returns the error [`MapperError::MapperDisabled`] if URL Cleaner is compiled without the `regex` feature.
    RemoveQueryParamsMatchingRegex(glue::RegexWrapper),
    /// Removes query paramaters whose name doesn't match the specified regex.
    /// Useful for parsing AdGuard rules.
    /// # Errors
    /// Returns the error [`MapperError::MapperDisabled`] if URL Cleaner is compiled without the `regex` feature.
    AllowQueryParamsMatchingRegex(glue::RegexWrapper),
    /// Replace the current URL with the value of the specified query paramater.
    /// Useful for websites for have a "are you sure you want to leave?" page with a URL like `https://example.com/outgoing?to=https://example.com`.
    /// # Errors
    /// If the specified query paramater cannot be found, returns the error [`MapperError::CannotFindQueryParam`].
    /// If the query paramater is found but its value cannot be parsed as a URL, returns the error [`MapperError::UrlParseError`].
    GetUrlFromQueryParam(String),
    /// Replace the current URL's path with the value of the specified query paramater.
    /// Useful for websites that have a "you must log in to see this page" page.
    /// # Errors
    /// If the specified query paramater cannot be found, returns the error [`MapperError::CannotFindQueryParam`].
    GetPathFromQueryParam(String),
    /// Replaces the URL's host to the provided host.
    /// Useful for websites that are just a wrapper around another website. For example, `vxtwitter.com`.
    /// # Errors
    /// If the resulting string cannot be parsed as a URL, returns the error [`MapperError::UrlParseError`].
    /// See [`Url::set_host`] for details.
    SetHost(String),
    /// Modifies the specified part of the URL.
    /// # Errors
    /// If `how` is `types::StringModification::ReplaceAt` and the specified range is either out of bounds or not on UTF-8 boundaries, returns the error [`MapperError::StringModificationErrorr`].
    /// If the modification fails, returns the error [`MapperError::ReplaceError`].
    ModifyUrlPart {
        /// The name of the part to modify.
        part: types::UrlPart,
        /// If the relevant [`Url`] part getter returns [`None`], this decides whether to return a [`super::conditions::ConditionError::UrlPartNotFound`] or pretend it's just an empty string and check that.
        /// Defaults to `true`.
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        /// How exactly to modify the part.
        how: types::StringModification
    },
    /// Removes the path segments with an index in the specified list.
    /// For most URLs the indices seems one-indexed as the path starts with a `"/"`.
    /// See [`Url::path`] for details.
    RemovePathSegments(Vec<usize>),
    /// Sends an HTTP request to the current URL and replaces it with the URL the website responds with.
    /// Useful for link shorteners like `bit.ly` and `t.co`.
    /// # Errors
    /// If URL Cleaner is compiled with the feature `cache-redirects`, the provided URL is found in the cache, but its cached result cannot be parsed as a URL, returns the error [`MapperError::UrlParseError`].
    /// If the [`reqwest::blocking::Client`] is not able to send the HTTP request, returns the error [`MapperError::ReqwestError`].
    /// All errors regarding caching the redirect to disk are ignored. This may change in the future.
    /// When compiled for WebAssembly, this funcion currently always returns the error [`MapperError::MapperDisabled`].
    /// This is both because CORS makes this mapper useless and because `reqwest::blocking` does not work on WASM targets.
    /// See [reqwest#891](https://github.com/seanmonstar/reqwest/issues/891) and [reqwest#1068](https://github.com/seanmonstar/reqwest/issues/1068) for details.
    ExpandShortLink,
    /// Sets the specified URL part to `with`.
    /// # Errors
    /// If `with` is `None` and `part` is one [`types::UrlPart::Whole`], [`types::UrlPart::Scheme`], [`types::UrlPart::Username`], or [`types::UrlPart::Path`], returns the error [`types::ReplaceError::PartCannotBeNone`].
    SetUrlPart {
        /// The name of the part to replace.
        part: types::UrlPart,
        /// The value to replace the part with.
        with: Option<String>
    },
    /// Applies a regular expression substitution to the specified URL part.
    /// if `none_to_empty_string` is `false`, then getting the password, host, domain, port, query, or fragment may result in a [`super::conditions::ConditionError::UrlPartNotFound`] error.
    /// Also note that ports are strings because I can't be bothered to handle numbers for just ports.
    /// # Errors
    /// Returns the error [`MapperError::MapperDisabled`] if URL Cleaner is compiled without the `regex` feature.
    /// If chosen part's getter returns `None` and `none_to_empty_string` is set to `false`, returns the error [`MapperError::UrlPartNotFound`].
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
    /// Extracts the specified query paramater's value and sets the specified part of the URL to that value.
    /// # Errors
    /// If the specified query paramater is not found, returns the error [`MapperError::CannotFindQueryParam`].
    /// If the requested part replacement fails (returns an error [`crate::types::ReplaceError`]), that error is returned.
    GetPartFromQueryParam {
        /// The name of the part to replace.
        part: types::UrlPart,
        /// The query paramater to get the part from.
        param_name: String
    },
    /// Execute a command and sets the URL to its output. Any argument paramater with the value `"{}"` is replaced with the URL. If the command STDOUT ends in a newline it is stripped.
    /// Useful when what you want to do is really specific and niche.
    /// # Errors
    /// Returns the error [`MapperError::MapperDisabled`] if URL Cleaner is compiled without the `commands` feature.
    /// Returns the error [`glue::CommandError`] if the command fails.
    ReplaceWithCommandOutput(glue::CommandWrapper)
}

const fn get_true() -> bool {true}

/// An enum of all possible errors a [`Mapper`] can reutrn.
#[derive(Error, Debug)]
pub enum MapperError {
    /// Returned on mappers that require regex, glob, or http when those features are disabled.
    #[allow(dead_code)]
    #[error("Url-cleaner was compiled without support for this mapper.")]
    MapperDisabled,
    /// The [`Mapper::Error`] mapper always returns this error.
    #[error("The \"Error\" mapper always returns this error.")]
    ExplicitError,
    /// Returned when the mapper has `none_to_empty_string` set to `false` and the requested part of the provided URL is `None`.
    #[error("The provided URL does not have the requested part.")]
    UrlPartNotFound,
    /// Returned when the provided URL's query does not contain a query paramater with the requested name.
    #[error("The URL provided does not contain the query paramater required.")]
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
    ReplaceError(#[from] types::ReplaceError),
    /// UTF-8 error.
    #[error(transparent)]
    Utf8Error(#[from] Utf8Error),
    /// The command failed.
    #[error(transparent)]
    CommandError(#[from] glue::CommandError),
    /// A string operation failed.
    #[error(transparent)]
    StringError(#[from] types::StringError)
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
    pub fn apply(&self, url: &mut Url) -> Result<(), MapperError> {
        match self {
            Self::None => {},
            Self::Error => Err(MapperError::ExplicitError)?,
            Self::Debug(mapper) => {
                let url_before_mapper=url.clone();
                let mapper_result=mapper.apply(url);
                eprintln!("=== Debug Mapper output ===\nMapper: {mapper:?}\nURL before mapper: {url_before_mapper:?}\nMapper return value: {mapper_result:?}\nURL after mapper: {url:?}");
                mapper_result?;
            }
            Self::IgnoreError(mapper) => {
                let _=mapper.apply(url);
            },
            Self::TryCatch{r#try, catch} => r#try.apply(url).or_else(|_| catch.apply(url))?,
            Self::All(mappers) => {
                let mut temp_url=url.clone();
                for mapper in mappers {
                    mapper.apply(&mut temp_url)?;
                }
                *url=temp_url;
            },
            Self::AllNoRevert(mappers) => {
                for mapper in mappers {
                    mapper.apply(url)?
                }
            },
            Self::AllIgnoreError(mappers) => {
                for mapper in mappers {
                    let _=mapper.apply(url);
                }
            },
            Self::RemoveQuery => {
                url.set_query(None);
            },
            Self::RemoveQueryParams(names) => {
                let new_query=form_urlencoded::Serializer::new(String::new()).extend_pairs(url.query_pairs().filter(|(name, _)| names.iter().all(|blocked_name| blocked_name!=name))).finish();
                if new_query.is_empty() {
                    url.set_query(None);
                } else {
                    url.set_query(Some(&new_query));
                }
            },
            Self::AllowQueryParams(names) => {
                let new_query=form_urlencoded::Serializer::new(String::new()).extend_pairs(url.query_pairs().filter(|(name, _)| names.iter().any(|allowed_name| allowed_name==name))).finish();
                if new_query.is_empty() {
                    url.set_query(None);
                } else {
                    url.set_query(Some(&new_query));
                }
            },
            #[cfg(feature = "regex")]
            Self::RemoveQueryParamsMatchingRegex(regex) => {
                let new_query=form_urlencoded::Serializer::new(String::new()).extend_pairs(url.query_pairs().filter(|(name, _)| !regex.is_match(name))).finish();
                if new_query.is_empty() {
                    url.set_query(None);
                } else {
                    url.set_query(Some(&new_query));
                }
            },
            #[cfg(not(feature = "regex"))]
            Self::RemoveQueryParamsMatchingRegex(..) => Err(MapperError::MapperDisabled)?,

            #[cfg(feature = "regex")]
            Self::AllowQueryParamsMatchingRegex(regex) => {
                let new_query=form_urlencoded::Serializer::new(String::new()).extend_pairs(url.query_pairs().filter(|(name, _)| regex.is_match(name))).finish();
                if new_query.is_empty() {
                    url.set_query(None);
                } else {
                    url.set_query(Some(&new_query));
                }
            },
            #[cfg(not(feature = "regex"))]
            Self::AllowQueryParamsMatchingRegex(..) => Err(MapperError::MapperDisabled)?,
            Self::GetUrlFromQueryParam(name) => {
                match url.query_pairs().into_owned().find(|(param_name, _)| param_name==name) {
                    Some((_, new_url)) => {*url=Url::parse(&new_url)?},
                    None => Err(MapperError::CannotFindQueryParam)?
                }
            },
            Self::GetPathFromQueryParam(name) => {
                match url.query_pairs().into_owned().find(|(param_name, _)| param_name==name) {
                    Some((_, new_path)) => {url.set_path(&new_path);},
                    None => Err(MapperError::CannotFindQueryParam)?
                }
            },
            Self::SetHost(new_host) => {
                url.set_host(Some(new_host))?;
            },
            Self::ModifyUrlPart{part, none_to_empty_string, how} => {
                let mut part_value=part.get_from(url)
                    .ok_or(MapperError::UrlPartNotFound).or(if *none_to_empty_string {Ok(Cow::Borrowed(""))} else {Err(MapperError::UrlPartNotFound)})?.to_string();
                how.apply(&mut part_value)?;
                part.replace_with(url, Some(&part_value))?;
            },
            Self::RemovePathSegments(indices) => {
                url.set_path(&url.path().split('/').enumerate().filter(|(i, _)| !indices.contains(&i)).map(|(_, x)| x).collect::<Vec<_>>().join("/"))
            },
            #[cfg(feature = "http")]
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
                #[cfg(not(target_family = "wasm"))]
                {
                    let new_url=reqwest::blocking::Client::new().get(url.to_string()).send()?.url().clone();
                    *url=new_url.clone();
                    // Intentionally ignore any and all file writing errors.
                    // Probably should return a warning but idk how to make that.
                    // enum Warning<T, W, E> {Ok(T), Warning(T, W), Error(E)} is obvious.
                    // But I'd want to bubble up a warning then return the Ok value with it.
                    #[cfg(feature = "cache-redirects")]
                    if let Ok(mut x) = OpenOptions::new().create(true).append(true).open("redirect-cache.txt") {
                        let _=x.write(format!("\n{}\t{}", url.as_str(), new_url.as_str()).as_bytes());
                    }
                }
                #[cfg(target_family = "wasm")]
                {
                    Err(MapperError::MapperDisabled)?
                    // let client=web_sys::XmlHttpRequest::new().unwrap();
                    // client.open("GET", url.as_str());
                    // client.send(); // Doesn't wait for it to return.
                    // *url=Url::parse(&client.response_url())?;
                }
            },
            #[cfg(not(feature = "http"))]
            Self::ExpandShortLink => Err(MapperError::MapperDisabled)?,
            Self::SetUrlPart{part, with} => part.replace_with(url, with.as_deref())?,
            #[cfg(feature = "regex")]
            Self::RegexSubUrlPart {part, none_to_empty_string, regex, replace} => {
                if cfg!(feature = "regex") {
                    let old_part_value=part
                        .get_from(url)
                        .ok_or(MapperError::UrlPartNotFound)
                        .or_else(|_| if *none_to_empty_string {Ok(Cow::Owned("".to_string()))} else {Err(MapperError::UrlPartNotFound)})?
                        .into_owned();
                    part.replace_with(url, Some(&regex.replace(&old_part_value, replace)))?;
                } else {
                    Err(MapperError::MapperDisabled)?;
                }
            },
            #[cfg(not(feature = "regex"))]
            Self::RegexSubUrlPart{..} => Err(MapperError::MapperDisabled)?,
            Self::GetPartFromQueryParam{part, param_name} => {
                match url.query_pairs().into_owned().find(|(name, _)| param_name==name) {
                    Some((_, new_part)) => {part.replace_with(url, Some(&new_part))?;},
                    None => Err(MapperError::CannotFindQueryParam)?
                }
            },
            #[cfg(feature = "commands")]
            Self::ReplaceWithCommandOutput(command) => {*url=command.get_url(url)?;},
            #[cfg(not(feature = "commands"))]
            Self::ReplaceWithCommandOutput(..) => Err(MapperError::MapperDisabled)?,
        };
        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::glue::RegexParts;

    macro_rules! exurl {
        () => {Url::parse("https://www.example.com").unwrap()};
    }

    #[test]
    fn remove_query_params() {
        let mut url=Url::parse("https://example.com?a=2&b=3&c=4&d=5").unwrap();
        assert!(Mapper::RemoveQueryParams(vec!["a".to_string()]).apply(&mut url).is_ok());
        assert_eq!(url.query(), Some("b=3&c=4&d=5"));
        assert!(Mapper::RemoveQueryParams(vec!["b".to_string(), "c".to_string()]).apply(&mut url).is_ok());
        assert_eq!(url.query(), Some("d=5"));
        assert!(Mapper::RemoveQueryParams(vec!["d".to_string()]).apply(&mut url).is_ok());
        assert_eq!(url.query(), None);
    }

    #[test]
    fn allow_query_params() {
        let mut url=Url::parse("https://example.com?a=2&b=3&c=4&d=5").unwrap();
        assert!(Mapper::RemoveQueryParams(vec!["a".to_string()]).apply(&mut url).is_ok());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn remove_query_params_matching_regex() {
        let mut url=Url::parse("https://example.com?a=2&b=3&c=4&d=5").unwrap();
        assert!(Mapper::AllowQueryParamsMatchingRegex(RegexParts::new("a|b|c").try_into().unwrap()).apply(&mut url).is_ok());
        assert_eq!(url.query(), Some("a=2&b=3&c=4"));
        assert!(Mapper::AllowQueryParamsMatchingRegex(RegexParts::new("d").try_into().unwrap()).apply(&mut url).is_ok());
        assert_eq!(url.query(), None);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn allow_query_params_matching_regex() {
        let mut url=Url::parse("https://example.com?a=2&b=3&c=4&d=5").unwrap();
        assert!(Mapper::RemoveQueryParamsMatchingRegex(RegexParts::new("a|b|c").try_into().unwrap()).apply(&mut url).is_ok());
        assert_eq!(url.query(), Some("d=5"));
        assert!(Mapper::RemoveQueryParamsMatchingRegex(RegexParts::new("d").try_into().unwrap()).apply(&mut url).is_ok());
        assert_eq!(url.query(), None);
    }

    #[test]
    fn try_catch() {
        assert!(Mapper::TryCatch {r#try: Box::new(Mapper::None ), catch: Box::new(Mapper::None )}.apply(&mut exurl!()).is_ok ());
        assert!(Mapper::TryCatch {r#try: Box::new(Mapper::None ), catch: Box::new(Mapper::Error)}.apply(&mut exurl!()).is_ok ());
        assert!(Mapper::TryCatch {r#try: Box::new(Mapper::Error), catch: Box::new(Mapper::None )}.apply(&mut exurl!()).is_ok ());
        assert!(Mapper::TryCatch {r#try: Box::new(Mapper::Error), catch: Box::new(Mapper::Error)}.apply(&mut exurl!()).is_err());
    }

    #[test]
    fn all() {
        let mut url=exurl!();
        assert!(Mapper::All(vec![Mapper::SetHost("2.com".to_string()), Mapper::Error]).apply(&mut url).is_err());
        assert_eq!(url.domain(), Some("www.example.com"));
    }

    #[test]
    fn all_no_revert() {
        let mut url=exurl!();
        assert!(Mapper::AllNoRevert(vec![Mapper::SetHost("3.com".to_string()), Mapper::Error, Mapper::SetHost("4.com".to_string())]).apply(&mut url).is_err());
        assert_eq!(url.domain(), Some("3.com"));
    }

    #[test]
    fn all_ignore_error() {
        let mut url=exurl!();
        assert!(Mapper::AllIgnoreError(vec![Mapper::SetHost("5.com".to_string()), Mapper::Error, Mapper::SetHost("6.com".to_string())]).apply(&mut url).is_ok());
        assert_eq!(url.domain(), Some("6.com"));
    }

    #[test]
    fn remove_path_segments() {
        let mut url=Url::parse("https://example.com/1/2/3/4/5/6").unwrap();
        assert!(Mapper::RemovePathSegments(vec![1,3,5,6]).apply(&mut url).is_ok());
        assert_eq!(url.path(), "/2/4");
    }
}
