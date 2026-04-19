//! [`UrlPart`].

#![allow(unused_assignments, reason = "False positive.")]

use std::ops::Bound;
use std::borrow::Cow;

use thiserror::Error;
use serde::{Serialize, Deserialize};
#[expect(unused_imports, reason = "Used in a doc comment.")]
use url::Url;

use crate::prelude::*;

/// A common API for getting and setting various parts of [`BetterUrl`]s.
///
/// For most parts, setting a URL's part to a value then getting that same part returns the same value.
///
/// Exceptions include setting part segments to values containing the split, `After`/`Before`/`Next` variants always returning [`None`], and probably some other things. I'll fix this doc later.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub enum UrlPart {
    /// Print debug information about the contained [`Self`].
    #[suitable(never)]
    Debug(Box<Self>),



    /// The whole URL.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    /// use better_url::*;
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



    /// [`BetterUrl::userinfo_str`] and [`BetterUrl::set_userinfo`].
    Userinfo,
    /// [`BetterUrl::username_str`] and [`BetterUrl::set_username`].
    Username,
    /// [`BetterUrl::password_str`] and [`BetterUrl::set_password`].
    Password,



    /// [`BetterUrl::host_str`] and [`BetterUrl::set_host`].
    Host,
    /// [`BetterUrl::domain_prefix`] and [`BetterUrl::set_domain_prefix`].
    DomainPrefix,
    /// [`BetterUrl::domain_middle`] and [`BetterUrl::set_domain_middle`].
    DomainMiddle,
    /// [`BetterUrl::domain_suffix`] and [`BetterUrl::set_domain_suffix`].
    DomainSuffix,
    /// [`BetterUrl::domain_origin`] and [`BetterUrl::set_domain_origin`].
    DomainOrigin,
    /// [`BetterUrl::domain_labels`] and [`BetterUrl::set_domain_labels`].
    DomainLabels,
    /// [`BetterUrl::domain_normal`].
    /// # Errors
    /// Trying to set this part returns [`SetUrlPartError::CantSetDomainNormal`].
    DomainNormal,



    /// [`BetterUrl::domain_segment`] and [`BetterUrl::set_domain_segment`].
    DomainSegment(isize),
    /// [`BetterUrl::domain_origin_segment`] and [`BetterUrl::set_domain_origin_segment`].
    DomainOriginSegment(isize),
    /// [`BetterUrl::domain_prefix_segment`] and [`BetterUrl::set_domain_prefix_segment`].
    DomainPrefixSegment(isize),
    /// [`BetterUrl::domain_suffix_segment`] and [`BetterUrl::set_domain_suffix_segment`].
    DomainSuffixSegment(isize),



    /// [`BetterUrl::port_str`] and [`BetterUrl::set_port`], but using strings.
    Port,



    /// [`BetterUrl::path`] and [`BetterUrl::set_path`].
    Path,
    /// [`BetterUrl::path_segment`] and [`BetterUrl::set_path_segment`].
    PathSegment(isize),
    /// [`BetterUrl::path_segment`]
    /// # Errors
    /// Trying to set this part returns the error [`SetUrlPartError::CantSetRawPathSegment`].
    RawPathSegment(isize),
    /// [`BetterUrl::path_segment_range`].
    /// # Errors
    /// Trying to set this part returns the error [`SetUrlPartError::CantSetRawPathSegmentRange`].
    RawPathSegmentRange {
        /// The start of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        start: Bound<isize>,
        /// The end of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        end: Bound<isize>
    },



    /// [`BetterUrl::query_str`] and [`BetterUrl::set_query`].
    Query,
    /// [`BetterUrl::query_param`] and [`BetterUrl::set_query_param`]
    QueryParam(QueryParamSelector),
    /// [`BetterUrl::query_param`].
    /// # Errors
    /// Trying to set this part returns the error [`SetUrlPartError::CantSetRawQueryParam`].
    RawQueryParam(QueryParamSelector),



    /// [`BetterUrl::fragment`] and [`BetterUrl::set_fragment`].
    Fragment,
    /// [`BetterUrl::fragment_query_param`] and [`BetterUrl::set_fragment_query_param`]
    FragmentParam(QueryParamSelector),
    /// [`BetterUrl::fragment_query_param`].
    /// # Errors
    /// Trying to set this part returns the error [`SetUrlPartError::CantSetRawQueryParam`].
    RawFragmentParam(QueryParamSelector),
}

