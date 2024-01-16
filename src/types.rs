use std::borrow::Cow;

use url::{Url, ParseError};
use thiserror::Error;
use std::str::FromStr;
use std::io::Error as IoError;

use serde::{
    Serialize,
    ser::Serializer,
    {de::Error as DeError, Deserialize, Deserializer}
};

/// An enum that makes using the various [`Url`] getters simpler.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum UrlPart {
    /// The whole URL. Corresponds to [`Url::as_str`].
    Whole,
    /// The scheme. Corresponds to [`Url::scheme`].
    Scheme,
    /// The username. Corresponds to [`Url::username`].
    Username,
    /// The password. Corresponds to [`Url::password`].
    Password,
    /// The host. Either a domain name or IPV4/6 address. Corresponds to [`Url::host`].
    Host,
    /// The subdomain. If the domain is `a.b.c.co.uk`, the value returned/changed by this is `a.b`.
    Subdomain,
    /// The domain minus the subdomain. If the domain is `a.b.c.co.uk` value returned/changed by this is `c.co.uk`.
    NotSubdomain,
    /// The domain. Corresponds to [`Url::domain`].
    Domain,
    /// The port as a string. Corresponds to [`Url::port`].
    /// Ports are always treated as strings for the sake of a simpler API.
    Port,
    /// A specficic segment of the URL's path.
    /// For most URLs the indices seems one-indexed as the path starts with a `"/"`.
    /// See [`Url::path`] for details.
    PathSegment(usize),
    /// The path. Corresponds to [`Url::path`].
    Path,
    /// The query. Corresponds to [`Url::query`].
    Query,
    /// The fragment. Corresponds to [`Url::fragment`].
    Fragment
}

