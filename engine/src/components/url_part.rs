//! [`UrlPart`].

#![allow(unused_assignments, reason = "False positive.")]

use std::ops::Bound;

use crate::prelude::*;

/// A common API for getting and setting various parts of [`BetterUrl`]s.
///
/// For most parts, setting a URL's part to a value then getting that same part returns the same value.
///
/// Exceptions include setting part segments to values containing the split, `After`/`Before`/`Next` variants always returning [`None`], and probably some other things. I'll fix this doc later.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub enum UrlPart {
    /// [`BetterUrl::as_str`] + [`BetterUrl::new`].
    Whole,



    /// [`BetterUrl::scheme`] and [`BetterUrl::set_scheme`].
    Scheme,



    /// [`BetterUrl::userinfo`] and [`BetterUrl::set_userinfo`].
    Userinfo,
    /// [`BetterUrl::username`] and [`BetterUrl::set_username`].
    Username,
    /// [`BetterUrl::password`] and [`BetterUrl::set_password`].
    Password,



    /// [`BetterUrl::host`] and [`BetterUrl::set_host`].
    Host,



    /// [`BetterUrl::domain_prefix`] + [`DomainSegments::decode`] and [`BetterUrl::set_domain_prefix`].
    DomainPrefix,
    /// [`BetterUrl::domain_middle`] + [`DomainSegment::decode`] and [`BetterUrl::set_domain_middle`].
    DomainMiddle,
    /// [`BetterUrl::domain_suffix`] + [`DomainSegments::decode`] and [`BetterUrl::set_domain_suffix`].
    DomainSuffix,
    /// [`BetterUrl::domain_origin`] + [`DomainSegments::decode`] and [`BetterUrl::set_domain_origin`].
    DomainOrigin,
    /// [`BetterUrl::domain_labels`] + [`DomainSegments::decode`] and [`BetterUrl::set_domain_labels`].
    DomainLabels,
    /// [`BetterUrl::domain_normal`] + [`DomainSegments::decode`].
    /// # Errors
    /// Trying to set this part returns [`SetUrlPartError::CantSetDomainNormal`].
    DomainNormal,

    /// [`BetterUrl::domain_segment`] + [`DomainSegment::decode`] and [`BetterUrl::set_domain_segment`].
    DomainSegment(isize),
    /// [`BetterUrl::domain_prefix_segment`] + [`DomainSegment::decode`] and [`BetterUrl::set_domain_prefix_segment`].
    DomainPrefixSegment(isize),
    /// [`BetterUrl::domain_suffix_segment`] + [`DomainSegment::decode`] and [`BetterUrl::set_domain_suffix_segment`].
    DomainSuffixSegment(isize),
    /// [`BetterUrl::domain_origin_segment`] + [`DomainSegment::decode`] and [`BetterUrl::set_domain_origin_segment`].
    DomainOriginSegment(isize),
    /// [`BetterUrl::domain_normal_segment`] + [`DomainSegment::decode`] and [`BetterUrl::set_domain_normal_segment`].
    DomainNormalSegment(isize),

    /// [`BetterUrl::domain_segment_range`] + [`DomainSegment::decode`] and [`BetterUrl::set_domain_segment_range`].
    DomainSegmentRange {
        /// The start of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        start: Bound<isize>,
        /// The end of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        end: Bound<isize>
    },
    /// [`BetterUrl::domain_prefix_segment_range`] + [`DomainSegment::decode`] and [`BetterUrl::set_domain_prefix_segment_range`].
    DomainPrefixSegmentRange {
        /// The start of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        start: Bound<isize>,
        /// The end of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        end: Bound<isize>
    },
    /// [`BetterUrl::domain_suffix_segment_range`] + [`DomainSegment::decode`] and [`BetterUrl::set_domain_suffix_segment_range`].
    DomainSuffixSegmentRange {
        /// The start of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        start: Bound<isize>,
        /// The end of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        end: Bound<isize>
    },
    /// [`BetterUrl::domain_origin_segment_range`] + [`DomainSegment::decode`] and [`BetterUrl::set_domain_origin_segment_range`].
    DomainOriginSegmentRange {
        /// The start of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        start: Bound<isize>,
        /// The end of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        end: Bound<isize>
    },
    /// [`BetterUrl::domain_normal_segment_range`] + [`DomainSegment::decode`] and [`BetterUrl::set_domain_normal_segment_range`].
    DomainNormalSegmentRange {
        /// The start of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        start: Bound<isize>,
        /// The end of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        end: Bound<isize>
    },



    /// [`BetterUrl::domain_prefix`] + [`DomainSegments::into_inner`].
    /// # Errors
    /// Trying to set this part returns [`SetUrlPartError::CantSetRawDomainPrefix`].
    RawDomainPrefix,
    /// [`BetterUrl::domain_middle`] + [`DomainSegment::into_inner`].
    /// # Errors
    /// Trying to set this part returns [`SetUrlPartError::CantSetRawDomainMiddle`].
    RawDomainMiddle,
    /// [`BetterUrl::domain_suffix`] + [`DomainSegments::into_inner`].
    /// # Errors
    /// Trying to set this part returns [`SetUrlPartError::CantSetRawDomainSuffix`].
    RawDomainSuffix,
    /// [`BetterUrl::domain_origin`] + [`DomainSegments::into_inner`].
    /// # Errors
    /// Trying to set this part returns [`SetUrlPartError::CantSetRawDomainOrigin`].
    RawDomainOrigin,
    /// [`BetterUrl::domain_labels`] + [`DomainSegments::into_inner`].
    /// # Errors
    /// Trying to set this part returns [`SetUrlPartError::CantSetRawDomainLabels`].
    RawDomainLabels,
    /// [`BetterUrl::domain_normal`] + [`DomainSegments::into_inner`].
    /// # Errors
    /// Trying to set this part returns [`SetUrlPartError::CantSetRawDomainNormal`].
    RawDomainNormal,

    /// [`BetterUrl::domain_segment`] + [`DomainSegment::into_inner`].
    /// # Errors
    /// Trying to set this part returns [`SetUrlPartError::CantSetRawDomainSegment`].
    RawDomainSegment(isize),
    /// [`BetterUrl::domain_prefix_segment`] + [`DomainSegment::into_inner`].
    /// # Errors
    /// Trying to set this part returns [`SetUrlPartError::CantSetRawDomainPrefixSegment`].
    RawDomainPrefixSegment(isize),
    /// [`BetterUrl::domain_suffix_segment`] + [`DomainSegment::into_inner`].
    /// # Errors
    /// Trying to set this part returns [`SetUrlPartError::CantSetRawDomainSuffixSegment`].
    RawDomainSuffixSegment(isize),
    /// [`BetterUrl::domain_origin_segment`] + [`DomainSegment::into_inner`].
    /// # Errors
    /// Trying to set this part returns [`SetUrlPartError::CantSetRawDomainOriginSegment`].
    RawDomainOriginSegment(isize),
    /// [`BetterUrl::domain_normal_segment`] + [`DomainSegment::into_inner`].
    /// # Errors
    /// Trying to set this part returns [`SetUrlPartError::CantSetRawDomainNormalSegment`].
    RawDomainNormalSegment(isize),

    /// [`BetterUrl::domain_segment_range`] + [`DomainSegments::into_inner`].
    /// # Errors
    /// Trying to set this part returns [`SetUrlPartError::CantSetRawDomainSegmentRange`].
    RawDomainSegmentRange {
        /// The start of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        start: Bound<isize>,
        /// The end of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        end: Bound<isize>
    },
    /// [`BetterUrl::domain_prefix_segment_range`] + [`DomainSegments::into_inner`].
    /// # Errors
    /// Trying to set this part returns [`SetUrlPartError::CantSetRawDomainPrefixSegmentRange`].
    RawDomainPrefixSegmentRange {
        /// The start of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        start: Bound<isize>,
        /// The end of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        end: Bound<isize>
    },
    /// [`BetterUrl::domain_suffix_segment_range`] + [`DomainSegments::into_inner`].
    /// # Errors
    /// Trying to set this part returns [`SetUrlPartError::CantSetRawDomainSuffixSegmentRange`].
    RawDomainSuffixSegmentRange {
        /// The start of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        start: Bound<isize>,
        /// The end of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        end: Bound<isize>
    },
    /// [`BetterUrl::domain_origin_segment_range`] + [`DomainSegments::into_inner`].
    /// # Errors
    /// Trying to set this part returns [`SetUrlPartError::CantSetRawDomainOriginSegmentRange`].
    RawDomainOriginSegmentRange {
        /// The start of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        start: Bound<isize>,
        /// The end of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        end: Bound<isize>
    },
    /// [`BetterUrl::domain_normal_segment_range`] + [`DomainSegments::into_inner`].
    /// # Errors
    /// Trying to set this part returns [`SetUrlPartError::CantSetRawDomainNormalSegmentRange`].
    RawDomainNormalSegmentRange {
        /// The start of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        start: Bound<isize>,
        /// The end of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        end: Bound<isize>
    },



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

