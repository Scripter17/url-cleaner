use url::{Url, ParseError};
use reqwest;
// use glob;

#[derive(Debug)]
pub enum Condition {
    All(Vec<Condition>),
    Any(Vec<Condition>),
    UnqualifiedHost(String),
    QualifiedHost(String),
    Path(String)
    // PathGlob(glob::Pattern)
}

impl Condition {
    pub fn satisfied_by(&self, url: &Url) -> bool {
        dbg!(format!("Checking {self:?} for {url:?}"));
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
            Self::Path(path) => path==url.path()
            // Self::PathGlob(pattern) => pattern.matches(url.path())
        };
        dbg!(format!("Condition is {res}"));
        res
    }
}

#[derive(Debug)]
pub enum Mapping {
    Multiple(Vec<Mapping>),
    RemoveAllQueryParams,
    RemoveSomeQueryParams(Vec<String>),
    AllowSomeQueryParams(Vec<String>),
    GetUrlFromQueryParam(String),
    SwapHost(String),
    Expand301
}

#[derive(Debug)]
pub enum MappingError {
    CannotFindQueryParam,
    UrlParseError(ParseError),
    RequestError(reqwest::Error),
    RedirectHeaderNotFound,
    HeaderStringError(reqwest::header::ToStrError),
}

impl Mapping {
    pub fn apply(&self, url: &mut Url) -> Result<(), MappingError> {
        match self {
            Self::Multiple(mappings) => {
                for mapping in mappings.iter() {
                    mapping.apply(url);
                }
            }
            Self::RemoveAllQueryParams => url.set_query(None),
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
                    Some((_, new_url)) => {*url=Url::parse(&new_url).map_err(|err| MappingError::UrlParseError(err))?;},
                    None => Err(MappingError::CannotFindQueryParam)?
                }
            },
            Self::SwapHost(new_host) => {
                url.set_host(Some(new_host)).map_err(|err| MappingError::UrlParseError(err))?
            },
            Self::Expand301 => {
                dbg!(format!("Expanding {url:?}"));
                let client=reqwest::blocking::Client::builder().redirect(reqwest::redirect::Policy::none()).build().unwrap();
                match client.get(url.to_string()).send() {
                    Ok(response) => {
                        dbg!(format!("{:?}", response.headers()));
                        match response.headers().get("location") {
                            Some(location_header) => match location_header.to_str() {
                                Ok(location_str) => match Url::parse(location_str) {
                                    Ok(new_url) => {*url=new_url;},
                                    Err(e) => {
                                        dbg!(format!("Failed to parse location header URL: {e:?}"));
                                        Err(MappingError::UrlParseError(e))?
                                    }
                                },
                                Err(e) => {
                                    dbg!(format!("Failed to stringify location header: {e:?}"));
                                    Err(MappingError::HeaderStringError(e))?
                                }
                            },
                            None => {
                                dbg!(format!("Location header not found"));
                                Err(MappingError::RedirectHeaderNotFound)?
                            }
                        };
                    }
                    Err(e) => {println!("Expanding url failed: {e:?}"); Err(MappingError::RequestError(e))?;}
                }
                dbg!(format!("Expanded url is now {url:?}"));
            }
        };
        Ok(())
    }

    pub fn mapped_url_from(&self, url: &Url) -> Result<Url, MappingError> {
        let mut url=url.clone();
        self.apply(&mut url)?;
        Ok(url)
    }
}

#[derive(Debug)]
pub struct Rule {
    pub condition: Condition,
    pub mapping: Mapping
}

#[derive(Debug)]
pub enum RuleError {
    FailedCondition
}

impl Rule {
    pub fn apply(&self, url: &mut Url) -> Result<(), RuleError> {
        if self.condition.satisfied_by(url) {
            self.mapping.apply(url);
            Ok(())
        } else {
            Err(RuleError::FailedCondition)
        }
    }

    pub fn mapped_url_from(&self, url: &Url) -> Result<Url, RuleError> {
        let mut url=url.clone();
        self.apply(&mut url)?;
        Ok(url)
    }
}
