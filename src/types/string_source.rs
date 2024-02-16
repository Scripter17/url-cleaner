use std::str::FromStr;
use std::convert::Infallible;
use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use url::Url;

use super::UrlPart;
use crate::config::Params;

/// Allows conditions and mappers to get strings from various sources without requiring different conditions and mappers for each variant.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum StringSource {
    /// Just a string. The most common varaint.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringSource;
    /// # use url::Url;
    /// # use url_cleaner::config::Params;
    /// # use std::borrow::Cow;
    /// let url = Url::parse("https://example.com").unwrap();
    /// assert_eq!(StringSource::String("abc".to_string()).get_string(&url, &Params::default(), false), Some(Cow::Borrowed("abc")));
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
    /// assert_eq!(StringSource::Part(UrlPart::Domain).get_string(&url, &Params::default(), false), Some(Cow::Borrowed("example.com")));
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
    /// assert_eq!(StringSource::Var("abc".to_string()).get_string(&url, &params, false), Some(Cow::Borrowed("xyz")));
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
    /// assert_eq!(x.get_string(&url, &params_1, false), Some(Cow::Borrowed("example.com")));
    /// assert_eq!(x.get_string(&url, &params_2, false), Some(Cow::Borrowed("xyz")));
    /// ```
    IfFlag {
        /// The name of the flag to check.
        flag: String,
        /// If the flag is set, use this.
        then: Box<StringSource>,
        /// If the flag is not set, use this.
        r#else: Box<StringSource>
    }
}

impl FromStr for StringSource {
    type Err=Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::String(s.to_string()))
    }
}

impl StringSource {
    /// Gets the string from the source.
    pub fn get_string<'a>(&'a self, url: &'a Url, params: &'a Params, none_to_empty_string: bool) -> Option<Cow<'a, str>> {
        let ret = match self {
            Self::String(x) => Some(Cow::Borrowed(x.as_str())),
            Self::Part(x) => x.get(url, none_to_empty_string),
            Self::Var(x) => params.vars.get(x).map(|x| Cow::Borrowed(x.as_str())),
            Self::IfFlag {flag, then, r#else} => if params.flags.contains(flag) {then.get_string(url, params, none_to_empty_string)} else {r#else.get_string(url, params, none_to_empty_string)}
        };
        if none_to_empty_string {
            ret.or(Some(Cow::Borrowed("")))
        } else {
            ret
        }
    }
}