/// Serde helper function.
fn unbounded<T>() -> Bound<T> {Bound::Unbounded}
/// Serde helper function.
fn is_unbounded<T>(x: &Bound<T>) -> bool {matches!(x, Bound::Unbounded)}

impl UrlPart {
    /// Gets the value.
    pub fn get<'a>(&self, url: &'a BetterUrl) -> Option<Cow<'a, str>> {
        Some(match self {
            Self::Debug(part) => {
                let ret = part.get(url);
                eprintln!("=== UrlPart::Debug ===\nUrlPart: {part:?}\nValue: {ret:?}");
                ret?
            },

            Self::Whole => url.as_str().into(),

            Self::Scheme => url.scheme().into_inner(),

            Self::Userinfo => url.userinfo().into_inner(),
            Self::Username => url.username().into_inner(),
            Self::Password => url.password().into_inner(),

            Self::Host         => url.host_str()?.into(),
            Self::DomainPrefix => url.domain_prefix()?.into(),
            Self::DomainMiddle => url.domain_middle()?.into(),
            Self::DomainSuffix => url.domain_suffix()?.into(),
            Self::DomainOrigin => url.domain_origin()?.into(),
            Self::DomainLabels => url.domain_labels()?.into(),
            Self::DomainNormal => url.domain_normal()?.into(),

            Self::DomainSegment      (index) => url.domain_segment       (*index)?.into(),
            Self::DomainOriginSegment(index) => url.domain_origin_segment(*index)?.into(),
            Self::DomainPrefixSegment(index) => url.domain_prefix_segment(*index)?.into(),
            Self::DomainSuffixSegment(index) => url.domain_suffix_segment(*index)?.into(),

            Self::Port => url.port_str()?.into(),

            Self::Path                             => url.path              (              ) .into_inner(),
            Self::PathSegment         (index     ) => url.path_segment      (*index        )?.decode    (),
            Self::RawPathSegment      (index     ) => url.path_segment      (*index        )?.into_inner(),
            Self::RawPathSegmentRange {start, end} => url.path_segment_range((*start, *end))?.into_inner(),

            Self::Query                => url.query().into_inner()?,
            Self::QueryParam   (param) => url.query_param(&param.name, param.index)?.into_value    ()?,
            Self::RawQueryParam(param) => url.query_param(&param.name, param.index)?.into_raw_value()?,

            Self::Fragment                => url.fragment().into_inner()?,
            Self::FragmentParam   (param) => url.fragment_query_param(&param.name, param.index)?.into_value    ()?,
            Self::RawFragmentParam(param) => url.fragment_query_param(&param.name, param.index)?.into_raw_value()?,
        })
    }

    /// Sets the value.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn set(&self, url: &mut BetterUrl, to: Option<&str>) -> Result<(), SetUrlPartError> {
        match self {
            Self::Debug(part) => {
                let old = part.get(url).to_owned();
                eprintln!("=== UrlPart::Debug ===\nUrlPart: {part:?}\nOld value: {old:?}\nNew value: {to:?}");
                part.set(url, to)?;
            },

            Self::Whole    => *url = BetterUrl::parse(to.ok_or(SetUrlPartError::WholeCantBeNone)?)?,

            Self::Scheme   => url.set_scheme(to.ok_or(SetUrlPartError::SchemeCantBeNone)?)?,

            Self::Userinfo => url.set_userinfo(to.ok_or(SetUrlPartError::UserinfoCantBeNone)?)?,
            Self::Username => url.set_username(to.ok_or(SetUrlPartError::UsernameCantBeNone)?)?,
            Self::Password => url.set_password(to.ok_or(SetUrlPartError::PasswordCantBeNone)?)?,

            Self::Host         => url.set_host         (to)?,
            Self::DomainPrefix => url.set_domain_prefix(to)?,
            Self::DomainMiddle => url.set_domain_middle(to)?,
            Self::DomainSuffix => url.set_domain_suffix(to)?,
            Self::DomainOrigin => url.set_domain_origin(to)?,
            Self::DomainLabels => url.set_domain_labels(to)?,
            Self::DomainNormal => Err(SetUrlPartError::CantSetDomainNormal)?,

            Self::DomainSegment      (n) => url.set_domain_segment       (*n, to)?,
            Self::DomainOriginSegment(n) => url.set_domain_origin_segment(*n, to)?,
            Self::DomainPrefixSegment(n) => url.set_domain_prefix_segment(*n, to)?,
            Self::DomainSuffixSegment(n) => url.set_domain_suffix_segment(*n, to)?,

            Self::Port => url.set_port(to.map(|x| x.parse().map_err(|_| SetUrlPartError::InvalidPort)).transpose()?)?,

            Self::Path                     => url.set_path(to.ok_or(SetUrlPartError::PathCantBeNone)?)?,
            Self::PathSegment         (n)  => url.set_path_segment(*n, to)?,
            Self::RawPathSegment      (_)  => Err(SetUrlPartError::CantSetRawPathSegment)?,
            Self::RawPathSegmentRange {..} => Err(SetUrlPartError::CantSetRawPathSegmentRange)?,

            Self::Query                => {url.set_query(to)?;},
            Self::QueryParam   (param) => {url.set_query_param(&param.name, param.index, to.map(Some))?;},
            Self::RawQueryParam(_)     => Err(SetUrlPartError::CantSetRawQueryParam)?,

            Self::Fragment                => {url.set_fragment(to)?;},
            Self::FragmentParam   (param) => {url.set_fragment_query_param(&param.name, param.index, to.map(Some))?;},
            Self::RawFragmentParam(_)     => Err(SetUrlPartError::CantSetRawFragmentParam)?,
        }

        Ok(())
    }
}

