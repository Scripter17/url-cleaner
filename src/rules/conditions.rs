use std::borrow::Cow;

use thiserror::Error;
use serde::Deserialize;
use url::Url;

use crate::glue;
use crate::types::UrlPartName;

#[derive(Debug, Deserialize, Clone)]
pub enum Condition {
    Always,
    Never,
    All(Vec<Condition>),
    Any(Vec<Condition>),
    Not(Box<Condition>),
    UnqualifiedHost(String),
    QualifiedHost(String),
    UnqualifiedAnyTld(String),
    QualifiedAnyTld(String),
    PathIs(String),
    QueryHasParam(String),
    QueryParamValueIs {
        name: String,
        value: String
    },
    // Disablable conditions
    QueryParamValueMatchesRegex {
        name: String,
        regex: glue::Regex
    },
    QueryParamValueMatchesGlob {
        name: String,
        glob: glue::Glob
    },
    PathMatchesRegex(glue::Regex),
    PathMatchesGlob(glue::Glob),
    UrlPartMatchesRegex {
        part_name: UrlPartName,
        none_to_empty_string: bool,
        regex: glue::Regex
    },
    UrlPartMatchesGlob {
        part_name: UrlPartName,
        none_to_empty_string: bool,
        glob: glue::Glob
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ConditionError {
    #[allow(dead_code)]
    #[error("Url-cleaner was compiled without support for this condition")]
    ConditionDisabled,
    #[error("The provided URL does not contain the requested part")]
    UrlPartNotFound
}

impl Condition {
    pub fn satisfied_by(&self, url: &Url) -> Result<bool, ConditionError> {
        Ok(match self {
            Self::Always => true,
            Self::Never => false,
            Self::All(conditions) => conditions.iter().all(|condition| condition.satisfied_by(url)==Ok(true)),
            Self::Any(conditions) => conditions.iter().any(|condition| condition.satisfied_by(url)==Ok(true)),
            Self::Not(condition) => !condition.satisfied_by(url)?,
            Self::UnqualifiedHost(parts) => match url.domain() {
                Some(domain) => domain.split(".").collect::<Vec<_>>().ends_with(&parts.split(".").collect::<Vec<_>>()),
                None => false
            },
            Self::QualifiedHost(parts) => match url.domain() {
                Some(domain) => domain==parts,
                None => false
            },
            Self::UnqualifiedAnyTld(name) => {
                match url.domain() {
                    Some(domain) => {
                        match domain.split('.').collect::<Vec<_>>().as_slice() {
                            // All ASCII ccTLD identifiers are two letters long, and all two-letter top-level domains are ccTLDs. - https://en.wikipedia.org/wiki/Country_code_top-level_domain
                            // I'm just hoping nobody using this ever registers google.whatever.uk and nobody ever tries to sanitize a URL from that domain
                            [.., name2, _, cctld] => name==name2 && cctld.len()==2,
                            [.., name2, _       ] => name==name2,
                            _                     => false
                        }
                    }
                    None => false
                }
            },
            Self::QualifiedAnyTld(name) => {
                match url.domain() {
                    Some(domain) => {
                        match domain.strip_prefix(name) {
                            Some(partial_domain) => {
                                match partial_domain.split('.').collect::<Vec<_>>().as_slice() {
                                    [_, cctld] => cctld.len()==2,
                                    [_       ] => true,
                                    _          => false
                                }
                            },
                            None => false
                        }
                    },
                    None => false
                }
            },
            Self::PathIs(path) => path==url.path(),
            Self::QueryHasParam(name) => url.query_pairs().any(|(ref name2, _)| name2==name),
            Self::QueryParamValueIs{name, value} => url.query_pairs().any(|(ref name2, ref value2)| name2==name && value2==value),
            // Disablable conditions
            Self::QueryParamValueMatchesRegex{name, regex} => url.query_pairs().any(|(ref name2, ref value2)| name2==name && regex.is_match(value2)),
            Self::QueryParamValueMatchesGlob {name, glob } => url.query_pairs().any(|(ref name2, ref value2)| name2==name && glob .matches (value2)),
            Self::PathMatchesRegex(regex) => regex.is_match(url.path()),
            Self::PathMatchesGlob (glob ) => glob  .matches(url.path()),
            Self::UrlPartMatchesRegex {part_name, none_to_empty_string, regex} => regex.is_match(part_name.get_from(url)
                .ok_or(ConditionError::UrlPartNotFound).or_else(|_| if *none_to_empty_string {Ok(Cow::Owned("".to_string()))} else {Err(ConditionError::UrlPartNotFound)})?.as_ref()),
            Self::UrlPartMatchesGlob {part_name, none_to_empty_string, glob} => glob.matches(part_name.get_from(url)
                .ok_or(ConditionError::UrlPartNotFound).or_else(|_| if *none_to_empty_string {Ok(Cow::Owned("".to_string()))} else {Err(ConditionError::UrlPartNotFound)})?.as_ref())
        })
    }
}
