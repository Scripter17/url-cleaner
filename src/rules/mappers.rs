//! The logic for how to modify a URL.

use serde::{Serialize, Deserialize};
use thiserror::Error;
use url::{Url, ParseError};
#[cfg(feature = "regex")]
use std::borrow::Cow;

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
    Error,
    /// Prints debugging information about the contained mapper and its effect on the URL to STDERR.
    /// Intended primarily for debugging logic errors.
    /// *Can* be used in production as in both bash and batch `x|y` only pipes `x`'s STDOUT, but it'll look ugly.
    Debug(Box<Mapper>),
    /// Ignores any error the contained mapper may throw.
    IgnoreError(Box<Mapper>),
    /// Applies the contained mappers in order. If a mapper throws an error, the URL is left unchanged and the error is propagated up.
    Multiple(Vec<Mapper>),
    /// Applies the contained mappers in order. If a mapper throws an error, the URL is left as whatever the previous contained mapper set it to and the error is propagated up.
    MultipleAbortOnError(Vec<Mapper>),
    /// Applies the contained mappers in order. If a mapper throws an error, subsequent mappers are still applied and the error is ignored.
    /// This is equivalent to wrapping every contained mapper in a [`Mapper::IgnoreError`].
    MultipleIgnoreError(Vec<Mapper>),
    /// Removes the URL's entire query.
    /// Useful for webites that only use the query for tracking.
    RemoveAllQueryParams,
    /// Removes query paramaters whose name is in the specified names.
    /// Useful for websites that append random stuff to shared URLs so the website knows your friend got that link from you.
    RemoveSomeQueryParams(Vec<String>),
    /// Removes query paramaters whose name isn't in the specified names.
    /// Useful for websites that keep changing their tracking paramaters and you're sick of updating your rule set.
    AllowSomeQueryParams(Vec<String>),
    /// Removes query paramaters whose name matches the specified regex.
    /// Useful for parsing AdGuard rules.
    RemoveQueryParamsMatchingRegex(glue::RegexWrapper),
    /// Removes query paramaters whose name doesn't match the specified regex.
    /// Useful for parsing AdGuard rules.
    AllowQueryParamsMatchingRegex(glue::RegexWrapper),
    /// Replace the current URL with the value of the specified query paramater.
    /// Useful for websites for have a "are you sure you want to leave?" page with a URL like `https://example.com/outgoing?to=https://example.com`.
    GetUrlFromQueryParam(String),
    /// Replace the current URL's path with the value of the specified query paramater.
    /// Useful for websites that have a "you must log in to see this page" page.
    GetPathFromQueryParam(String),
    /// Replaces the URL's host to the provided host.
    /// Useful for websites that are just a wrapper around another website. For example, `vxtwitter.com`.
    SwapHost(String),
    /// Sends an HTTP request to the current URL and replaces it with the URL the website responds with.
    /// Useful for link shorteners like `bit.ly` and `t.co`.
    Expand301,
    /// Applies a regular expression substitution to the specified URL part.
    /// if `none_to_empty_string` is `false`, then getting the password, host, domain, port, query, or fragment may result in a [`ConditionError::UrlPartNotFound`](super::conditions::ConditionError::UrlPartNotFound) error.
    /// Also note that ports are strings because I can't be bothered to handle numbers for just ports.
    RegexSubUrlPart {
        /// The name of the part to modify.
        part_name: types::UrlPartName,
        /// If the relevant [`Url`] part getter returns [`None`], this decides whether to return a [`ConditionError::UrlPartNotFound`] or pretend it's just an empty string and check that.
        /// Defaults to [`true`].
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        /// The [`glue::GlobWrapper`] that is used to match and extract parts of the selected part.
        regex: glue::RegexWrapper,
        /// The pattern the extracted parts are put into.
        /// See [`regex::Regex::replace`] for details.
        replace: String
    },
    /// Execute a command. Any argument paramater with the value `"{}"` is replaced with the URL. If the command STDOUT ends in a newline it is stripped.
    /// Useful when what you want to do is really specific and niche.
    ReplaceWithCommandOutput(glue::CommandWrapper)
}

