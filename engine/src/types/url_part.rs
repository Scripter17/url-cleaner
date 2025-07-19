//! A common API for getting and setting various parts of [`BetterUrl`]s.

use std::borrow::Cow;
use std::str::FromStr;

use thiserror::Error;
use serde::{Serialize, Deserialize};
#[expect(unused_imports, reason = "Used in a doc comment.")]
use url::Url;

use crate::types::*;
use crate::glue::*;
use crate::util::*;

/// A common API for getting and setting various parts of [`BetterUrl`]s.
///
/// For most parts, setting a URL's part to a value then getting that same part returns the same value.
///
/// Exceptions include setting part segments to values containing the split, `After`/`Before`/`Next` variants always returning [`None`], and probably some other things. I'll fix this doc later.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Suitability)]
#[serde(deny_unknown_fields)]
pub enum UrlPart {
    /// Print debug information about the contained [`Self`].
    #[suitable(never)]
    Debug(Box<Self>),



    /// The whole URL.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    /// assert_eq!(UrlPart::Whole.get(&url), Some("https://example.com/".into()));
    ///
    /// UrlPart::Whole.set(&mut url, Some("https://example2.com")).unwrap();
    /// assert_eq!(UrlPart::Whole.get(&url), Some("https://example2.com/".into()));
    /// assert_eq!(url.as_str(), "https://example2.com/");
    /// ```
    Whole,



    /// [`Url::scheme`] and [`BetterUrl::set_scheme`].
    Scheme,
    /// [`Url::username`] and [`BetterUrl::set_username`].
    Username,
    /// [`Url::password`] and [`BetterUrl::set_password`].
    Password,



    /// [`Url::host`] and [`BetterUrl::set_host`].
    Host,
    /// [`BetterUrl::normalized_host`].
    /// # Errors
    /// Trying to set this part returns [`SetUrlPartError::CannotSetNormalizedHost`].
    NormalizedHost,



    /// [`BetterUrl::domain_segment`] and [`BetterUrl::set_domain_segment`].
    DomainSegment(isize),
    /// [`BetterUrl::subdomain_segment`] and [`BetterUrl::set_subdomain_segment`].
    SubdomainSegment(isize),
    /// [`BetterUrl::domain_suffix_segment`] and [`BetterUrl::set_domain_suffix_segment`].
    DomainSuffixSegment(isize),



    /// [`BetterUrl::domain`] and [`BetterUrl::set_domain`].
    Domain,
    /// [`BetterUrl::subdomain`] and [`BetterUrl::set_subdomain`].
    Subdomain,
    /// [`BetterUrl::reg_domain`] and [`BetterUrl::set_reg_domain`].
    RegDomain,
    /// [`BetterUrl::not_domain_suffix`] and [`BetterUrl::set_not_domain_suffix`].
    NotDomainSuffix,
    /// [`BetterUrl::domain_middle`] and [`BetterUrl::set_domain_middle`].
    DomainMiddle,
    /// [`BetterUrl::domain_suffix`] and [`BetterUrl::set_domain_suffix`].
    DomainSuffix,



    /// [`Url::port`] and [`BetterUrl::set_port`], but using strings.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// assert_eq!(UrlPart::Port.get(&url), None);
    ///
    /// UrlPart::Port.set(&mut url, Some("443")).unwrap();
    /// assert_eq!(UrlPart::Port.get(&url), None);
    ///
    /// UrlPart::Port.set(&mut url, Some("80")).unwrap();
    /// assert_eq!(UrlPart::Port.get(&url), Some("80".into()));
    /// ```
    Port,



    /// [`Url::path`] and [`BetterUrl::set_path`].
    Path,
    /// [`BetterUrl::path_segment`] and [`BetterUrl::set_path_segment`].
    PathSegment(isize),
    /// [`BetterUrl::path_segment`] and [`BetterUrl::set_raw_path_segment`].
    RawPathSegment(isize),
    /// [`BetterUrl::path_segments_str`] and [`BetterUrl::set_path_segments_str`]
    PathSegments,
    /// [`BetterUrl::first_n_path_segments`] and [`BetterUrl::set_first_n_path_segments`].
    FirstNPathSegments(usize),
    /// [`BetterUrl::path_segments_after_first_n`] and [`BetterUrl::set_path_segments_after_first_n`]
    PathSegmentsAfterFirstN(usize),
    /// [`BetterUrl::last_n_path_segments`] and [`BetterUrl::set_last_n_path_segments`].
    LastNPathSegments(usize),
    /// [`BetterUrl::path_segments_before_last_n`] and [`BetterUrl::set_path_segments_before_last_n`].
    PathSegmentsBeforeLastN(usize),



