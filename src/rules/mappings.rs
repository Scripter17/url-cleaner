use serde::Deserialize;
use thiserror::Error;
use url::{Url, ParseError};
use std::path::Path;
use std::borrow::Cow;

#[cfg(feature = "http")]
use reqwest;

#[cfg(feature = "cache-redirects")]
use std::{
    io::{self, BufRead, Write},
    fs::{OpenOptions, File}
};

use crate::glue;
use crate::types;

#[derive(Debug, Deserialize, Clone)]
pub enum Mapping {
    IgnoreError(Box<Mapping>),
    Multiple(Vec<Mapping>),
    MultipleAbortOnError(Vec<Mapping>),
    MultipleIgnoreError(Vec<Mapping>),
    RemoveAllQueryParams,
    RemoveSomeQueryParams(Vec<String>),
    AllowSomeQueryParams(Vec<String>),
    GetUrlFromQueryParam(String),
    GetPathFromQueryParam(String),
    SwapHost(String),
    Expand301,
    RegexSubUrlPart {
        part_name: types::UrlPartName,
        none_to_empty_string: bool,
        regex: glue::Regex,
        replace: String
    }
}

#[derive(Error, Debug)]
pub enum MappingError {
    #[allow(dead_code)]
    #[error("Url-cleaner was compiled without support for this mapper")]
    MapperDisabled,
    #[error("Provided URL does not have the requested part")]
    UrlPartNotFound,
    #[error("The URL provided does not contain the query paramater required")]
    CannotFindQueryParam,
    #[error("Coult not parse the would-be new URL")]
    UrlParseError(#[from] ParseError),
    #[cfg(feature = "http")]
    #[error("The HTTP request failed")]
    ReqwestError(#[from] reqwest::Error),
    #[cfg(feature = "cache-redirects")]
    #[error("IO Error")]
    IoError(#[from] io::Error),
    #[error("Could not convert result of replacement as a URL")]
    PartReplace(#[from] types::PartReplaceError)
}

#[cfg(feature = "cache-redirects")]
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

impl Mapping {
    pub fn apply(&self, url: &mut Url) -> Result<(), MappingError> {
        match self {
            Self::IgnoreError(mapping) => {
                let _=mapping.apply(url);
            },
            Self::Multiple(mappings) => {
                let mut temp_url=url.clone();
                for mapping in mappings {
                    mapping.apply(&mut temp_url)?;
                }
                *url=temp_url;
            },
            Self::MultipleAbortOnError(mappings) => {
                for mapping in mappings {
                    mapping.apply(url)?
                }
            },
            Self::MultipleIgnoreError(mappings) => {
                for mapping in mappings {
                    let _=mapping.apply(url);
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
                    None => Err(MappingError::CannotFindQueryParam)?
                }
            },
            Self::GetPathFromQueryParam(name) => {
                match url.query_pairs().into_owned().find(|(param_name, _)| param_name==name) {
                    Some((_, new_path)) => {url.set_path(&new_path);},
                    None => Err(MappingError::CannotFindQueryParam)?
                }
            },
            Self::SwapHost(new_host) => {
                url.set_host(Some(new_host))?;
            },
            Self::Expand301 => {
                #[cfg(all(not(feature = "http"), not(feature = "cache-redirects")))]
                Err(MappingError::MapperDisabled)?;
                #[cfg(feature = "cache-redirects")]
                if let Ok(lines) = read_lines("redirect-cache.txt") {
                    for line in lines.filter_map(Result::ok) {
                        if let Some((short, long)) = line.split_once('\t') {
                            if url.as_str()==short {
                                *url=Url::parse(&long)?;
                                return Ok(());
                            }
                        }
                    }
                }
                #[cfg(feature = "http")]
                {
                    #[cfg(not(target_family = "wasm"))]
                    {
                        let new_url=reqwest::blocking::Client::new().get(url.to_string()).send()?.url().clone();
                        *url=new_url.clone();
                        #[cfg(feature = "cache-redirects")]
                        OpenOptions::new().create(true).append(true).open("redirect-cache.txt")?.write(format!("{}\t{}", url.as_str(), new_url.as_str()).as_bytes())?;
                    }
                    #[cfg(target_family = "wasm")]
                    todo!();
                }
            },
            Self::RegexSubUrlPart {part_name, none_to_empty_string, regex, replace} => {
                if cfg!(feature = "regex") {
                    let old_part_value=part_name
                        .get_from(url)
                        .ok_or(MappingError::UrlPartNotFound)
                        .or_else(|_| if *none_to_empty_string {Ok(Cow::Owned("".to_string()))} else {Err(MappingError::UrlPartNotFound)})?
                        .into_owned();
                    part_name.replace_with(url, regex.replace(&old_part_value, replace).as_ref())?;
                } else {
                    Err(MappingError::MapperDisabled)?;
                }
            }
        };
        Ok(())
    }
}