impl UrlPart {
    /// [`Self::get`], replacing [`None`] with the error [`UrlPartNotFound`].
    /// # Errors
    /// If the call to [`UrlPart::get`] returns [`None`], returns the error [`UrlPartNotFound`].
    pub fn get_some<'a>(&self, url: &'a BetterUrl) -> Result<Cow<'a, str>, UrlPartNotFound> {
        self.get(url).ok_or(UrlPartNotFound)
    }

    /// Gets the value.
    pub fn get<'a>(&self, url: &'a BetterUrl) -> Option<Cow<'a, str>> {
        Some(match self {
            Self::Whole => url.as_str().into(),

            Self::Scheme => url.scheme().into_inner(),

            Self::Userinfo => url.userinfo().into_inner(),
            Self::Username => url.username().into_inner(),
            Self::Password => url.password().into_inner(),

            Self::Host => url.host_str()?.into(),

            Self::DomainPrefix => url.domain_prefix()?.decode(),
            Self::DomainMiddle => url.domain_middle()?.decode(),
            Self::DomainSuffix => url.domain_suffix()?.decode(),
            Self::DomainOrigin => url.domain_origin()?.decode(),
            Self::DomainLabels => url.domain_labels()?.decode(),
            Self::DomainNormal => url.domain_normal()?.decode(),

            Self::DomainSegment      (index) => url.domain_segment       (*index)?.decode(),
            Self::DomainPrefixSegment(index) => url.domain_prefix_segment(*index)?.decode(),
            Self::DomainSuffixSegment(index) => url.domain_suffix_segment(*index)?.decode(),
            Self::DomainOriginSegment(index) => url.domain_origin_segment(*index)?.decode(),
            Self::DomainNormalSegment(index) => url.domain_normal_segment(*index)?.decode(),

            Self::DomainSegmentRange       {start, end} => url.domain_segment_range       ((*start, *end))?.decode(),
            Self::DomainPrefixSegmentRange {start, end} => url.domain_prefix_segment_range((*start, *end))?.decode(),
            Self::DomainSuffixSegmentRange {start, end} => url.domain_suffix_segment_range((*start, *end))?.decode(),
            Self::DomainOriginSegmentRange {start, end} => url.domain_origin_segment_range((*start, *end))?.decode(),
            Self::DomainNormalSegmentRange {start, end} => url.domain_normal_segment_range((*start, *end))?.decode(),

            Self::RawDomainPrefix => url.domain_prefix()?.into_inner(),
            Self::RawDomainMiddle => url.domain_middle()?.into_inner(),
            Self::RawDomainSuffix => url.domain_suffix()?.into_inner(),
            Self::RawDomainOrigin => url.domain_origin()?.into_inner(),
            Self::RawDomainLabels => url.domain_labels()?.into_inner(),
            Self::RawDomainNormal => url.domain_normal()?.into_inner(),

            Self::RawDomainSegment      (index) => url.domain_segment       (*index)?.into_inner(),
            Self::RawDomainPrefixSegment(index) => url.domain_prefix_segment(*index)?.into_inner(),
            Self::RawDomainSuffixSegment(index) => url.domain_suffix_segment(*index)?.into_inner(),
            Self::RawDomainOriginSegment(index) => url.domain_origin_segment(*index)?.into_inner(),
            Self::RawDomainNormalSegment(index) => url.domain_normal_segment(*index)?.into_inner(),

            Self::RawDomainSegmentRange       {start, end} => url.domain_segment_range       ((*start, *end))?.into_inner(),
            Self::RawDomainPrefixSegmentRange {start, end} => url.domain_prefix_segment_range((*start, *end))?.into_inner(),
            Self::RawDomainSuffixSegmentRange {start, end} => url.domain_suffix_segment_range((*start, *end))?.into_inner(),
            Self::RawDomainOriginSegmentRange {start, end} => url.domain_origin_segment_range((*start, *end))?.into_inner(),
            Self::RawDomainNormalSegmentRange {start, end} => url.domain_normal_segment_range((*start, *end))?.into_inner(),

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
    pub fn set(&self, url: &mut BetterUrl, to: Option<&str>) -> Result<bool, SetUrlPartError> {
        Ok(match self {
            Self::Whole => {
                let new = to.ok_or(SetUrlPartError::WholeCantBeNone)?;

                if url == new {
                    return Ok(false);
                }

                let new = url::Url::parse(new)?;

                if *url == new {
                    return Ok(false);
                }

                *url = new.into();

                true
            },

            Self::Scheme => url.set_scheme(to.ok_or(SetUrlPartError::SchemeCantBeNone)?)?,

            Self::Userinfo => url.set_userinfo(to.ok_or(SetUrlPartError::UserinfoCantBeNone)?)?,
            Self::Username => url.set_username(to.ok_or(SetUrlPartError::UsernameCantBeNone)?)?,
            Self::Password => url.set_password(to.ok_or(SetUrlPartError::PasswordCantBeNone)?)?,

            Self::Host => url.set_host(to)?,

            Self::DomainPrefix => url.set_domain_prefix(to)?,
            Self::DomainMiddle => url.set_domain_middle(to)?,
            Self::DomainSuffix => url.set_domain_suffix(to)?,
            Self::DomainOrigin => url.set_domain_origin(to)?,
            Self::DomainLabels => url.set_domain_labels(to.ok_or(SetUrlPartError::DomainLabelsCantBeNone)?)?,
            Self::DomainNormal => url.set_domain_normal(to)?,

            Self::DomainSegment      (n) => url.set_domain_segment       (*n, to)?,
            Self::DomainPrefixSegment(n) => url.set_domain_prefix_segment(*n, to)?,
            Self::DomainSuffixSegment(n) => url.set_domain_suffix_segment(*n, to)?,
            Self::DomainOriginSegment(n) => url.set_domain_origin_segment(*n, to)?,
            Self::DomainNormalSegment(n) => url.set_domain_normal_segment(*n, to)?,

            Self::DomainSegmentRange      {start, end} => url.set_domain_segment_range       ((*start, *end), to)?,
            Self::DomainPrefixSegmentRange{start, end} => url.set_domain_prefix_segment_range((*start, *end), to)?,
            Self::DomainSuffixSegmentRange{start, end} => url.set_domain_suffix_segment_range((*start, *end), to)?,
            Self::DomainOriginSegmentRange{start, end} => url.set_domain_origin_segment_range((*start, *end), to)?,
            Self::DomainNormalSegmentRange{start, end} => url.set_domain_normal_segment_range((*start, *end), to)?,

            Self::RawDomainPrefix => Err(SetUrlPartError::CantSetRawDomainPrefix)?,
            Self::RawDomainMiddle => Err(SetUrlPartError::CantSetRawDomainMiddle)?,
            Self::RawDomainSuffix => Err(SetUrlPartError::CantSetRawDomainSuffix)?,
            Self::RawDomainOrigin => Err(SetUrlPartError::CantSetRawDomainOrigin)?,
            Self::RawDomainLabels => Err(SetUrlPartError::CantSetRawDomainLabels)?,
            Self::RawDomainNormal => Err(SetUrlPartError::CantSetRawDomainNormal)?,

            Self::RawDomainSegment      (_) => Err(SetUrlPartError::CantSetRawDomainSegment)?,
            Self::RawDomainPrefixSegment(_) => Err(SetUrlPartError::CantSetRawDomainPrefixSegment)?,
            Self::RawDomainSuffixSegment(_) => Err(SetUrlPartError::CantSetRawDomainSuffixSegment)?,
            Self::RawDomainOriginSegment(_) => Err(SetUrlPartError::CantSetRawDomainOriginSegment)?,
            Self::RawDomainNormalSegment(_) => Err(SetUrlPartError::CantSetRawDomainNormalSegment)?,

            Self::RawDomainSegmentRange       {..} => Err(SetUrlPartError::CantSetRawDomainSegmentRange)?,
            Self::RawDomainPrefixSegmentRange {..} => Err(SetUrlPartError::CantSetRawDomainPrefixSegmentRange)?,
            Self::RawDomainSuffixSegmentRange {..} => Err(SetUrlPartError::CantSetRawDomainSuffixSegmentRange)?,
            Self::RawDomainOriginSegmentRange {..} => Err(SetUrlPartError::CantSetRawDomainOriginSegmentRange)?,
            Self::RawDomainNormalSegmentRange {..} => Err(SetUrlPartError::CantSetRawDomainNormalSegmentRange)?,

            Self::Port => url.set_port(to.map(|x| x.parse().map_err(|_| SetUrlPartError::InvalidPort)).transpose()?)?,

            Self::Path                     => url.set_path(to.ok_or(SetUrlPartError::PathCantBeNone)?)?,
            Self::PathSegment         (n)  => url.set_path_segment(*n, to)?,
            Self::RawPathSegment      (_)  => Err(SetUrlPartError::CantSetRawPathSegment)?,
            Self::RawPathSegmentRange {..} => Err(SetUrlPartError::CantSetRawPathSegmentRange)?,

            Self::Query                => url.set_query(to)?,
            Self::QueryParam   (param) => url.set_query_param(&param.name, param.index, to.map(Some))?,
            Self::RawQueryParam(_)     => Err(SetUrlPartError::CantSetRawQueryParam)?,

            Self::Fragment                => url.set_fragment(to)?,
            Self::FragmentParam   (param) => url.set_fragment_query_param(&param.name, param.index, to.map(Some))?,
            Self::RawFragmentParam(_)     => Err(SetUrlPartError::CantSetRawFragmentParam)?,
        })
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
    /// Returned when attempting to set a URL's domain labels to [`None`].
    #[error("Attempted to set a URL's domain labels to None.")]
    DomainLabelsCantBeNone,

    /// Returned when attempting to set a [`UrlPart::RawDomainPrefix`].
    #[error("Attempted to set a UrlPart::RawDomainPrefix.")]
    CantSetRawDomainPrefix,
    /// Returned when attempting to set a [`UrlPart::RawDomainMiddle`].
    #[error("Attempted to set a UrlPart::RawDomainMiddle.")]
    CantSetRawDomainMiddle,
    /// Returned when attempting to set a [`UrlPart::RawDomainSuffix`].
    #[error("Attempted to set a UrlPart::RawDomainSuffix.")]
    CantSetRawDomainSuffix,
    /// Returned when attempting to set a [`UrlPart::RawDomainOrigin`].
    #[error("Attempted to set a UrlPart::RawDomainOrigin.")]
    CantSetRawDomainOrigin,
    /// Returned when attempting to set a [`UrlPart::RawDomainLabels`].
    #[error("Attempted to set a UrlPart::RawDomainLabels.")]
    CantSetRawDomainLabels,
    /// Returned when attempting to set a [`UrlPart::RawDomainNormal`].
    #[error("Attempted to set a UrlPart::RawDomainNormal.")]
    CantSetRawDomainNormal,

    /// Returned when attempting to set a [`UrlPart::RawDomainSegment`].
    #[error("Attempted to set a UrlPart::RawDomainSegment.")]
    CantSetRawDomainSegment,
    /// Returned when attempting to set a [`UrlPart::RawDomainPrefixSegment`].
    #[error("Attempted to set a UrlPart::RawDomainPrefixSegment.")]
    CantSetRawDomainPrefixSegment,
    /// Returned when attempting to set a [`UrlPart::RawDomainSuffixSegment`].
    #[error("Attempted to set a UrlPart::RawDomainSuffixSegment.")]
    CantSetRawDomainSuffixSegment,
    /// Returned when attempting to set a [`UrlPart::RawDomainOriginSegment`].
    #[error("Attempted to set a UrlPart::RawDomainOriginSegment.")]
    CantSetRawDomainOriginSegment,
    /// Returned when attempting to set a [`UrlPart::RawDomainNormalSegment`].
    #[error("Attempted to set a UrlPart::RawDomainNormalSegment.")]
    CantSetRawDomainNormalSegment,

    /// Returned when attempting to set a [`UrlPart::RawDomainSegmentRange`].
    #[error("Attempted to set a UrlPart::RawDomainSegmentRange.")]
    CantSetRawDomainSegmentRange,
    /// Returned when attempting to set a [`UrlPart::RawDomainPrefixSegmentRange`].
    #[error("Attempted to set a UrlPart::RawDomainPrefixSegmentRange.")]
    CantSetRawDomainPrefixSegmentRange,
    /// Returned when attempting to set a [`UrlPart::RawDomainSuffixSegmentRange`].
    #[error("Attempted to set a UrlPart::RawDomainSuffixSegmentRange.")]
    CantSetRawDomainSuffixSegmentRange,
    /// Returned when attempting to set a [`UrlPart::RawDomainOriginSegmentRange`].
    #[error("Attempted to set a UrlPart::RawDomainOriginSegmentRange.")]
    CantSetRawDomainOriginSegmentRange,
    /// Returned when attempting to set a [`UrlPart::RawDomainNormalSegmentRange`].
    #[error("Attempted to set a UrlPart::RawDomainNormalSegmentRange.")]
    CantSetRawDomainNormalSegmentRange,

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
