//! [`UrlPart`].

use std::ops::Bound;

use crate::prelude::*;

/// A common API for getting various parts of [`BetterUrl`]s.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub enum UrlPart {
    /** [`BetterUrl::as_str`].       **/ Whole,

    /** [`BetterUrl::scheme_str`].   **/ Scheme,

    /** [`BetterUrl::userinfo_str`]. **/ Userinfo,
    /** [`BetterUrl::username_str`]. **/ Username,
    /** [`BetterUrl::password_str`]. **/ Password,



    /** [`BetterUrl::host_str`].     **/ Host,

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
        ///
        /// Defaults to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")] start: Bound<isize>,
        /// The end of the range.
        ///
        /// Defaults to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")] end  : Bound<isize>,
    },
    /// [`BetterUrl::domain_prefix_range_str`].
    DomainPrefixSegmentRange {
        /// The start of the range.
        ///
        /// Defaults to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")] start: Bound<isize>,
        /// The end of the range.
        ///
        /// Defaults to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")] end  : Bound<isize>,
    },
    /// [`BetterUrl::domain_suffix_range_str`].
    DomainSuffixSegmentRange {
        /// The start of the range.
        ///
        /// Defaults to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")] start: Bound<isize>,
        /// The end of the range.
        ///
        /// Defaults to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")] end  : Bound<isize>,
    },
    /// [`BetterUrl::domain_origin_range_str`].
    DomainOriginSegmentRange {
        /// The start of the range.
        ///
        /// Defaults to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")] start: Bound<isize>,
        /// The end of the range.
        ///
        /// Defaults to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")] end  : Bound<isize>,
    },
    /// [`BetterUrl::domain_normal_range_str`].
    DomainNormalSegmentRange {
        /// The start of the range.
        ///
        /// Defaults to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")] start: Bound<isize>,
        /// The end of the range.
        ///
        /// Defaults to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")] end  : Bound<isize>,
    },



    /** [`BetterUrl::domain_prefix`] + [`DomainSegments::to_unicode`]. **/ DomainPrefixToUnicode,
    /** [`BetterUrl::domain_middle`] + [`DomainSegment::to_unicode`].  **/ DomainMiddleToUnicode,
    /** [`BetterUrl::domain_suffix`] + [`DomainSegments::to_unicode`]. **/ DomainSuffixToUnicode,
    /** [`BetterUrl::domain_origin`] + [`DomainSegments::to_unicode`]. **/ DomainOriginToUnicode,
    /** [`BetterUrl::domain_labels`] + [`DomainSegments::to_unicode`]. **/ DomainLabelsToUnicode,
    /** [`BetterUrl::domain_normal`] + [`DomainSegments::to_unicode`]. **/ DomainNormalToUnicode,

    /** [`BetterUrl::domain_segment`]        + [`DomainSegment::to_unicode`]. **/ DomainSegmentToUnicode      (isize),
    /** [`BetterUrl::domain_prefix_segment`] + [`DomainSegment::to_unicode`]. **/ DomainPrefixSegmentToUnicode(isize),
    /** [`BetterUrl::domain_suffix_segment`] + [`DomainSegment::to_unicode`]. **/ DomainSuffixSegmentToUnicode(isize),
    /** [`BetterUrl::domain_origin_segment`] + [`DomainSegment::to_unicode`]. **/ DomainOriginSegmentToUnicode(isize),
    /** [`BetterUrl::domain_normal_segment`] + [`DomainSegment::to_unicode`]. **/ DomainNormalSegmentToUnicode(isize),

    /// [`BetterUrl::domain_range`] + [`DomainSegments::to_unicode`].
    DomainSegmentRangeToUnicode {
        /// The start of the range.
        ///
        /// Defaults to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")] start: Bound<isize>,
        /// The end of the range.
        ///
        /// Defaults to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")] end  : Bound<isize>,
    },
    /// [`BetterUrl::domain_prefix_range`] + [`DomainSegments::to_unicode`].
    DomainPrefixSegmentRangeToUnicode {
        /// The start of the range.
        ///
        /// Defaults to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")] start: Bound<isize>,
        /// The end of the range.
        ///
        /// Defaults to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")] end  : Bound<isize>,
    },
    /// [`BetterUrl::domain_suffix_range`] + [`DomainSegments::to_unicode`].
    DomainSuffixSegmentRangeToUnicode {
        /// The start of the range.
        ///
        /// Defaults to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")] start: Bound<isize>,
        /// The end of the range.
        ///
        /// Defaults to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")] end  : Bound<isize>,
    },
    /// [`BetterUrl::domain_origin_range`] + [`DomainSegments::to_unicode`].
    DomainOriginSegmentRangeToUnicode {
        /// The start of the range.
        ///
        /// Defaults to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")] start: Bound<isize>,
        /// The end of the range.
        ///
        /// Defaults to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")] end  : Bound<isize>,
    },
    /// [`BetterUrl::domain_normal_range`] + [`DomainSegments::to_unicode`].
    DomainNormalSegmentRangeToUnicode {
        /// The start of the range.
        ///
        /// Defaults to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")] start: Bound<isize>,
        /// The end of the range.
        ///
        /// Defaults to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")] end  : Bound<isize>,
    },



    /** [`BetterUrl::port_str`]. **/ Port,



    /** [`BetterUrl::path_str`].                                     **/ Path,
    /** [`BetterUrl::path_segment`] + [`PathSegment::lossy_decode`]. **/ PathSegment(isize),
    /** [`BetterUrl::path_segment_str`].                             **/ RawPathSegment(isize),

    /// [`BetterUrl::path_segment_range_str`].
    RawPathSegmentRange {
        /// The start of the range.
        ///
        /// Defaults to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")] start: Bound<isize>,
        /// The end of the range.
        ///
        /// Defaults to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")] end  : Bound<isize>,
    },

    /** [`BetterUrl::query_str`].                                                                      **/ Query,
    /** [`BetterUrl::query_param`] + [`QuerySegment::into_value`] + [`MaybeQueryValue::lossy_decode`]. **/ QueryParam(QueryParamSelector),
    /** [`BetterUrl::query_param`] + [`QuerySegment::into_value`] + [`MaybeQueryValue::into_inner`].   **/ RawQueryParam(QueryParamSelector),

    /** [`BetterUrl::fragment_str`].                                                                                            **/ Fragment,
    /** [`BetterUrl::fragment_query_param`] + [`FragmentQuerySegment::into_value`] + [`MaybeFragmentQueryValue::lossy_decode`]. **/ FragmentParam(QueryParamSelector),
    /** [`BetterUrl::fragment_query_param`] + [`FragmentQuerySegment::into_value`] + [`MaybeFragmentQueryValue::into_inner`].   **/ RawFragmentParam(QueryParamSelector),
}

