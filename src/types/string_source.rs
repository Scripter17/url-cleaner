use std::str::FromStr;
use std::convert::Infallible;
use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use url::Url;
use thiserror::Error;

use super::{UrlPart, StringModification, StringModificationError, StringError};
use crate::config::Params;
use crate::glue::box_string_or_struct;

/// Allows conditions and mappers to get strings from various sources without requiring different conditions and mappers for each source.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum StringSource {
    Error,
    Debug(Box<Self>),
    /// Just a string. The most common varaint.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringSource;
    /// # use url::Url;
    /// # use url_cleaner::config::Params;
    /// # use std::borrow::Cow;
    /// let url = Url::parse("https://example.com").unwrap();
    /// assert!(StringSource::String("abc".to_string()).get_string(&url, &Params::default(), false).is_ok_and(|x| x==Some(Cow::Borrowed("abc"))));
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
    /// assert!(StringSource::Part(UrlPart::Domain).get_string(&url, &Params::default(), false).is_ok_and(|x| x==Some(Cow::Borrowed("example.com"))));
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
    /// assert!(StringSource::Var("abc".to_string()).get_string(&url, &params, false).is_ok_and(|x| x==Some(Cow::Borrowed("xyz"))));
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
    /// assert!(x.get_string(&url, &params_1, false).is_ok_and(|x| x==Some(Cow::Borrowed("example.com"))));
    /// assert!(x.get_string(&url, &params_2, false).is_ok_and(|x| x==Some(Cow::Borrowed("xyz"))));
    /// ```
    IfFlag {
        /// The name of the flag to check.
        flag: String,
        /// If the flag is set, use this.
        then: Box<Self>,
        /// If the flag is not set, use this.
        r#else: Box<Self>
    },
    /// # Errors
    /// If the call to [`StringModification::apply`] errors, returns that error.
    Modified {
        #[serde(deserialize_with = "box_string_or_struct")]
        source: Box<Self>,
        modification: StringModification
    }
}

impl FromStr for StringSource {
    type Err=Infallible;

    /// Simply encase the provided string in a [`StringSource::String`] since it's the most common variant.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::String(s.to_string()))
    }
}

#[derive(Debug, Error)]
pub enum StringSourceError {
    #[error(transparent)]
    StringError(#[from] StringError),
    #[error(transparent)]
    StringModificationError(#[from] StringModificationError),
    #[error("StringSource::Error was used.")]
    ExplicitError
}

impl StringSource {
    /// Gets the string from the source.
    /// # Errors
    /// If `self` is [`Self::Modified`] and the call to [`StringModification::apply`] errors, that error is returned.
    pub fn get_string<'a>(&'a self, url: &'a Url, params: &'a Params, none_to_empty_string: bool) -> Result<Option<Cow<'a, str>>, StringSourceError> {
        let ret = Ok(match self {
            Self::String(x) => Some(Cow::Borrowed(x.as_str())),
            Self::Part(x) => x.get(url, none_to_empty_string),
            Self::Var(x) => params.vars.get(x).map(|x| Cow::Borrowed(x.as_str())),
            Self::IfFlag {flag, then, r#else} => if params.flags.contains(flag) {then.get_string(url, params, none_to_empty_string)?} else {r#else.get_string(url, params, none_to_empty_string)?},
            Self::Modified {source, modification} => {
                let x=source.as_ref().get_string(url, params, none_to_empty_string)?.map(Cow::into_owned);
                if let Some(mut x) = x {
                    modification.apply(&mut x, params)?;
                    Some(Cow::Owned(x))
                } else {
                    x.map(Cow::Owned)
                }
            },
            Self::Debug(source) => {
                let ret=source.get_string(url, params, none_to_empty_string);
                eprintln!("=== Debug StringSource ===\nSource: {source:?}\nParams: {params:?}\nnone_to_empty_string: {none_to_empty_string:?}\nret: {ret:?}");
                ret?
            },
            Self::Error => Err(StringSourceError::ExplicitError)?
        });
        if none_to_empty_string {
            ret.map(|x| x.or(Some(Cow::Borrowed(""))))
        } else {
            ret
        }
    }
}