fn get_true() -> bool {true}

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
    #[error("Could not parse the would-be new URL.")]
    UrlParseError(#[from] ParseError),
    /// Returned when an HTTP request fails. Currently only applies to the Expand301 mapper.
    #[error("The HTTP request failed.")]
    ReqwestError(#[from] ReqwestError),
    /// Returned when an I/O error occurs. Currently only applies when Expand301 is set to cache redirects.
    #[error("IO Error.")]
    IoError(#[from] IoError),
    /// Returned when a part replacement fails.
    #[error("Replacement error.")]
    ReplaceError(#[from] types::ReplaceError),
    /// UTF-8 error.
    #[error("UTF-8 error.")]
    Utf8Error(#[from] Utf8Error),
    /// The command failed.
    #[error("The command failed.")]
    CommandError(#[from] glue::CommandError)
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
            Self::Multiple(mappers) => {
                let mut temp_url=url.clone();
                for mapper in mappers {
                    mapper.apply(&mut temp_url)?;
                }
                *url=temp_url;
            },
            Self::MultipleAbortOnError(mappers) => {
                for mapper in mappers {
                    mapper.apply(url)?
                }
            },
            Self::MultipleIgnoreError(mappers) => {
                for mapper in mappers {
                    let _=mapper.apply(url);
                }
            },
            Self::RemoveAllQueryParams => {
                url.set_query(None);
            },
            Self::RemoveSomeQueryParams(names) => {
                // Apparently `x.y().z(f())` will execute `x.y()` before `f()`
                let new_query=url.query_pairs().into_owned().filter(|(name, _)| names.iter().all(|blocked_name| blocked_name!=name)).collect::<Vec<_>>();
                url.query_pairs_mut().clear().extend_pairs(new_query);
            },
            Self::AllowSomeQueryParams(names) => {
                let new_query=url.query_pairs().into_owned().filter(|(name, _)| names.iter().any(|allowed_name| allowed_name==name)).collect::<Vec<_>>();
                url.query_pairs_mut().clear().extend_pairs(new_query);
            },
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
            Self::SwapHost(new_host) => {
                url.set_host(Some(new_host))?;
            },

            // Disablanle conditions

            #[cfg(feature = "http")]
            Self::Expand301 => {
                #[cfg(feature = "cache-redirects")]
                if let Ok(lines) = read_lines("301-cache.txt") {
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
                    if let Ok(mut x) = OpenOptions::new().create(true).append(true).open("301-cache.txt") {
                        let _=x.write(format!("\n{}\t{}", url.as_str(), new_url.as_str()).as_bytes());
                    }
                }
                #[cfg(target_family = "wasm")]
                {
                    todo!();
                    // let client=web_sys::XmlHttpRequest::new().unwrap();
                    // client.open("GET", url.as_str());
                    // client.send(); // Doesn't wait for it to return.
                    // *url=Url::parse(&client.response_url())?;
                }
            },
            #[cfg(not(feature = "http"))]
            Self::Expand301 => Err(MapperError::MapperDisabled)?,

            #[cfg(feature = "regex")]
            Self::RegexSubUrlPart {part_name, none_to_empty_string, regex, replace} => {
                if cfg!(feature = "regex") {
                    let old_part_value=part_name
                        .get_from(url)
                        .ok_or(MapperError::UrlPartNotFound)
                        .or_else(|_| if *none_to_empty_string {Ok(Cow::Owned("".to_string()))} else {Err(MapperError::UrlPartNotFound)})?
                        .into_owned();
                    part_name.replace_with(url, regex.replace(&old_part_value, replace).as_ref())?;
                } else {
                    Err(MapperError::MapperDisabled)?;
                }
            },
            #[cfg(feature = "regex")]
            Self::RemoveQueryParamsMatchingRegex(regex) => {
                let new_query=url.query_pairs().into_owned().filter(|(name, _)| !regex.is_match(name)).collect::<Vec<_>>();
                url.query_pairs_mut().clear().extend_pairs(new_query);
            },
            #[cfg(feature = "regex")]
            Self::AllowQueryParamsMatchingRegex(regex) => {
                let new_query=url.query_pairs().into_owned().filter(|(name, _)| regex.is_match(name)).collect::<Vec<_>>();
                url.query_pairs_mut().clear().extend_pairs(new_query);
            },

            #[cfg(not(feature = "regex"))] Self::RegexSubUrlPart               {..} => Err(MapperError::MapperDisabled)?,
            #[cfg(not(feature = "regex"))] Self::RemoveQueryParamsMatchingRegex(..) => Err(MapperError::MapperDisabled)?,
            #[cfg(not(feature = "regex"))] Self::AllowQueryParamsMatchingRegex (..) => Err(MapperError::MapperDisabled)?,

            #[cfg(    feature = "commands") ] Self::ReplaceWithCommandOutput(command) => {*url=command.get_url(url)?;},
            #[cfg(not(feature = "commands"))] Self::ReplaceWithCommandOutput(..)      => Err(MapperError::MapperDisabled)?,
        };
        Ok(())
    }
}
