use url::{Url, ParseError};
use regex::Regex;
use serde::{Deserialize, Serialize};
use reqwest;

#[cfg(feature = "cache-301s")]
use std::{
    io::{self, BufRead, Write},
    fs::{OpenOptions, File},
    path::Path
};

#[derive(Debug, Deserialize, Serialize)]
pub enum Condition {
    All(Vec<Condition>),
    Any(Vec<Condition>),
    UnqualifiedHost(String),
    QualifiedHost(String),
    AnyTld(String),
    Path(String),
    QueryHasParam(String)
}

impl Condition {
    pub fn satisfied_by(&self, url: &Url) -> bool {
        let res=match self {
            Self::All(conditions) => conditions.iter().all(|condition| condition.satisfied_by(url)),
            Self::Any(conditions) => conditions.iter().any(|condition| condition.satisfied_by(url)),
            Self::UnqualifiedHost(parts) => match url.domain() {
                Some(domain) => domain.split(".").collect::<Vec<_>>().ends_with(&parts.split(".").collect::<Vec<_>>()),
                None => return false
            },
            Self::QualifiedHost(parts) => match url.domain() {
                Some(domain) => domain==parts,
                None => return false
            },
            Self::AnyTld(name) => {
                match url.domain() {
                    Some(domain) => Regex::new(&format!(r"(?:^|.+\.){name}(\.\w+(\.\w\w)?)")).unwrap().is_match(domain),
                    None => false
                }
            }
            Self::Path(path) => path==url.path(),
            Self::QueryHasParam(name) => url.query_pairs().into_owned().any(|(ref name2, _)| name2==name)
        };
        res
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Mapping {
    Multiple(Vec<Mapping>),
    RemoveAllQueryParams,
    RemoveSomeQueryParams(Vec<String>),
    AllowSomeQueryParams(Vec<String>),
    GetUrlFromQueryParam(String),
    SwapHost(String),
    Expand301,
    PathFromQueryParam(String),
    RemoveSubdomain
}

#[derive(Debug)]
pub enum MappingError {
    CannotFindQueryParam,
    UrlParseError(ParseError),
    ReqwestError(reqwest::Error),
    IoError(io::Error)
}

impl From<reqwest::Error> for MappingError {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value)
    }
}

impl From<ParseError> for MappingError {
    fn from(value: ParseError) -> Self {
        Self::UrlParseError(value)
    }
}

impl From<io::Error> for MappingError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

#[cfg(feature = "cache-301s")]
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

impl Mapping {
    pub fn apply(&self, url: &mut Url) -> Result<(), MappingError> {
        match self {
            Self::Multiple(mappings) => {
                for mapping in mappings.iter() {
                    mapping.apply(url)?;
                }
            },
            Self::RemoveAllQueryParams => {
                url.set_query(None);
            },
            Self::RemoveSomeQueryParams(names) => {
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
            Self::PathFromQueryParam(name) => {
                match url.query_pairs().into_owned().find(|(param_name, _)| param_name==name) {
                    Some((_, new_path)) => {url.set_path(&new_path);},
                    None => Err(MappingError::CannotFindQueryParam)?
                }
            },
            Self::SwapHost(new_host) => {
                url.set_host(Some(new_host))?;
            },
            Self::Expand301 => {
                #[cfg(feature = "cache-301s")]
                {
                    if let Ok(lines) = read_lines("301-cache.txt") {
                        for line in lines.filter_map(Result::ok) {
                            if let Some((short, long)) = line.split_once('\t') {
                                if url.as_str()==short {
                                    *url=Url::parse(&long)?;
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
                #[cfg(not(target_family = "wasm"))]
                {
                    let client=reqwest::blocking::Client::new();
                    match client.get(url.to_string()).send() {
                        Ok(response) => {
                            let new_url=response.url();
                            OpenOptions::new().append(true).open("301-cache.txt")?.write(format!("{}\t{}", url.as_str(), new_url.as_str()).as_bytes())?;
                            *url=new_url.clone();
                        },
                        Err(e) => {println!("Expanding url failed: {e:?}"); Err(e)?;}
                    }
                }
                #[cfg(target_family = "wasm")]
                {
                    todo!();
                }
            },
            Self::RemoveSubdomain => {
                todo!();
            }
        };
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Rule {
    pub condition: Condition,
    pub mapping: Mapping
}

#[derive(Debug)]
pub enum RuleError {
    FailedCondition,
    MappingError(MappingError)
}

impl Rule {
    pub fn apply(&self, url: &mut Url) -> Result<(), RuleError> {
        if self.condition.satisfied_by(url) {
            match self.mapping.apply(url) {
                Ok(_) => Ok(()),
                Err(e) => Err(RuleError::MappingError(e))
            }
        } else {
            Err(RuleError::FailedCondition)
        }
    }
}