/// An enum of all possible errors [`UrlPart::replace_with`] can return.
#[derive(Debug, Error)]
pub enum ReplaceError {
    /// Attempted replacement would not produce a valid URL.
    #[error(transparent)]
    ParseError(#[from] ParseError),
    /// [`UrlPart::replace_with`] attempted to set a part that cannot be None to None.
    #[error("UrlPart::replace_with attempted to set a part that cannot be None to None.")]
    PartCannotBeNone,
    /// The provided scheme would not have produced a valid URL.
    #[error("The provided scheme would not have produced a valid URL.")]
    InvalidScheme,
    /// The provided port is not a number.
    #[error("The provided port is not a number.")]
    InvalidPort,
    /// Cannot set port for this URL. Either because it is cannot-be-a-base, does not have a host, or has the file scheme.
    #[error("Cannot set port for this URL. Either because it is cannot-be-a-base, does not have a host, or has the file scheme.")]
    CannotSetPort,
    /// Cannot set username for this URL. Either because it is cannot-be-a-base or does not have a host.
    #[error("Cannot set username for this URL. Either because it is cannot-be-a-base or does not have a host.")]
    CannotSetUsername,
    /// Cannot set password for this URL. Either because it is cannot-be-a-base or does not have a host.
    #[error("Cannot set password for this URL. Either because it is cannot-be-a-base or does not have a host.")]
    CannotSetPassword,
    /// Returned by `UrlPart::Subdomain.get_from` when `UrlPart::Domain.get_from` returns `None`.
    #[error("The URL's host is not a domain.")]
    HostIsNotADomain
}

impl UrlPart {
    /// Extracts the specified part of the provided URL
    pub fn get_from<'a>(&self, url: &'a Url) -> Option<Cow<'a, str>> {
        Some(match self {
            Self::Whole          => Cow::Borrowed(url.as_str()),
            Self::Scheme         => Cow::Borrowed(url.scheme()),
            Self::Username       => Cow::Borrowed(url.username()),
            Self::Password       => Cow::Borrowed(url.password()?),
            Self::Host           => Cow::Borrowed(url.host_str()?),
            Self::Subdomain      => Cow::Borrowed({
                let domain=url.domain()?;
                // `psl::suffix_str` should never return `None`. Testing required.
                domain.strip_suffix(psl::suffix_str(domain)?)?.strip_suffix('.').unwrap_or("").rsplit_once('.').unwrap_or(("", "")).0
            }),
            Self::NotSubdomain   => Cow::Borrowed({
                let domain=url.domain()?;
                // `psl::suffix_str` should never return `None`. Testing required.
                domain.strip_suffix(psl::suffix_str(domain)?)?.strip_suffix('.').unwrap_or("").rsplit_once('.').unwrap_or(("", "")).1
            }),
            Self::Domain         => Cow::Borrowed(url.domain()?),
            Self::Port           => Cow::Owned   (url.port()?.to_string()), // I cannot be bothered to add number handling
            Self::PathSegment(n) => Cow::Borrowed(url.path().split('/').nth(*n)?),
            Self::Path           => Cow::Borrowed(url.path()),
            Self::Query          => Cow::Borrowed(url.query()?),
            Self::Fragment       => Cow::Borrowed(url.fragment()?)
        })
    }

    /// Replaces the specified part of the provided URL with the provided value
    pub fn replace_with(&self, url: &mut Url, with: Option<&str>) -> Result<(), ReplaceError> {
        match self {
            Self::Whole          => *url=Url::parse (with.ok_or(ReplaceError::PartCannotBeNone)?)?,
            Self::Scheme         => url.set_scheme  (with.ok_or(ReplaceError::PartCannotBeNone)?).map_err(|_| ReplaceError::InvalidScheme)?,
            Self::Username       => url.set_username(with.ok_or(ReplaceError::PartCannotBeNone)?).map_err(|_| ReplaceError::CannotSetUsername)?,
            Self::Password       => url.set_password(with).map_err(|_| ReplaceError::CannotSetPassword)?,
            Self::Host           => url.set_host    (with)?,
            Self::Subdomain      => {
                let mut new_domain=with.ok_or(ReplaceError::PartCannotBeNone)?.to_string();
                new_domain.push('.');
                new_domain.push_str(&Self::NotSubdomain.get_from(url).ok_or(ReplaceError::HostIsNotADomain)?);
                url.set_host(Some(&new_domain))?;
            },
            Self::NotSubdomain   => {
                let mut new_domain=Self::Subdomain.get_from(url).ok_or(ReplaceError::HostIsNotADomain)?.to_string();
                new_domain.push('.');
                new_domain.push_str(with.ok_or(ReplaceError::PartCannotBeNone)?);
                url.set_host(Some(&new_domain))?;
            },
            Self::Domain         => url.set_host    (with)?,
            Self::Port           => url.set_port    (with.map(|x| x.parse().map_err(|_| ReplaceError::InvalidPort)).transpose()?).map_err(|_| ReplaceError::CannotSetPort)?,
            Self::PathSegment(n) => match with {
                Some(with) => url.set_path(&url.path().split('/').enumerate().map(|(i, x)| if i==*n {with} else {x}).collect::<Vec<_>>().join("/")),
                None => Err(ReplaceError::PartCannotBeNone)?
            },
            Self::Path           => url.set_path    (with.ok_or(ReplaceError::PartCannotBeNone)?),
            Self::Query          => url.set_query   (with),
            Self::Fragment       => url.set_fragment(with)
        }
        Ok(())
    }
}

/// The method [`crate::rules::conditions::Condition::DomainCondition`] should use.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum DomainConditionRule {
    /// Use the specified URL. If the source of the URL being cleaned is a link on a webpage then this should contain the URL of that webpage.
    #[serde(serialize_with = "serialize_url", deserialize_with = "deserialize_url")]
    Url(Url),
    /// Makes [`crate::rules::conditions::Condition::DomainCondition`] always pass.
    Always,
    /// Makes [`crate::rules::conditions::Condition::DomainCondition`] always fail.
    Never,
    /// Similar to [`DomainConditionRule::Url`] except the contained URL would always be the URL being cleaned.
    /// This is the default as I assume it's the one that works most of the time.
    #[default]
    UseUrlBeingCleaned
}

fn deserialize_url<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Url, D::Error> {
    let x: &'de str=Deserialize::deserialize(deserializer)?;
    Url::parse(x).map_err(|_| D::Error::custom(format!("Invalid URL pattern: {x:?}.")))
}
fn serialize_url<S: Serializer>(value: &Url, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(value.as_str())
}

