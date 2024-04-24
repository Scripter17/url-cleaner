//! The [`rules::Rule`] type is the primary interface for URL manipulation.

use url::Url;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use thiserror::Error;

mod conditions;
pub use conditions::*;
mod mappers;
pub use mappers::*;

pub use crate::types::*;

/// The main API for modifying URLs.
/// 
/// [`Rule::Normal`] is almost always what you want.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Rule {
    /// A faster but less versatile mode that uses a hashmap to save on iterations in [`Rules`].
    /// Strips leading `"www."` from the provided URL to act like [`conditions::Condition::MaybeWWWDomain`].
    /// # Errors
    /// If the provided URL doesn't have a host, returns the error [`RuleError::UrlHasNoHost`].
    /// 
    /// If the provided URL's host isn't in the rule's map, returns the error [`RuleError::HostNotInMap`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::{Rule, Mapper, Params};
    /// # use url::Url;
    /// # use std::collections::HashMap;
    /// let rule=Rule::HostMap(HashMap::from_iter([
    ///     ("example1.com".to_string(), Mapper::SetHost("example2.com".to_string())),
    ///     ("example2.com".to_string(), Mapper::SetHost("example1.com".to_string()))
    /// ]));
    /// 
    /// let mut url1 = Url::parse("https://example1.com").unwrap();
    /// assert!(rule.apply(&mut url1, &Params::default()).is_ok());
    /// assert_eq!(url1.as_str(), "https://example2.com/");
    /// 
    /// let mut url2 = Url::parse("https://example2.com").unwrap();
    /// assert!(rule.apply(&mut url2, &Params::default()).is_ok());
    /// assert_eq!(url2.as_str(), "https://example1.com/");
    /// ```
    HostMap(HashMap<String, Mapper>),
    /// Runs all the contained rules until none of their conditions pass.
    /// Runs at most `limit` times. (Defaults to 10).
    /// # Errors
    /// If a contained [`Self`] returns any error other than [`RuleError::FailedCondition`], that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::{Rule, Condition, Mapper, Params};
    /// # use url_cleaner::types::UrlPart;
    /// # use url::Url;
    /// # use std::str::FromStr;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// assert!(Rule::RepeatUntilNonePass {
    ///     rules: vec![
    ///         Rule::Normal {
    ///             condition: Condition::Always,
    ///             mapper: Mapper::SetPart {
    ///                 part: UrlPart::NextPathSegment,
    ///                 value: Some(FromStr::from_str("a").unwrap())
    ///             }
    ///         }
    ///     ],
    ///     limit: 10
    /// }.apply(&mut url, &Params::default()).is_ok());
    /// assert_eq!(url.as_str(), "https://example.com/a/a/a/a/a/a/a/a/a/a");
    /// ```
    RepeatUntilNonePass {
        /// The rules to repeat.
        rules: Vec<Rule>,
        /// The max amount of times to repeat them.
        /// Defaults to 10.
        #[serde(default = "get_10_u8")]
        limit: u8
    },
    /// The basic condition mapper rule type.
    /// This is the last variant because of the [`#[serde(untageed)]`](https://serde.rs/variant-attrs.html#untagged) macro.
    /// # Errors
    /// If the the contained condition or mapper returns an error, that error is returned.
    /// 
    /// If the [`Condition`] doesn't pass, returns the error [`RuleError::FailedCondition`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::{Rule, Condition, Mapper, Params};
    /// # use url::Url;
    /// // [`RuleError::FailedCondition`] is returned when the condition does not pass.
    /// // [`Rules`] just ignores them because it's a higher level API.
    /// assert!(Rule::Normal{condition: Condition::Never, mapper: Mapper::None}.apply(&mut Url::parse("https://example.com").unwrap(), &Params::default()).is_err());
    /// ```
    #[serde(untagged)]
    Normal {
        /// The condition under which the provided URL is modified.
        condition: Condition,
        /// The mapper used to modify the provided URL.
        mapper: Mapper
    }
}

/// Serde helper function. The default value of [`Rule::RepeatUntilNonePass::limit`].
const fn get_10_u8() -> u8 {10}

/// The errors that [`Rule`] can return.
#[derive(Debug, Error)]
pub enum RuleError {
    /// The URL does not meet the rule's condition.
    #[error("The URL does not meet the rule's condition.")]
    FailedCondition,
    /// The condition returned an error.
    #[error(transparent)]
    ConditionError(#[from] ConditionError),
    /// The mapper returned an error.
    #[error(transparent)]
    MapperError(#[from] MapperError),
    /// Returned when the provided URL doesn't have a host to find in a [`Rule::HostMap`].
    #[error("The provided URL doesn't have a host to find in the HashMap.")]
    UrlHasNoHost,
    /// Returned when the provided URL's host isn't in a [`Rule::HostMap`].
    #[error("The provided URL's host was not found in the `Rule::HostMap`.")]
    HostNotInMap
}

impl Rule {
    /// Apply the rule to the url in-place.
    /// # Errors
    /// See each of [`Self`]'s variant's documentation for details.
    pub fn apply(&self, url: &mut Url, params: &Params) -> Result<(), RuleError> {
        match self {
            Self::Normal{condition, mapper} => if condition.satisfied_by(url, params)? {
                mapper.apply(url, params)?;
                Ok(())
            } else {
                Err(RuleError::FailedCondition)
            },
            Self::HostMap(map) => Ok(map.get(url.host_str().map(|x| x.strip_prefix("www.").unwrap_or(x)).ok_or(RuleError::UrlHasNoHost)?).ok_or(RuleError::HostNotInMap)?.apply(url, params)?),
            Self::RepeatUntilNonePass{rules, limit} => {
                for _ in 0..*limit {
                    let mut done=true;
                    for rule in rules {
                        match rule.apply(url, params) {
                            Err(RuleError::FailedCondition) => {},
                            Ok(()) => done=false,
                            e @ Err(_) => e?
                        }
                    }
                    if done {break}
                }
                Ok(())
            }
        }
    }
}

/// A wrapper around a vector of rules.
/// 
/// Exists mainly for convenience.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Rules(pub Vec<Rule>);

#[allow(dead_code)]
impl Rules {
    /// Applies each rule to the provided [`Url`] in order.
    /// Bubbles up every unignored error except for [`RuleError::FailedCondition`], [`RuleError::UrlHasNoHost`], and [`RuleError::HostNotInMap`].
    /// If an error is returned, `url` is left unmodified.
    /// # Errors
    /// If any contained [`Rule`] returns an error except [`RuleError::FailedCondition`], [`RuleError::UrlHasNoHost`], or [`RuleError::HostNotInMap`] is encountered, that error is returned.
    pub fn apply(&self, url: &mut Url, params: &Params) -> Result<(), RuleError> {
        let mut temp_url=url.clone();
        for rule in &self.0 {
            match rule.apply(&mut temp_url, params) {
                Err(RuleError::FailedCondition | RuleError::UrlHasNoHost | RuleError::HostNotInMap) => {},
                e @ Err(_) => e?,
                _ => {}
            }
        }
        *url=temp_url;
        Ok(())
    }
}