    /// [`Url::query`] and [`BetterUrl::set_query`].
    Query,
    /// [`BetterUrl::query_param`] and [`BetterUrl::set_query_param`]
    QueryParam(QueryParamSelector),
    /// [`BetterUrl::raw_query_param`] and [`BetterUrl::set_raw_query_param`]
    RawQueryParam(QueryParamSelector),



    /// [`Url::fragment`] and [`BetterUrl::set_fragment`].
    Fragment,



    /// Uses [`BetterUrlPosition`]s to get multiple adjacent parts at the same time.
    /// # Errors
    /// Currently cannot set a UrlPart::PositionRange because it's complicated.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::glue::BetterUrlPosition;
    ///
    /// // Note that the `#1` at the end is the fragment, so just getting the query gives the wrong answer.
    /// let url = BetterUrl::parse("https://href.li/?https://example.com/?abc=123&def=456#1").unwrap();
    /// assert_eq!(
    ///     UrlPart::PositionRange {
    ///         start: BetterUrlPosition::BeforeQuery,
    ///         end: BetterUrlPosition::AfterFragment
    ///     }.get(&url),
    ///     Some("https://example.com/?abc=123&def=456#1".into())
    /// );
    /// ```
    PositionRange {
        /// The start of the range to get/set.
        start: BetterUrlPosition,
        /// The end of the range to get/set.
        end: BetterUrlPosition
    }
}

/// Allows getting and setting specific instances of a query parameter.
///
/// For example, it allows getting and setting the second `a` in `https://example.com?a=1&a=2`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(remote = "Self")]
pub struct QueryParamSelector {
    /// The name of the query parameter to get.
    pub name: String,
    /// The index of matching query parameters to get.
    ///
    /// Defaults to `0`.
    #[serde(default, skip_serializing_if = "is_default")]
    pub index: usize
}

string_or_struct_magic!(QueryParamSelector);

impl FromStr for QueryParamSelector {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl From<&str> for QueryParamSelector {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<String> for QueryParamSelector {
    fn from(value: String) -> Self {
        Self {
            name: value,
            index: Default::default()
        }
    }
}

impl UrlPart {
    /// Gets the value.
    pub fn get<'a>(&self, url: &'a BetterUrl) -> Option<Cow<'a, str>> {
        debug!(UrlPart::get, self, url);
        Some(match self {
            Self::Debug(part) => {
                let ret = part.get(url);
                eprintln!("=== UrlPart::Debug ===\nUrlPart: {part:?}\nValue: {ret:?}");
                ret?
            },

            Self::Whole => Cow::Borrowed(url.as_str()),

            Self::Scheme   => Cow::Borrowed(url.scheme()),
            Self::Username => Cow::Borrowed(url.username()),
            Self::Password => Cow::Borrowed(url.password()?),

            Self::Host           => Cow::Borrowed(url.host_str()?),
            Self::NormalizedHost => Cow::Borrowed(url.normalized_host()?),

            Self::DomainSegment      (index) => Cow::Borrowed(url.domain_segment       (*index)?),
            Self::SubdomainSegment   (index) => Cow::Borrowed(url.subdomain_segment    (*index)?),
            Self::DomainSuffixSegment(index) => Cow::Borrowed(url.domain_suffix_segment(*index)?),

            Self::Domain          => Cow::Borrowed(url.domain           ()?),
            Self::Subdomain       => Cow::Borrowed(url.subdomain        ()?),
            Self::RegDomain       => Cow::Borrowed(url.reg_domain       ()?),
            Self::NotDomainSuffix => Cow::Borrowed(url.not_domain_suffix()?),
            Self::DomainMiddle    => Cow::Borrowed(url.domain_middle    ()?),
            Self::DomainSuffix    => Cow::Borrowed(url.domain_suffix    ()?),

            Self::Port => Cow::Owned(url.port()?.to_string()),

            Self::Path => Cow::Borrowed(url.path()),
            Self::PathSegment(index) => Cow::Borrowed(url.path_segment(*index)??),
            Self::RawPathSegment(index) => Cow::Borrowed(url.path_segment(*index)??),
            Self::PathSegments => Cow::Borrowed(url.path_segments_str()?),
            Self::FirstNPathSegments(n) => Cow::Borrowed(url.first_n_path_segments(*n)??),
            Self::PathSegmentsAfterFirstN(n) => Cow::Borrowed(url.path_segments_after_first_n(*n)??),
            Self::LastNPathSegments(n) => Cow::Borrowed(url.last_n_path_segments(*n)??),
            Self::PathSegmentsBeforeLastN(n) => Cow::Borrowed(url.path_segments_before_last_n(*n)??),

            Self::Query => Cow::Borrowed(url.query()?),
            Self::QueryParam   (QueryParamSelector {name, index}) => url.query_param(name, *index)???,
            Self::RawQueryParam(QueryParamSelector {name, index}) => Cow::Borrowed(url.raw_query_param(name, *index)???),

            Self::Fragment => Cow::Borrowed(url.fragment()?),

            Self::PositionRange {start, end} => Cow::Borrowed(&url[start.0..end.0])
        })
    }