impl FromStr for DomainConditionRule {
    type Err=ParseError;

    fn from_str(x: &str) -> Result<Self, Self::Err> {
        Ok(match x {
            "Always" => DomainConditionRule::Always,
            "Never" => DomainConditionRule::Never,
            "UseUrlBeingCleaned" => DomainConditionRule::UseUrlBeingCleaned,
            _ => DomainConditionRule::Url(Url::parse(x)?)
        })
    }
}

impl ToString for DomainConditionRule {
    fn to_string(&self) -> String {
        match self {
            Self::Url(url) => url.to_string(),
            Self::Always => "Always".to_string(),
            Self::Never => "Never".to_string(),
            Self::UseUrlBeingCleaned => "UseUrlBeingCleaned".to_string()
        }
    }
}

/// An enum that, if I've done my job properly, contains details on any possible error that can happen when cleaning a URL.
/// Except for if a [`crate::rules::mappers::Mapper::ExpandShortLink`] response can't be cached. That error is ignored pending a version of [`Result`] that can handle partial errors.
/// Not only is it a recoverable error, it's an error that doesn't need to be recovered from.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum CleaningError {
    /// There was an error getting the rules.
    #[error("There was an error getting the rules.")]
    GetRulesError(#[from] crate::rules::GetRulesError),
    /// There was an error executing a rule.
    #[error("There was an error executing a rule.")]
    RuleError(#[from] crate::rules::RuleError),
    /// There was an error parsing the URL.
    #[error("There was an error parsing the URL.")]
    UrlParseError(#[from] ParseError),
    /// IO error.
    #[error("IO error")]
    IoError(#[from] IoError)
}

/// The location of a string. Used by [`crate::rules::conditions::Condition::UrlPartContains`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum StringLocation {
    /// [`str::contains`].
    #[default]
    Anywhere,
    /// [`str::starts_with`].
    Start,
    /// [`str::ends_with`].
    End,
    /// `str::get(start..).handle_error().substr.starts_with(...)`.
    StartsAt(usize),
    /// `str::get(..end).handle_error().substr.ends_with(...)`.
    EndsAt(usize),
    /// `str::get(start..end).handle_error()==...`.
    RangeIs {
        /// The start of the range to check.
        start: usize,
        /// The end of the range to check.
        end: usize
    },
    /// `str::get(start.end).handle_error().substr.contains(...)`
    RangeHas {
        /// The start of the range to check.
        start: usize,
        /// The end of the range to check.
        end: usize
    },
    /// `str::get(start..).handle_error().substr.contains(...)`.
    After(usize),
    /// `str::get(..end).handle_error().substr.contains(...)`.
    Before(usize)
}

impl StringLocation {
    /// Ceck if `needle` exists in `haystack` according to `self`'s rules.
    pub fn satisfied_by(&self, haystack: &str, needle: &str) -> Result<bool, StringError> {
        Ok(match self {
            Self::Anywhere             => haystack.contains   (needle),
            Self::Start                => haystack.starts_with(needle),
            Self::End                  => haystack.ends_with  (needle),
            Self::StartsAt(start     ) => haystack.get(*start..    ).ok_or(StringError::InvalidSlice)?.starts_with(needle),
            Self::EndsAt  (       end) => haystack.get(      ..*end).ok_or(StringError::InvalidSlice)?.ends_with  (needle),
            Self::RangeIs {start, end} => haystack.get(*start..*end).ok_or(StringError::InvalidSlice)?==needle,
            Self::RangeHas{start, end} => haystack.get(*start..*end).ok_or(StringError::InvalidSlice)?.contains(needle),
            Self::After   (start     ) => haystack.get(*start..    ).ok_or(StringError::InvalidSlice)?.contains(needle),
            Self::Before  (       end) => haystack.get(      ..*end).ok_or(StringError::InvalidSlice)?.contains(needle)
        })
    }
}

/// Where and how to modify a string. Used by [`crate::rules::mappers::Mapper::ModifyUrlPart`].
#[derive(Debug, Clone,Serialize, Deserialize, PartialEq, Eq)]
pub enum StringModification {
    /// Append the contained string to the end of the part.
    Append(String),
    /// Prepend the contained string to the beginning of the part.
    Prepend(String),
    /// Replace all instances of `find` with `replace`.
    Replace{
        /// The value to look for.
        find: String,
        /// The value to replace with.
        replace: String
    },
    /// Replace the specified range with `replace`.
    ReplaceAt{
        /// The start of the range to replace.
        start: usize,
        /// The end of the range to replace.
        end: usize,
        /// The value to replace the range with.
        replace: String
    }
}

impl StringModification {
    /// Apply the modification in-place.
    pub fn apply(&self, to: &mut String) -> Result<(), StringError> {
        match self {
            Self::Append(value)                  => to.push_str(value),
            Self::Prepend(value)                 => {let mut ret=value.to_string(); ret.push_str(to); *to=ret;},
            Self::Replace{find, replace}         => {*to=to.replace(find, replace);},
            Self::ReplaceAt{start, end, replace} => {
                let mut ret=to.get(..*start).ok_or(StringError::InvalidSlice)?.to_string();
                ret.push_str(replace);
                ret.push_str(to.get(*end..).ok_or(StringError::InvalidSlice)?);
                *to=ret;
            }
        };
        Ok(())
    }
}

/// The enum of all possible errors that can happen when using `StringModification`.
#[derive(Debug, Clone, Error)]
pub enum StringError {
    /// The requested slice either was not on a UTF-8 boundary or was out of bounds.
    #[error("The requested slice either was not on a UTF-8 boundary or was out of bounds.")]
    InvalidSlice
}

#[cfg(test)]
mod tests {
    use super::*;

    const fn passes(x: bool) -> bool {x}
    const fn fails(x: bool) -> bool {!x}

    #[test]
    fn url_part_path_segment_replace() {
        let mut url=Url::parse("https://example.com/a/b/c/d").unwrap();
        assert!(UrlPart::PathSegment(2).replace_with(&mut url, Some("e")).is_ok());
        assert_eq!(url.path(), "/a/e/c/d");
    }

    #[test]
    fn string_location_anywhere() {
        assert!(StringLocation::Anywhere.satisfied_by("abcdef", "cde").is_ok_and(passes));
        assert!(StringLocation::Anywhere.satisfied_by("abcdef", "efg").is_ok_and(fails));
    }

    #[test]
    fn string_location_start() {
        assert!(StringLocation::Start.satisfied_by("abcdef", "abc").is_ok_and(passes));
        assert!(StringLocation::Start.satisfied_by("abcdef", "bcd").is_ok_and(fails));
    }

    #[test]
    fn string_location_end() {
        assert!(StringLocation::End.satisfied_by("abcdef", "def").is_ok_and(passes));
        assert!(StringLocation::End.satisfied_by("abcdef", "cde").is_ok_and(fails));
    }

    #[test]
    fn string_location_starts_at() {
        assert!(StringLocation::StartsAt(0).satisfied_by("abcdef", "abc").is_ok_and(passes));
        assert!(StringLocation::StartsAt(1).satisfied_by("abcdef", "bcd").is_ok_and(passes));
        assert!(StringLocation::StartsAt(5).satisfied_by("abcdef", "f"  ).is_ok_and(passes));
        assert!(StringLocation::StartsAt(0).satisfied_by("abcdef", "bcd").is_ok_and(fails));
        assert!(StringLocation::StartsAt(1).satisfied_by("abcdef", "cde").is_ok_and(fails));
        assert!(StringLocation::StartsAt(5).satisfied_by("abcdef", "def").is_ok_and(fails));
    }

    #[test]
    fn string_location_ends_at() {
        assert!(StringLocation::EndsAt(3).satisfied_by("abcdef", "abc").is_ok_and(passes));
        assert!(StringLocation::EndsAt(4).satisfied_by("abcdef", "bcd").is_ok_and(passes));
        assert!(StringLocation::EndsAt(6).satisfied_by("abcdef", "def").is_ok_and(passes));
        assert!(StringLocation::EndsAt(6).satisfied_by("abcdef", "f"  ).is_ok_and(passes));
        assert!(StringLocation::EndsAt(3).satisfied_by("abcdef", "bcd").is_ok_and(fails));
        assert!(StringLocation::EndsAt(4).satisfied_by("abcdef", "cde").is_ok_and(fails));
    }

    #[test]
    fn string_location_range_is() {
        assert!(StringLocation::RangeIs{start: 0, end: 3}.satisfied_by("abcdef", "abc"   ).is_ok_and(passes));
        assert!(StringLocation::RangeIs{start: 1, end: 4}.satisfied_by("abcdef", "bcd"   ).is_ok_and(passes));
        assert!(StringLocation::RangeIs{start: 0, end: 6}.satisfied_by("abcdef", "abcdef").is_ok_and(passes));
        assert!(StringLocation::RangeIs{start: 5, end: 6}.satisfied_by("abcdef", "f"     ).is_ok_and(passes));
        assert!(StringLocation::RangeIs{start: 6, end: 7}.satisfied_by("abcdef", "f"     ).is_err());
        assert!(StringLocation::RangeIs{start: 7, end: 8}.satisfied_by("abcdef", "f"     ).is_err());
    }

    #[test]
    fn string_location_range_has() {
        assert!(StringLocation::RangeHas{start: 0, end: 1}.satisfied_by("abcdef", "a"   ).is_ok_and(passes));
        assert!(StringLocation::RangeHas{start: 0, end: 2}.satisfied_by("abcdef", "a"   ).is_ok_and(passes));
        assert!(StringLocation::RangeHas{start: 0, end: 6}.satisfied_by("abcdef", "bcde").is_ok_and(passes));
        assert!(StringLocation::RangeHas{start: 1, end: 6}.satisfied_by("abcdef", "a"   ).is_ok_and(fails));
        assert!(StringLocation::RangeHas{start: 0, end: 7}.satisfied_by("abcdef", ""    ).is_err());
    }

    #[test]
    fn string_location_after() {
        assert!(StringLocation::After(0).satisfied_by("abcdef", "abcdef").is_ok_and(passes));
        assert!(StringLocation::After(1).satisfied_by("abcdef", "bcdef" ).is_ok_and(passes));
        assert!(StringLocation::After(1).satisfied_by("abcdef", "1"     ).is_ok_and(fails));
        assert!(StringLocation::After(6).satisfied_by("abcdef", "f"     ).is_ok_and(fails));
        assert!(StringLocation::After(7).satisfied_by("abcdef", ""      ).is_err());
    }

    #[test]
    fn string_location_before() {
        assert!(StringLocation::Before(0).satisfied_by("abcdef", ""   ).is_ok_and(passes));
        assert!(StringLocation::Before(1).satisfied_by("abcdef", "a"  ).is_ok_and(passes));
        assert!(StringLocation::Before(6).satisfied_by("abcdef", "a"  ).is_ok_and(passes));
        assert!(StringLocation::Before(4).satisfied_by("abcdef", "def").is_ok_and(fails ));
        assert!(StringLocation::Before(7).satisfied_by("abcdef", "a"  ).is_err());
    }

    #[test]
    fn string_modification_append() {
        let mut x = "abcdef".to_string();
        assert!(StringModification::Append("ghi".to_string()).apply(&mut x).is_ok());
        assert_eq!(&x, "abcdefghi");
    }

    #[test]
    fn string_modification_prepend() {
        let mut x = "abcdef".to_string();
        assert!(StringModification::Prepend("ghi".to_string()).apply(&mut x).is_ok());
        assert_eq!(&x, "ghiabcdef");
    }

    #[test]
    fn string_modification_replace() {
        let mut x = "abcabc".to_string();
        assert!(StringModification::Replace{find: "ab".to_string(), replace: "xy".to_string()}.apply(&mut x).is_ok());
        assert_eq!(&x, "xycxyc");
    }

    #[test]
    fn string_modification_replace_at() {
        let mut x = "abcdef".to_string();
        assert!(StringModification::ReplaceAt{start: 6, end: 7, replace: "g".to_string()}.apply(&mut x).is_err());
        assert_eq!(&x, "abcdef");
        assert!(StringModification::ReplaceAt{start: 1, end: 4, replace: "...".to_string()}.apply(&mut x).is_ok());
        assert_eq!(&x, "a...ef");
    }
}
