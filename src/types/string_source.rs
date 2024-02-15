use std::str::FromStr;
use std::convert::Infallible;
use std::borrow::Cow;
use std::ops::Deref;

use serde::{Serialize, Deserialize};
use url::Url;

use super::UrlPart;
use crate::config::Params;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum StringSource {
    String(String),
    Part(UrlPart),
    Var(String)
}

impl FromStr for StringSource {
    type Err=Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::String(s.to_string()))
    }
}

impl StringSource {
    pub fn get_string<'a>(&'a self, url: &'a Url, params: &'a Params, none_to_empty_string: bool) -> Option<Cow<'a, str>> {
        let ret = match self {
            Self::String(x) => Some(Cow::Borrowed(x.deref())),
            Self::Part(x) => x.get(url, none_to_empty_string),
            Self::Var(x) => params.vars.get(x).map(|x| Cow::Borrowed(x.deref()))
        };
        if none_to_empty_string {
            ret.or(Some(Cow::Borrowed("")))
        } else {
            ret
        }
    }
}