    /// Sets the value.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn set(&self, url: &mut BetterUrl, to: Option<&str>) -> Result<(), SetUrlPartError> {
        debug!(UrlPart::set, self, url, to);
        match (self, to) {
            (Self::Debug(part), _) => {
                let old = part.get(url).to_owned();
                eprintln!("=== UrlPart::Debug ===\nUrlPart: {part:?}\nOld value: {old:?}\nNew value: {to:?}");
                part.set(url, to)?;
            },

            (Self::Whole   , Some(to)) => *url=BetterUrl::parse(to)?,
            (Self::Whole   , None    ) => Err(SetUrlPartError::WholeCannotBeNone)?,

            (Self::Scheme  , Some(to)) => url.set_scheme(to)?,
            (Self::Scheme  , None    ) => Err(SetUrlPartError::SchemeCannotBeNone)?,

            (Self::Username, Some(to)) => url.set_username(to)?,
            (Self::Username, None    ) => Err(SetUrlPartError::UsernameCannotBeNone)?,

            (Self::Password, _       ) => url.set_password(to)?,

            (Self::Host , _) => url.set_host(to)?,
            (Self::NormalizedHost, _) => Err(SetUrlPartError::CannotSetNormalizedHost)?,

            (Self::DomainSegment      (n), _) => url.set_domain_segment       (*n, to)?,
            (Self::SubdomainSegment   (n), _) => url.set_subdomain_segment    (*n, to)?,
            (Self::DomainSuffixSegment(n), _) => url.set_domain_suffix_segment(*n, to)?,

            (Self::Domain         , _) => url.set_domain           (to)?,
            (Self::Subdomain      , _) => url.set_subdomain        (to)?,
            (Self::RegDomain      , _) => url.set_reg_domain       (to)?,
            (Self::NotDomainSuffix, _) => url.set_not_domain_suffix(to)?,
            (Self::DomainMiddle   , _) => url.set_domain_middle    (to)?,
            (Self::DomainSuffix   , _) => url.set_domain_suffix    (to)?,

            (Self::Port, _) => url.set_port(to.map(|x| x.parse().map_err(|_| SetUrlPartError::InvalidPort)).transpose()?)?,

            (Self::Path, Some(to)) => url.set_path(to),
            (Self::Path, None    ) => Err(SetUrlPartError::PathCannotBeNone)?,
            (Self::PathSegment(n), _) => url.set_path_segment(*n, to)?,
            (Self::RawPathSegment(n), _) => url.set_raw_path_segment(*n, to)?,
            (Self::PathSegments, Some(to)) => url.set_path_segments_str(to)?,
            (Self::PathSegments, None) => Err(SetUrlPartError::CannotSetPathSegmentsToNone)?,
            (Self::FirstNPathSegments(n), _) => url.set_first_n_path_segments(*n, to)?,
            (Self::PathSegmentsAfterFirstN(n), _) => url.set_path_segments_after_first_n(*n, to)?,
            (Self::LastNPathSegments(n), _) => url.set_last_n_path_segments(*n, to)?,
            (Self::PathSegmentsBeforeLastN(n), _) => url.set_path_segments_before_last_n(*n, to)?,

            (Self::Query, _) => url.set_query(to),
            (Self::QueryParam   (QueryParamSelector {name, index}), _) => url.set_query_param(name, *index, to.map(Some))?,
            (Self::RawQueryParam(QueryParamSelector {name, index}), _) => url.set_raw_query_param(name, *index, to.map(Some))?,

            (Self::Fragment, _) => url.set_fragment(to),

            (Self::PositionRange {..}, _) => Err(SetUrlPartError::CannotSetPositionRange)?
        }
        Ok(())
    }
}