/// The enum of errors [`UrlPart::set`] can return.
#[derive(Debug, Error)]
pub enum SetUrlPartError {
    ///[`url::ParseError`].
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when attempting to set [`UrlPart::Whole`] to [`None`].
    #[error("Attempted to set a whole URL to None.")]
    WholeCantBeNone,



    /// Returned when attempting to set a URL's scheme to [`None`].
    #[error("Attempted to set a URL's scheme to None.")]
    SchemeCantBeNone,
    ///[`SetSchemeError`].
    #[error(transparent)]
    SetSchemeError(#[from] SetSchemeError),



    /// Returned when attempting to set a URL's userinfo to [`None`].
    #[error("Attempted to set a URL's userinfo to None.")]
    UserinfoCantBeNone,
    ///[`SetUserinfoError`].
    #[error(transparent)]
    SetUserinfoError(#[from] SetUserinfoError),
    /// Returned when attempting to set a URL's username to [`None`].
    #[error("Attempted to set a URL's username to None.")]
    UsernameCantBeNone,
    ///[`SetUsernameError`].
    #[error(transparent)]
    SetUsernameError(#[from] SetUsernameError),
    /// Returned when attempting to set a URL's password to [`None`].
    #[error("Attempted to set a URL's password to None.")]
    PasswordCantBeNone,
    ///[`SetPasswordError`].
    #[error(transparent)]
    SetPasswordError(#[from] SetPasswordError),



    ///[`SetHostError`].
    #[error(transparent)]
    SetHostError(#[from] SetHostError),
    ///[`SetDomainError`].
    #[error(transparent)]
    SetDomainError(#[from] SetDomainError),
    /// Returned when attempting to set a [`UrlPart::DomainNormal`].
    #[error("Attempted to set a UrlPart::DomainNormal.")]
    CantSetDomainNormal,



    /// Returned when attempting to set a port to a value that isn't a number between 0 and 65535 (inclusive).
    #[error("Attempted to set a port to a value that isn't a number between 0 and 65535 (inclusive).")]
    InvalidPort,
    ///[`SetPortError`].
    #[error(transparent)]
    SetPortError(#[from] SetPortError),



    /// Returned when attempting to set a URL's path to [`None`].
    #[error("Attempted to set a URL's path to None.")]
    PathCantBeNone,
    ///[`SetPathError`].
    #[error(transparent)]
    SetPathError(#[from] SetPathError),
    /// Returned when attempting to set a [`UrlPart::RawPathSegment`].
    #[error("Attempted to set a UrlPart::RawPathSegment.")]
    CantSetRawPathSegment,
    /// Returned when attempting to set a [`UrlPart::RawPathSegmentRange`].
    #[error("Attempted to set a UrlPart::RawPathSegmentRange.")]
    CantSetRawPathSegmentRange,


    ///[`SetQueryError`].
    #[error(transparent)]
    SetQueryError(#[from] SetQueryError),
    /// Returned when attempting to set a [`UrlPart::RawQueryParam`].
    #[error("Attempted to set a UrlPart::RawQueryParam.")]
    CantSetRawQueryParam,



    ///[`SetFragmentError`].
    #[error(transparent)]
    SetFragmentError(#[from] SetFragmentError),
    /// Returned when attempting to set a [`UrlPart::RawFragmentParam`].
    #[error("Attempted to set a UrlPart::RawFragmentParam.")]
    CantSetRawFragmentParam,
}
