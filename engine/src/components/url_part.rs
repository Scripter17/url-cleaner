//! [`UrlPart`].

#![allow(unused_assignments, reason = "False positive.")]

use std::ops::Bound;

use crate::prelude::*;

/// A common API for getting various parts of [`BetterUrl`]s.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub enum UrlPart {
    /// [`BetterUrl::as_str`].
    Whole,



    /// [`BetterUrl::scheme_str`].
    Scheme,



    /// [`BetterUrl::userinfo_str`].
    Userinfo,
    /// [`BetterUrl::username_str`].
    Username,
    /// [`BetterUrl::password_str`].
    Password,



    /// [`BetterUrl::host_str`].
    Host,



    /** [`BetterUrl::domain_prefix_str`]. **/ DomainPrefix,
    /** [`BetterUrl::domain_middle_str`]. **/ DomainMiddle,
    /** [`BetterUrl::domain_suffix_str`]. **/ DomainSuffix,
    /** [`BetterUrl::domain_origin_str`]. **/ DomainOrigin,
    /** [`BetterUrl::domain_labels_str`]. **/ DomainLabels,
    /** [`BetterUrl::domain_normal_str`]. **/ DomainNormal,

    /** [`BetterUrl::domain_segment_str`].        **/ DomainSegment(isize),
    /** [`BetterUrl::domain_prefix_segment_str`]. **/ DomainPrefixSegment(isize),
    /** [`BetterUrl::domain_suffix_segment_str`]. **/ DomainSuffixSegment(isize),
    /** [`BetterUrl::domain_origin_segment_str`]. **/ DomainOriginSegment(isize),
    /** [`BetterUrl::domain_normal_segment_str`]. **/ DomainNormalSegment(isize),

    /// [`BetterUrl::domain_range_str`].
    DomainSegmentRange {
        /// The start of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        start: Bound<isize>,
        /// The end of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        end: Bound<isize>
    },
    /// [`BetterUrl::domain_prefix_range_str`].
    DomainPrefixSegmentRange {
        /// The start of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        start: Bound<isize>,
        /// The end of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        end: Bound<isize>
    },
    /// [`BetterUrl::domain_suffix_range_str`].
    DomainSuffixSegmentRange {
        /// The start of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        start: Bound<isize>,
        /// The end of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        end: Bound<isize>
    },
    /// [`BetterUrl::domain_origin_range_str`].
    DomainOriginSegmentRange {
        /// The start of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        start: Bound<isize>,
        /// The end of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        end: Bound<isize>
    },
    /// [`BetterUrl::domain_normal_range_str`].
    DomainNormalSegmentRange {
        /// The start of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        start: Bound<isize>,
        /// The end of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        end: Bound<isize>
    },



    /** [`BetterUrl::domain_prefix`] + [`DomainSegments::decode`]. **/ DecodedDomainPrefix,
    /** [`BetterUrl::domain_middle`] + [`DomainSegment::decode`].  **/ DecodedDomainMiddle,
    /** [`BetterUrl::domain_suffix`] + [`DomainSegments::decode`]. **/ DecodedDomainSuffix,
    /** [`BetterUrl::domain_origin`] + [`DomainSegments::decode`]. **/ DecodedDomainOrigin,
    /** [`BetterUrl::domain_labels`] + [`DomainSegments::decode`]. **/ DecodedDomainLabels,
    /** [`BetterUrl::domain_normal`] + [`DomainSegments::decode`]. **/ DecodedDomainNormal,

    /** [`BetterUrl::domain_segment`]        + [`DomainSegment::decode`]. **/ DecodedDomainSegment      (isize),
    /** [`BetterUrl::domain_prefix_segment`] + [`DomainSegment::decode`]. **/ DecodedDomainPrefixSegment(isize),
    /** [`BetterUrl::domain_suffix_segment`] + [`DomainSegment::decode`]. **/ DecodedDomainSuffixSegment(isize),
    /** [`BetterUrl::domain_origin_segment`] + [`DomainSegment::decode`]. **/ DecodedDomainOriginSegment(isize),
    /** [`BetterUrl::domain_normal_segment`] + [`DomainSegment::decode`]. **/ DecodedDomainNormalSegment(isize),

    /// [`BetterUrl::domain_range`] + [`DomainSegments::decode`].
    DecodedDomainSegmentRange {
        /// The start of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        start: Bound<isize>,
        /// The end of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        end: Bound<isize>
    },
    /// [`BetterUrl::domain_prefix_range`] + [`DomainSegments::decode`].
    DecodedDomainPrefixSegmentRange {
        /// The start of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        start: Bound<isize>,
        /// The end of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        end: Bound<isize>
    },
    /// [`BetterUrl::domain_suffix_range`] + [`DomainSegments::decode`].
    DecodedDomainSuffixSegmentRange {
        /// The start of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        start: Bound<isize>,
        /// The end of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        end: Bound<isize>
    },
    /// [`BetterUrl::domain_origin_range`] + [`DomainSegments::decode`].
    DecodedDomainOriginSegmentRange {
        /// The start of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        start: Bound<isize>,
        /// The end of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        end: Bound<isize>
    },
    /// [`BetterUrl::domain_normal_range`] + [`DomainSegments::decode`].
    DecodedDomainNormalSegmentRange {
        /// The start of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        start: Bound<isize>,
        /// The end of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        end: Bound<isize>
    },



    /// [`BetterUrl::port_str`].
    Port,



    /// [`BetterUrl::path_str`].
    Path,
    /// [`BetterUrl::path_segment`].
    PathSegment(isize),
    /// [`BetterUrl::path_segment_str`]
    RawPathSegment(isize),
    /// [`BetterUrl::path_segment_range_str`].
    RawPathSegmentRange {
        /// The start of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        start: Bound<isize>,
        /// The end of the range.
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        end: Bound<isize>
    },



    /// [`BetterUrl::query_str`].
    Query,
    /// [`BetterUrl::query_param`].
    QueryParam(QueryParamSelector),
    /// [`BetterUrl::query_param`].
    RawQueryParam(QueryParamSelector),



    /// [`BetterUrl::fragment_str`].
    Fragment,
    /// [`BetterUrl::fragment_query_param`].
    FragmentParam(QueryParamSelector),
    /// [`BetterUrl::fragment_query_param`].
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

            Self::Scheme => url.scheme_str().into(),

            Self::Userinfo => url.userinfo_str().into(),
            Self::Username => url.username_str().into(),
            Self::Password => url.password_str().into(),

            Self::Host => url.host_str()?.into(),

            Self::DomainPrefix => url.domain_prefix_str()?.into(),
            Self::DomainMiddle => url.domain_middle_str()?.into(),
            Self::DomainSuffix => url.domain_suffix_str()?.into(),
            Self::DomainOrigin => url.domain_origin_str()?.into(),
            Self::DomainLabels => url.domain_labels_str()?.into(),
            Self::DomainNormal => url.domain_normal_str()?.into(),

            Self::DomainSegment      (index) => url.domain_segment_str       (*index)?.into(),
            Self::DomainPrefixSegment(index) => url.domain_prefix_segment_str(*index)?.into(),
            Self::DomainSuffixSegment(index) => url.domain_suffix_segment_str(*index)?.into(),
            Self::DomainOriginSegment(index) => url.domain_origin_segment_str(*index)?.into(),
            Self::DomainNormalSegment(index) => url.domain_normal_segment_str(*index)?.into(),

            Self::DomainSegmentRange       {start, end} => url.domain_range_str       ((*start, *end))?.into(),
            Self::DomainPrefixSegmentRange {start, end} => url.domain_prefix_range_str((*start, *end))?.into(),
            Self::DomainSuffixSegmentRange {start, end} => url.domain_suffix_range_str((*start, *end))?.into(),
            Self::DomainOriginSegmentRange {start, end} => url.domain_origin_range_str((*start, *end))?.into(),
            Self::DomainNormalSegmentRange {start, end} => url.domain_normal_range_str((*start, *end))?.into(),

            Self::DecodedDomainPrefix => url.domain_prefix()?.decode(),
            Self::DecodedDomainMiddle => url.domain_middle()?.decode(),
            Self::DecodedDomainSuffix => url.domain_suffix()?.decode(),
            Self::DecodedDomainOrigin => url.domain_origin()?.decode(),
            Self::DecodedDomainLabels => url.domain_labels()?.decode(),
            Self::DecodedDomainNormal => url.domain_normal()?.decode(),

            Self::DecodedDomainSegment      (index) => url.domain_segment       (*index)?.decode(),
            Self::DecodedDomainPrefixSegment(index) => url.domain_prefix_segment(*index)?.decode(),
            Self::DecodedDomainSuffixSegment(index) => url.domain_suffix_segment(*index)?.decode(),
            Self::DecodedDomainOriginSegment(index) => url.domain_origin_segment(*index)?.decode(),
            Self::DecodedDomainNormalSegment(index) => url.domain_normal_segment(*index)?.decode(),

            Self::DecodedDomainSegmentRange       {start, end} => url.domain_range       ((*start, *end))?.decode(),
            Self::DecodedDomainPrefixSegmentRange {start, end} => url.domain_prefix_range((*start, *end))?.decode(),
            Self::DecodedDomainSuffixSegmentRange {start, end} => url.domain_suffix_range((*start, *end))?.decode(),
            Self::DecodedDomainOriginSegmentRange {start, end} => url.domain_origin_range((*start, *end))?.decode(),
            Self::DecodedDomainNormalSegmentRange {start, end} => url.domain_normal_range((*start, *end))?.decode(),

            Self::Port => url.port_str()?.into(),

            Self::Path                             => url.path_str              (              ) .into  (),
            Self::PathSegment         (index     ) => url.path_segment          (*index        )?.decode(),
            Self::RawPathSegment      (index     ) => url.path_segment_str      (*index        )?.into  (),
            Self::RawPathSegmentRange {start, end} => url.path_segment_range_str((*start, *end))?.into  (),

            Self::Query                => url.query_str()?.into(),
            Self::QueryParam   (param) => url.query_param(&param.name, param.index)?.into_value    ()?,
            Self::RawQueryParam(param) => url.query_param(&param.name, param.index)?.into_raw_value()?,

            Self::Fragment                => url.fragment_str()?.into(),
            Self::FragmentParam   (param) => url.fragment_query_param(&param.name, param.index)?.into_value    ()?,
            Self::RawFragmentParam(param) => url.fragment_query_param(&param.name, param.index)?.into_raw_value()?,
        })
    }
}