impl UrlPart {
    /// [`Self::get`], replacing [`None`] with the error [`UrlPartNotFound`].
    /// # Errors
    /// If [`UrlPart::get`] returns [`None`], returns the error [`UrlPartNotFound`].
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

            Self::DomainPrefixToUnicode => url.domain_prefix()?.to_unicode(),
            Self::DomainMiddleToUnicode => url.domain_middle()?.to_unicode(),
            Self::DomainSuffixToUnicode => url.domain_suffix()?.to_unicode(),
            Self::DomainOriginToUnicode => url.domain_origin()?.to_unicode(),
            Self::DomainLabelsToUnicode => url.domain_labels()?.to_unicode(),
            Self::DomainNormalToUnicode => url.domain_normal()?.to_unicode(),

            Self::DomainSegmentToUnicode      (index) => url.domain_segment       (*index)?.to_unicode(),
            Self::DomainPrefixSegmentToUnicode(index) => url.domain_prefix_segment(*index)?.to_unicode(),
            Self::DomainSuffixSegmentToUnicode(index) => url.domain_suffix_segment(*index)?.to_unicode(),
            Self::DomainOriginSegmentToUnicode(index) => url.domain_origin_segment(*index)?.to_unicode(),
            Self::DomainNormalSegmentToUnicode(index) => url.domain_normal_segment(*index)?.to_unicode(),

            Self::DomainSegmentRangeToUnicode       {start, end} => url.domain_range       ((*start, *end))?.to_unicode(),
            Self::DomainPrefixSegmentRangeToUnicode {start, end} => url.domain_prefix_range((*start, *end))?.to_unicode(),
            Self::DomainSuffixSegmentRangeToUnicode {start, end} => url.domain_suffix_range((*start, *end))?.to_unicode(),
            Self::DomainOriginSegmentRangeToUnicode {start, end} => url.domain_origin_range((*start, *end))?.to_unicode(),
            Self::DomainNormalSegmentRangeToUnicode {start, end} => url.domain_normal_range((*start, *end))?.to_unicode(),

            Self::Port => url.port_str()?.into(),

            Self::Path                             => url.path_str              (              ) .into        (),
            Self::PathSegment         (index     ) => url.path_segment          (*index        )?.lossy_decode(),
            Self::RawPathSegment      (index     ) => url.path_segment_str      (*index        )?.into        (),
            Self::RawPathSegmentRange {start, end} => url.path_segment_range_str((*start, *end))?.into        (),

            Self::Query                => url.query_str()?.into(),
            Self::QueryParam   (param) => url.query_param(&param.name, param.index)?.into_value().lossy_decode()?,
            Self::RawQueryParam(param) => url.query_param(&param.name, param.index)?.into_value().into_inner  ()?,

            Self::Fragment                => url.fragment_str()?.into(),
            Self::FragmentParam   (param) => url.fragment_query_param(&param.name, param.index)?.into_value().lossy_decode()?,
            Self::RawFragmentParam(param) => url.fragment_query_param(&param.name, param.index)?.into_value().into_inner  ()?,
        })
    }
}