/// The enum of errors [`UrlPart::set`] can return.
#[derive(Debug, Error)]
pub enum SetUrlPartError {
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)] UrlParseError(#[from] url::ParseError),
    /// Returned when attempting to set [`UrlPart::Whole`] to [`None`].
    #[error("Attempted to set a whole URL to None.")]
    WholeCannotBeNone,

    // Pre-host stuff.

    /// Returned when attempting to set a URL's scheme to [`None`].
    #[error("Attempted to set a URL's scheme to None.")]
    SchemeCannotBeNone,
    /// Returned when a [`SetSchemeError`] is encountered.
    #[error(transparent)]
    SetSchemeError(#[from] SetSchemeError),
    /// Returned when attempting to set a URL's username to [`None`].
    #[error("Attempted to set a URL's username to None.")]
    UsernameCannotBeNone,
    /// Returned when a [`SetUsernameError`] is encountered.
    #[error(transparent)]
    SetUsernameError(#[from] SetUsernameError),
    /// Returned when a [`SetPasswordError`] is encountered.
    #[error(transparent)]
    SetPasswordError(#[from] SetPasswordError),

    // Host stuff.

    /// Returned when a [`SetHostError`] is encountered.
    #[error(transparent)] SetHostError(#[from] SetHostError),
    /// Returned when a [`SetIpHostError`] is encountered.
    #[error(transparent)] SetIpHostError(#[from] SetIpHostError),
    /// Returned when a [`SetSubdomainError)`] is encountered.
    #[error(transparent)] SetSubdomainError(#[from] SetSubdomainError),
    /// Returned when a [`SetDomainError)`] is encountered.
    #[error(transparent)] SetDomainError(#[from] SetDomainError),
    /// Returned when a [`SetNotDomainSuffixError)`] is encountered.
    #[error(transparent)] SetNotDomainSuffixError(#[from] SetNotDomainSuffixError),
    /// Returned when a [`SetDomainMiddleError)`] is encountered.
    #[error(transparent)] SetDomainMiddleError(#[from] SetDomainMiddleError),
    /// Returned when a [`SetRegDomainError)`] is encountered.
    #[error(transparent)] SetRegDomainError(#[from] SetRegDomainError),
    /// Returned when a [`SetDomainSuffixError)`] is encountered.
    #[error(transparent)] SetDomainSuffixError(#[from] SetDomainSuffixError),
    /// Returned when a [`SetDomainSegmentError)`] is encountered.
    #[error(transparent)] SetDomainSegmentError(#[from] SetDomainSegmentError),
    /// Returned when a [`SetSubdomainSegmentError)`] is encountered.
    #[error(transparent)] SetSubdomainSegmentError(#[from] SetSubdomainSegmentError),
    /// Returned when a [`SetDomainSuffixSegmentError)`] is encountered.
    #[error(transparent)] SetDomainSuffixSegmentError(#[from] SetDomainSuffixSegmentError),

    // Post-host stuff.

    /// Returned when attempting to set a port to a value that isn't a number between 0 and 65535 (inclusive).
    #[error("Attempted to set a port to a value that isn't a number between 0 and 65535 (inclusive).")]
    InvalidPort,
    /// Returned when a [`SetPortError`] is encountered.
    #[error(transparent)]
    SetPortError(#[from] SetPortError),

    /// Returned when a [`SetPathSegmentError`] is encountered.
    #[error(transparent)]
    SetPathSegmentError(#[from] SetPathSegmentError),
    /// Returned when a [`SetPathSegmentsStrError`] is encountered.
    #[error(transparent)]
    SetPathSegmentsStrError(#[from] SetPathSegmentsStrError),
    /// Returned when attempting to set a URL's path to [`None`].
    #[error("Attempted to set the URL's path to None.")]
    PathCannotBeNone,
    /// Returned when attempting to set [`UrlPart::PathSegments`] to [`None`].
    ///
    /// URLs with no path segments still have a path, therefore the operation is incoherent.
    #[error("Cannot set path segments to None, even for URLs with no path segments because they still have a path.")]
    CannotSetPathSegmentsToNone,
    /// Returned when a [`SetFirstNPathSegmentsError`] is encountered.
    #[error(transparent)]
    SetFirstNPathSegmentsError(#[from] SetFirstNPathSegmentsError),
    /// Returned when a [`SetPathSegmentsAfterFirstNError`] is encountered.
    #[error(transparent)]
    SetPathSegmentsAfterFirstNError(#[from] SetPathSegmentsAfterFirstNError),
    /// Returned when a [`SetLastNPathSegmentsError`] is encountered.
    #[error(transparent)]
    SetLastNPathSegmentsError(#[from] SetLastNPathSegmentsError),
    /// Returned when a [`SetPathSegmentsBeforeLastNError`] is encountered.
    #[error(transparent)]
    SetPathSegmentsBeforeLastNError(#[from] SetPathSegmentsBeforeLastNError),

    /// Returned when a [`SetQueryParamError)`] is encountered.
    #[error(transparent)]
    SetQueryParamError(#[from] SetQueryParamError),

    /// Returned when attempting to set a [`UrlPart::NormalizedHost`].
    #[error("Attempted to set a UrlPart::NormalizedHost.")]
    CannotSetNormalizedHost,
    /// Currently cannot set a UrlPart::PositionRange because it's complicated.
    #[error("Currently cannot set a UrlPart::PositionRange because it's complicated.")]
    CannotSetPositionRange
}
