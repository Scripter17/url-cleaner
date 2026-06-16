//! [`Condition`].

use crate::prelude::*;

/// When to do an [`Action`].
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub enum Condition {
    // Debug/constants

    /// [`true`].
    Always,
    /// [`false`].
    ///
    /// The default.
    #[default]
    Never,
    /// [`ExplicitError`].
    Error(String),

    // Logic

    /// If [`Self::If::if`] then [`Self::If::then`], otherwise [`Self::If::else`].
    If {
        /// The if.
        r#if: Box<Self>,
        /// The then.
        then: Box<Self>,
        /// The else.
        ///
        /// Defaults to [`Self::Never`].
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Box<Self>
    },
    /// Invert.
    Not(Box<Self>),
    /// All.
    All(Vec<Self>),
    /// Any.
    Any(Vec<Self>),

    // Error handling

    /// Map [`Err`] to [`true`].
    ErrorToSatisfied(Box<Self>),
    /// Map [`Err`] to [`false`].
    ErrorToUnsatisfied(Box<Self>),
    /// [`Self::TryElse::try`], or [`Self::TryElse::else`] if it's [`Err`].
    TryElse {
        /// The try.
        r#try: Box<Self>,
        /// The else.
        r#else: Box<Self>
    },
    /// The first contained [`Self`] to return [`Ok`].
    /// # Errors
    /// If no contained [`Self`] returns [`Ok`], returns the error [`FirstNotErrorErrors`] containing every error.
    FirstNotError(Vec<Self>),

    // Maps

    /// [`UrlPart::get`] + [`Map::get`].
    PartMap {
        /// The [`UrlPart`] to index [`Self::PartMap::map`] with.
        part: UrlPart,
        /// The [`Map`] to index with [`Self::PartMap::part`].
        #[serde(flatten)]
        map: Box<Map<Self>>
    },
    /// [`StringSource::get`] + [`Map::get`].
    StringMap {
        /// The [`StringSource`] to index [`Self::StringMap::map`] with.
        value: StringSource,
        /// The [`Map`] to index with [`Self::StringMap::value`].
        #[serde(flatten)]
        map: Box<Map<Self>>
    },
    /// [`PartitioningSource::get`] + [`Partitioning::get`] + [`UrlPart::get`] + [`Map::index`].
    PartPartitioning {
        /// The [`Partitioning`].
        partitioning: PartitioningSource,
        /// The value.
        part: UrlPart,
        /// The [`Map`].
        #[serde(flatten)]
        map: Box<Map<Self>>
    },
    /// [`PartitioningSource::get`] + [`Partitioning::get`] + [`StringSource::get`] + [`Map::index`].
    StringPartitioning {
        /// The [`Partitioning`].
        partitioning: PartitioningSource,
        /// The value.
        value: StringSource,
        /// The [`Map`].
        #[serde(flatten)]
        map: Box<Map<Self>>
    },

    // Params

    /// [`FlagSource::get`].
    FlagIsSet(FlagSource),
    /// [`FlagSource::get`] inverted.
    FlagIsNotSet(FlagSource),

    // String source

    /// If [`Self::StringIs::left`] is [`Self::StringIs::right`].
    StringIs {
        /// The left side.
        left: StringSource,
        /// The right side.
        right: StringSource
    },
    /// If [`Self::StringIsInSet::value`] is in [`Self::StringIsInSet::set`].
    StringIsInSet {
        /// The [`StringSource`].
        value: StringSource,
        /// The [`SetSource`].
        set: SetSource
    },
    /// If [`Self::StringStartsWith::value`] starts with [`Self::StringStartsWith::prefix`].
    StringStartsWith {
        /// The [`StringSource`] to search in.
        value: StringSource,
        /// The [`StringSource`] to search for.
        prefix: StringSource
    },
    /// If [`Self::StringEndsWith::value`] starts with [`Self::StringEndsWith::suffix`].
    StringEndsWith {
        /// The [`StringSource`] to search in.
        value: StringSource,
        /// The [`StringSource`] to search for.
        suffix: StringSource
    },
    /// [`StringLocation::contains`].
    StringContains {
        /// The value to search in.
        value: StringSource,
        /// The value to search for.
        substring: StringSource,
        /// The [`StringLocation`].
        ///
        /// Defaults to [`StringLocation::Anywhere`].
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation
    },
    /// [`StringMatcher::check`].
    StringMatches {
        /// The value to check.
        value: StringSource,
        /// The [`StringMatcher`].
        matcher: StringMatcher
    },

    // Whole

    /// [`BetterUrl::is_special`].
    UrlIsSpecial,
    /// [`BetterUrl::is_special_not_file`].
    UrlIsSpecialNotFile,
    /// [`BetterUrl::is_file`].
    UrlIsFile,
    /// [`BetterUrl::is_non_special`].
    UrlIsNonSpecial,

    // Scheme

    /// If the [`BetterUrl::scheme_str`] is the specified value.
    SchemeIs(StringSource),
    /// If the [`BetterUrl::scheme_str`] is in the [`Set`].
    SchemeIsInSet(SetSource),
    /// [`SchemeDetails::is_http`].
    SchemeIsHttp,
    /// [`SchemeDetails::is_https`].
    SchemeIsHttps,
    /// [`SchemeDetails::is_http_or_https`].
    SchemeIsHttpOrHttps,

    // Host is

    /// If the [`BetterUrl::host_str`] is equal to the specified string.
    HostIs(StringSource),
    /// If the [`BetterUrl::domain_normal`] is equal to the specified string.
    DomainNormalIs(StringSource),
    /// If the [`BetterUrl::domain_origin`] is equal to the specified string.
    DomainOriginIs(StringSource),
    /// If the [`BetterUrl::domain_prefix`] is equal to the specified string.
    DomainPrefixIs(StringSource),
    /// If the [`BetterUrl::domain_middle`] is equal to the specified string.
    DomainMiddleIs(StringSource),
    /// If the [`BetterUrl::domain_suffix`] is equal to the specified string.
    DomainSuffixIs(StringSource),



    /// If the [`BetterUrl::domain_segment`] is the specified value.
    DomainSegmentIs {
        /// The segment to check.
        index: isize,
        /// The value to check for.
        value: StringSource
    },
    /// If the [`BetterUrl::domain_prefix_segment`] is the specified value.
    DomainPrefixSegmentIs {
        /// The segment to check.
        index: isize,
        /// The value to check for.
        value: StringSource
    },
    /// If the [`BetterUrl::domain_suffix_segment`] is the specified value.
    DomainSuffixSegmentIs {
        /// The segment to check.
        index: isize,
        /// The value to check for.
        value: StringSource
    },
    /// If the [`BetterUrl::domain_origin_segment`] is the specified value.
    DomainOriginSegmentIs {
        /// The segment to check.
        index: isize,
        /// The value to check for.
        value: StringSource
    },

    // Host is in set

    /// If the [`BetterUrl::host_str`] is in the specified [`Set`].
    HostIsInSet(SetSource),
    /// If the [`BetterUrl::domain_normal`] is in the specified [`Set`].
    DomainNormalIsInSet(SetSource),
    /// If the [`BetterUrl::domain_prefix`] is in the specified [`Set`].
    DomainPrefixIsInSet(SetSource),
    /// If the [`BetterUrl::domain_origin`] is in the specified [`Set`].
    DomainOriginIsInSet(SetSource),
    /// If the [`BetterUrl::domain_middle`] is in the specified [`Set`].
    DomainMiddleIsInSet(SetSource),
    /// If the [`BetterUrl::domain_suffix`] is in the specified [`Set`].
    DomainSuffixIsInSet(SetSource),



    /// If the [`BetterUrl::domain_segment`] is in the specified [`Set`].
    DomainSegmentIsInSet {
        /// The segment to check.
        index: isize,
        /// The [`Set`] to check in.
        set: SetSource
    },
    /// If the [`BetterUrl::domain_origin_segment`] is in the specified [`Set`].
    DomainOriginSegmentIsInSet {
        /// The segment to check.
        index: isize,
        /// The [`Set`] to check in.
        set: SetSource
    },
    /// If the [`BetterUrl::domain_prefix_segment`] is in the specified [`Set`].
    DomainPrefixSegmentIsInSet {
        /// The segment to check.
        index: isize,
        /// The [`Set`] to check in.
        set: SetSource
    },
    /// If the [`BetterUrl::domain_suffix_segment`] is in the specified [`Set`].
    DomainSuffixSegmentIsInSet {
        /// The segment to check.
        index: isize,
        /// The [`Set`] to check in.
        set: SetSource
    },

    // Misc. host

    /// [`BetterUrl::has_host`].
    UrlHasHost,
    /// [`BetterUrl::host_is_domain`].
    HostIsDomain,
    /// [`BetterUrl::host_is_ip`].
    HostIsIp,
    /// [`BetterUrl::host_is_ipv4`].
    HostIsIpv4,
    /// [`BetterUrl::host_is_ipv6`].
    HostIsIpv6,
    /// [`BetterUrl::host_is_opaque`].
    HostIsOpaque,
    /// [`BetterUrl::host_is_empty`].
    HostIsEmpty,

    /// [`BetterUrl::has_domain_prefix`].
    UrlHasDomainPrefix,
    /// [`BetterUrl::has_domain_middle`].
    UrlHasDomainMiddle,
    /// [`BetterUrl::has_domain_suffix`].
    UrlHasDomainSuffix,
    /// [`BetterUrl::has_domain_origin`].
    UrlHasDomainOrigin,
    /// [`BetterUrl::has_domain_labels`].
    UrlHasDomainLabels,
    /// [`BetterUrl::has_domain_normal`].
    UrlHasDomainNormal,

    /// [`IpDetails::is_loopback`].
    HostIsLoopbackIp,
    /// [`IpDetails::is_multicast`].
    HostIsMulticastIp,
    /// [`IpDetails::is_unspecified`].
    HostIsUnspecifiedIp,

    /// [`Ipv4Details::is_broadcast`].
    HostIsBroadcastIpv4,
    /// [`Ipv4Details::is_documentation`].
    HostIsDocumentationIpv4,
    /// [`Ipv4Details::is_link_local`].
    HostIsLinkLocalIpv4,
    /// [`Ipv4Details::is_loopback`].
    HostIsLoopbackIpv4,
    /// [`Ipv4Details::is_multicast`].
    HostIsMulticastIpv4,
    /// [`Ipv4Details::is_private`].
    HostIsPrivateIpv4,
    /// [`Ipv4Details::is_unspecified`].
    HostIsUnspecifiedIpv4,

    /// [`Ipv6Details::is_loopback`].
    HostIsLoopbackIpv6,
    /// [`Ipv6Details::is_unicast_link_local`].
    HostIsUnicastLinkLocalIpv6,
    /// [`Ipv6Details::is_multicast`].
    HostIsMulticastIpv6,
    /// [`Ipv6Details::is_unique_local`].
    HostIsUniqueLocalIpv6,
    /// [`Ipv6Details::is_unspecified`].
    HostIsUnspecifiedIpv6,

    // Path

    /// [`BetterUrl::path_is_segmented`].
    PathIsSegmented,
    /// [`BetterUrl::path_is_opaque`].
    PathIsOpaque,
    /// [`BetterUrl::has_path_segment`].
    PathHasSegment(isize),

    /// If [`BetterUrl::path_str`] is the specified value.
    PathIs(StringSource),
    /// If [`BetterUrl::path_str`] is in the [`Set`].
    PathIsInSet(SetSource),
    /// If [`BetterUrl::path_str`] starts with the specified value.
    PathStartsWith(StringSource),
    /// If [`BetterUrl::path_str`] ends with the specified value.
    PathEndsWith(StringSource),
    /// If [`BetterUrl::path_str`] contains the specified value.
    PathContains {
        /// The value to check for.
        substring: StringSource,
        /// The [`StringLocation`].
        ///
        /// Defaults to [`StringLocation::Anywhere`].
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation
    },
    /// If [`BetterUrl::path_str`] satisfies the specified [`StringMatcher`].
    PathMatches(StringMatcher),

    // Path segment

    /// If the [`BetterUrl::path_segment`] + [`PathSegment::decode`] is the specified value.
    PathSegmentIs {
        /// The segment to check.
        index: isize,
        /// The value to check for.
        value: StringSource
    },
    /// If the [`BetterUrl::path_segment`] + [`PathSegment::decode`] is in the [`Set`].
    PathSegmentIsInSet {
        /// The segment to check.
        index: isize,
        /// The [`Set`] to check in.
        set: SetSource
    },
    /// If the [`BetterUrl::path_segment`] + [`PathSegment::decode`] starts with the specified value.
    /// # Errors
    /// If [`BetterUrl::path_segment`] reutrns [`None`], returns the error [`PathSegmentNotFound`].
    PathSegmentStartsWith {
        /// The segment to check.
        index: isize,
        /// The value to check for.
        prefix: StringSource
    },
    /// If the [`BetterUrl::path_segment`] + [`PathSegment::decode`] ends with the specified value.
    /// # Errors
    /// If [`BetterUrl::path_segment`] reutrns [`None`], returns the error [`PathSegmentNotFound`].
    PathSegmentEndsWith {
        /// The segment to check.
        index: isize,
        /// The value to check for.
        suffix: StringSource
    },
    /// If the [`BetterUrl::path_segment`] + [`PathSegment::decode`] contains the specified value.
    /// # Errors
    /// If [`BetterUrl::path_segment`] reutrns [`None`], returns the error [`PathSegmentNotFound`].
    PathSegmentContains {
        /// The path segment to get.
        index: isize,
        /// The value to check for.
        substring: StringSource,
        /// The location to cehck at.
        ///
        /// Defaults to [`StringLocation::Anywhere`].
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation
    },
    /// If the [`BetterUrl::path_segment`] + [`PathSegment::decode`] satisfies the [`StringMatcher`].
    PathSegmentMatches {
        /// The segment to check.
        index: isize,
        /// The [`StringMatcher`].
        matcher: StringMatcher
    },

    /// If the [`BetterUrl::path_segment`] + [`PathSegment::into_inner`] is the specified value.
    RawPathSegmentIs {
        /// The segment to check.
        index: isize,
        /// The value to check for.
        value: StringSource
    },
    /// If the [`BetterUrl::path_segment`] + [`PathSegment::into_inner`] is in the [`Set`].
    RawPathSegmentIsInSet {
        /// The segment to check.
        index: isize,
        /// The [`Set`] to check in.
        set: SetSource
    },
    /// If the [`BetterUrl::path_segment`] + [`PathSegment::into_inner`] starts with the specified value.
    /// # Errors
    /// If [`BetterUrl::path_segment`] reutrns [`None`], returns the error [`PathSegmentNotFound`].
    RawPathSegmentStartsWith {
        /// The segment to check.
        index: isize,
        /// The value to check for.
        prefix: StringSource
    },
    /// If the [`BetterUrl::path_segment`] + [`PathSegment::into_inner`] ends with the specified value.
    /// # Errors
    /// If [`BetterUrl::path_segment`] reutrns [`None`], returns the error [`PathSegmentNotFound`].
    RawPathSegmentEndsWith {
        /// The segment to check.
        index: isize,
        /// The value to check for.
        suffix: StringSource
    },
    /// If the [`BetterUrl::path_segment`] + [`PathSegment::into_inner`] contains the specified value.
    /// # Errors
    /// If [`BetterUrl::path_segment`] reutrns [`None`], returns the error [`PathSegmentNotFound`].
    RawPathSegmentContains {
        /// The path segment to get.
        index: isize,
        /// The value to check for.
        substring: StringSource,
        /// The location to cehck at.
        ///
        /// Defaults to [`StringLocation::Anywhere`].
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation
    },
    /// If the [`BetterUrl::path_segment`] + [`PathSegment::into_inner`] satisfies the [`StringMatcher`].
    RawPathSegmentMatches {
        /// The segment to check.
        index: isize,
        /// The [`StringMatcher`].
        matcher: StringMatcher
    },

    // Query

    /// [`BetterUrl::query_str`] + [`Eq::eq`].
    QueryIs(StringSource),
    /// [`BetterUrl::query_str`] + [`Set::Contains`].
    QueryIsInSet(SetSource),
    /// If [`BetterUrl::query_str`] starts with the specified value.
    QueryStartsWith(StringSource),
    /// If [`BetterUrl::query_str`] ends with the specified value.
    QueryEndsWith(StringSource),
    /// If [`BetterUrl::query_str`] contains the specified value.
    QueryContains {
        /// The value to check for.
        substring: StringSource,
        /// The [`StringLocation`].
        ///
        /// Defaults to [`StringLocation::Anywhere`].
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation
    },
    /// [`BetterUrl::query_str`] + [`StringMatcher::check`].
    QueryMatches(StringMatcher),

    /// [`BetterUrl::has_query_param`].
    QueryHasParam(QueryParamSelector),

    // Query params

    /// [`BetterUrl::query_param`] + [`QuerySegment::into_value`] ([`Option::flatten`]ed) + [`Eq::eq`].
    QueryParamIs {
        /// The [`QueryParamSelector`].
        param: QueryParamSelector,
        /// The value to check for.
        value: StringSource
    },
    /// [`BetterUrl::query_param`] + [`QuerySegment::into_value`] ([`Option::flatten`]ed) + [`Set::contains`].
    QueryParamIsInSet {
        /// The [`QueryParamSelector`].
        param: QueryParamSelector,
        /// The [`Set`].
        set: SetSource
    },
    /// If the [`BetterUrl::path_segment`] + [`QuerySegment::into_value`] starts with the specified value.
    /// # Errors
    /// If [`BetterUrl::path_segment`] reutrns [`None`], returns the error [`QueryParamNotFound`].
    QueryParamStartsWith {
        /// The [`QueryParamSelector`].
        param: QueryParamSelector,
        /// The value to check for.
        prefix: StringSource
    },
    /// If the [`BetterUrl::path_segment`] + [`QuerySegment::into_value`] ends with the specified value.
    /// # Errors
    /// If [`BetterUrl::path_segment`] reutrns [`None`], returns the error [`QueryParamNotFound`].
    QueryParamEndsWith {
        /// The [`QueryParamSelector`].
        param: QueryParamSelector,
        /// The value to check for.
        suffix: StringSource
    },
    /// If the [`BetterUrl::path_segment`] + [`QuerySegment::into_value`] contains the specified value.
    /// # Errors
    /// If [`BetterUrl::path_segment`] reutrns [`None`], returns the error [`QueryParamNotFound`].
    QueryParamContains {
        /// The [`QueryParamSelectorget.
        param: QueryParamSelector,
        /// The value to check for.
        substring: StringSource,
        /// The location to cehck at.
        ///
        /// Defaults to [`StringLocation::Anywhere`].
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation
    },
    /// [`BetterUrl::query_param`] + [`QuerySegmen::into_value`] ([`Option::flatten`]) +ed [`StringMatcher::check`]
    QueryParamMatches {
        /// The [`QueryParamSelector`].
        param: QueryParamSelector,
        /// The [`StringMatcher`]
        matcher: StringMatcher
    },



    /// [`BetterUrl::query_param`] + [`QuerySegment::into_raw_value`] ([`Option::flatten`]ed) + [`Eq::eq`].
    RawQueryParamIs {
        /// The [`QueryParamSelector`].
        param: QueryParamSelector,
        /// The value to check for.
        value: StringSource
    },
    /// [`BetterUrl::query_param`] + [`QuerySegment::into_raw_value`] ([`Option::flatten`]ed) + [`Set::contains`].
    RawQueryParamIsInSet {
        /// The [`QueryParamSelector`].
        param: QueryParamSelector,
        /// The [`Set`].
        set: SetSource
    },
    /// If the [`BetterUrl::path_segment`] + [`QuerySegment::into_raw_value`] starts with the specified value.
    /// # Errors
    /// If [`BetterUrl::path_segment`] reutrns [`None`], returns the error [`QueryParamNotFound`].
    RawQueryParamStartsWith {
        /// The [`QueryParamSelector`].
        param: QueryParamSelector,
        /// The value to check for.
        prefix: StringSource
    },
    /// If the [`BetterUrl::path_segment`] + [`QuerySegment::into_raw_value`] ends with the specified value.
    /// # Errors
    /// If [`BetterUrl::path_segment`] reutrns [`None`], returns the error [`QueryParamNotFound`].
    RawQueryParamEndsWith {
        /// The [`QueryParamSelector`].
        param: QueryParamSelector,
        /// The value to check for.
        suffix: StringSource
    },
    /// If the [`BetterUrl::path_segment`] + [`QuerySegment::into_raw_value`] contains the specified value.
    /// # Errors
    /// If [`BetterUrl::path_segment`] reutrns [`None`], returns the error [`QueryParamNotFound`].
    RawQueryParamContains {
        /// The [`QueryParamSelectorget.
        param: QueryParamSelector,
        /// The value to check for.
        substring: StringSource,
        /// The location to cehck at.
        ///
        /// Defaults to [`StringLocation::Anywhere`].
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation
    },
    /// [`BetterUrl::query_param`] + [`QuerySegmen::into_raw_value`] ([`Option::flatten`]ed) + [`StringMatcher::check`]
    RawQueryParamMatches {
        /// The [`QueryParamSelector`].
        param: QueryParamSelector,
        /// The [`StringMatcher`]
        matcher: StringMatcher
    },

    // Fragment

    /// [`BetterUrl::fragment_str`] + [`Eq::eq`].
    FragmentIs(StringSource),
    /// [`BetterUrl::fragment_str`] + [`Set::contains`].
    FragmentIsInSet(SetSource),
    /// [`BetterUrl::fragment_str`] + [`StringMatcher::check`].
    FragmentMatches(StringMatcher),

    // General parts

    /// If [`Self::PartIs::part`] is [`Self::PartIs::value`].
    PartIs {
        /// The [`UrlPart`].
        part: UrlPart,
        /// The [`StringSource`].
        value: StringSource
    },
    /// If [`Self::PartIsInSet::part`] is in [`Self::PartIsInSet::set`].
    PartIsInSet {
        /// The [`UrlPart`].
        part: UrlPart,
        /// The [`SetSource`].
        set: SetSource
    },
    /// If [`Self::PartStartsWith::part`] starts with [`Self::PartStartsWith::prefix`].
    PartStartsWith {
        /// The [`UrlPart`].
        part: UrlPart,
        /// The [`StringSource`].
        prefix: StringSource
    },
    /// If [`Self::PartEndsWith::part`] starts with [`Self::PartEndsWith::suffix`].
    PartEndsWith {
        /// The [`UrlPart`].
        part: UrlPart,
        /// The [`StringSource`].
        suffix: StringSource
    },
    /// If [`Self::PartContains::part`] contains [`Self::PartContains::substring`] at [`Self::PartContains::at`].
    PartContains {
        /// The [`UrlPart`].
        part: UrlPart,
        /// The [`StringSource`].
        substring: StringSource,
        /// The [`StringLocation`].
        ///
        /// Defaults to [`StringLocation::Anywhere`].
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation
    },
    /// If [`Self::PartMatches::part`] matches [`Self::PartMatches::matcher`].
    PartMatches {
        /// The [`UrlPart`].
        part: UrlPart,
        /// The [`StringMatcher`].
        matcher: StringMatcher
    },

    /// If [`Self::PartIsSomeAndStartsWith::part`] is [`Some`] and starts with [`Self::PartIsSomeAndStartsWith::prefix`].
    PartIsSomeAndStartsWith {
        /// The [`UrlPart`].
        part: UrlPart,
        /// the [`StringSource`].
        prefix: StringSource
    },
    /// If [`Self::PartIsSomeAndEndsWith::part`] is [`Some`] and starts with [`Self::PartIsSomeAndEndsWith::suffix`].
    PartIsSomeAndEndsWith {
        /// The [`UrlPart`].
        part: UrlPart,
        /// the [`StringSource`].
        suffix: StringSource
    },
    /// If [`Self::PartIsSomeAndContains::part`] is [`Some`] and contains [`Self::PartIsSomeAndContains::substring`] at [`Self::PartIsSomeAndContains::at`].
    PartIsSomeAndContains {
        /// The [`UrlPart`].
        part: UrlPart,
        /// the [`StringSource`].
        substring: StringSource,
        /// The [`StringLocation`].
        ///
        /// Defaults to [`StringLocation::Anywhere`].
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation
    },

    // Misc.

    /// Uses a [`Self`] from [`Cleaner::functions`].
    Function(Box<FunctionCall>),
    /// Uses a [`Self`] from [`TaskState::call_args`].
    FunctionArg(StringSource),
    /// Calls the specified function and returns its value.
    ///
    /// Because this uses function pointers, this plays weirdly with [`PartialEq`]/[`Eq`].
    ///
    /// Additionally, using a function pointer means this variant cannot be [`Serialize`]d or [`Deserialize`]d.
    #[suitable(never)]
    #[serde(skip)]
    Extern(ConditionExtern)
}

/// The enum of errors [`Condition::check`] can return.
#[derive(Debug, Error)]
pub enum ConditionError {
    /// [`ExplicitError`].
    #[error(transparent)]
    ExplicitError(#[from] ExplicitError),
    /// [`TryElseError`].
    #[error(transparent)]
    TryElseError(#[from] Box<TryElseError<Self>>),
    /// [`FirstNotErrorErrors`].
    #[error(transparent)]
    FirstNotErrorErrors(#[from] FirstNotErrorErrors<Self>),

    /// [`StringNotFound`].
    #[error(transparent)]
    StringNotFound(#[from] StringNotFound),
    /// [`StringSourceError`].
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
    /// [`StringMatcherError`].
    #[error(transparent)]
    StringMatcherError(#[from] StringMatcherError),
    /// [`StringLocationError`].
    #[error(transparent)]
    StringLocationError(#[from] StringLocationError),

    /// [`PartitioningSourceError`].
    #[error(transparent)]
    PartitioningSourceError(#[from] PartitioningSourceError),
    /// [`PartitioningNotFound`].
    #[error(transparent)]
    PartitioningNotFound(#[from] PartitioningNotFound),

    /// [`SetSourceError`].
    #[error(transparent)]
    SetSourceError(#[from] SetSourceError),
    /// [`SetNotFound`].
    #[error(transparent)]
    SetNotFound(#[from] SetNotFound),


    /// [`UrlPartNotFound`].
    #[error(transparent)]
    UrlPartNotFound(#[from] UrlPartNotFound),

    /// [`FlagSourceError`].
    #[error(transparent)]
    FlagSourceError(#[from] FlagSourceError),
    /// [`VarSourceError`].
    #[error(transparent)]
    VarSourceError(#[from] VarSourceError),

    /// Returned when a part of the URL is [`None`] where it has to be [`Some`].
    #[error("A part of the URL is None where it had to be Some.")]
    UrlPartIsNone,
    /// Returned when attempting to get a path segment not in a URL.
    #[error("Attempted to get a path segment not in the URL.")]
    PathSegmentNotFound,

    /// [`PathIsOpaque`].
    #[error(transparent)]
    PathIsOpaque(#[from] PathIsOpaque),
    /// Returned when the query is [`None`] when it has to be [`Some`].
    #[error("The query was None when it had to be Some.")]
    QueryIsNone,
    /// Returned when the query parameter isn't found.
    #[error("The query parameter wasn't found.")]
    QueryParamNotFound,
    /// Returned when the query parameter is found but has no value.
    #[error("The query parameter was found but had no value.")]
    QueryParamNoValue,

    /// [`FunctionNotFound`].
    #[error(transparent)]
    FunctionNotFound(#[from] FunctionNotFound),
    /// [`NotInFunction`].
    #[error(transparent)]
    NotInFunction(#[from] NotInFunction),
    /// [`FunctionArgFunctionNotFound`].
    #[error(transparent)]
    FunctionArgFunctionNotFound(#[from] FunctionArgFunctionNotFound),

    /// An arbitrary [`std::error::Error`] returned by [`Condition::Extern`].
    #[error(transparent)]
    Extern(Box<dyn std::error::Error + Send + Sync>)
}

impl Condition {
    /// If it's satisfied.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn check<'j>(&'j self, task_state: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<bool, ConditionError> {
        debug!(Condition::check, self, task_state.url, args; self._check(task_state, args))
    }

    /// [`Self::check`].
    fn _check<'j>(&'j self, task_state: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<bool, ConditionError> {
        Ok(match self {
            // Debug/constants

            Self::Always => true,
            Self::Never => false,
            Self::Error(msg) => Err(ExplicitError(msg.clone()))?,

            // Logic

            Self::If {r#if, then, r#else} => match r#if.check(task_state, args)? {
                true  => then  .check(task_state, args)?,
                false => r#else.check(task_state, args)?
            },
            Self::Not(condition) => !condition.check(task_state, args)?,
            Self::All(conditions) => {
                for condition in conditions {
                    if !condition.check(task_state, args)? {
                        return Ok(false);
                    }
                }
                true
            },
            Self::Any(conditions) => {
                for condition in conditions {
                    if condition.check(task_state, args)? {
                        return Ok(true);
                    }
                }
                false
            },

            // Error handling

            Self::ErrorToSatisfied  (condition) => condition.check(task_state, args).unwrap_or(true ),
            Self::ErrorToUnsatisfied(condition) => condition.check(task_state, args).unwrap_or(false),
            Self::TryElse {r#try, r#else} => match r#try.check(task_state, args) {
                Ok(x) => x,
                Err(try_error) => match r#else.check(task_state, args) {
                    Ok(x) => x,
                    Err(else_error) => Err(TryElseError {try_error, else_error})?
                }
            },
            Self::FirstNotError(matchers) => {
                let mut errors = Vec::new();

                for matcher in matchers {
                    match matcher.check(task_state, args) {
                        Ok (x) => return Ok(x),
                        Err(e) => errors.push(e)
                    }
                }

                Err(FirstNotErrorErrors(errors))?
            },

            // Maps

            Self::PartMap  {part , map} => if let Some(condition) = map.get(part.get(&task_state.url)) {condition.check(task_state, args)?} else {false},
            Self::StringMap{value, map} => if let Some(condition) = map.get(get!(?&value))             {condition.check(task_state, args)?} else {false},

            Self::PartPartitioning   {partitioning, part , map} => if let Some(condition) = map.get(get!(partitioning).get(part.get(&task_state.url).as_deref())) {condition.check(task_state, args)?} else {false},
            Self::StringPartitioning {partitioning, value, map} => if let Some(condition) = map.get(get!(partitioning).get(get!(?&value)))                        {condition.check(task_state, args)?} else {false},

            // Params

            Self::FlagIsSet   (flag) =>  flag.get(task_state, args)?,
            Self::FlagIsNotSet(flag) => !flag.get(task_state, args)?,

            // Strings

            Self::StringIs         {left , right        } => get!(?left) == get!(?right),
            Self::StringIsInSet    {value, set          } => get!(set  ).contains   (get!(?&value )),
            Self::StringStartsWith {value, prefix       } => get!(value).starts_with(get!(& prefix)),
            Self::StringEndsWith   {value, suffix       } => get!(value).ends_with  (get!(& suffix)),
            Self::StringContains   {value, substring, at} => at.check(get!(&value), get!(&substring))?,
            Self::StringMatches    {value, matcher      } => matcher.check(task_state, args, get!(?&value))?,

            // Parts

            Self::PartIs        {part, value        } => part.get(&task_state.url) == get!(?value),
            Self::PartIsInSet   {part, set          } => get!(set ).contains(part.get(&task_state.url).as_deref()),
            Self::PartStartsWith{part, prefix       } => part.get_some(&task_state.url)?.starts_with(get!(&prefix)),
            Self::PartEndsWith  {part, suffix       } => part.get_some(&task_state.url)?.ends_with  (get!(&suffix)),
            Self::PartContains  {part, substring, at} => at.check(&part.get_some(&task_state.url)?,  get!(&substring))?,
            Self::PartMatches   {part, matcher      } => matcher.check(task_state, args, part.get(&task_state.url).as_deref())?,

            Self::PartIsSomeAndStartsWith {part, prefix       } => if let Some(x) = part.get(&task_state.url) {x.starts_with(get!(&prefix   )) } else {false},
            Self::PartIsSomeAndEndsWith   {part, suffix       } => if let Some(x) = part.get(&task_state.url) {x.ends_with  (get!(&suffix   )) } else {false},
            Self::PartIsSomeAndContains   {part, substring, at} => if let Some(x) = part.get(&task_state.url) {at.check(&x,  get!(&substring))?} else {false},

            // Whole

            Self::UrlIsSpecial        => task_state.url.is_special    (),
            Self::UrlIsSpecialNotFile => task_state.url.is_special    (),
            Self::UrlIsFile           => task_state.url.is_file       (),
            Self::UrlIsNonSpecial     => task_state.url.is_non_special(),

            // Scheme

            Self::SchemeIs     (value ) => task_state.url.scheme_str() == get!(value),
            Self::SchemeIsInSet(set   ) => get!(set).contains_some(task_state.url.scheme_str()),
            Self::SchemeIsHttp        => task_state.url.scheme_details().is_http         (),
            Self::SchemeIsHttps       => task_state.url.scheme_details().is_https        (),
            Self::SchemeIsHttpOrHttps => task_state.url.scheme_details().is_http_or_https(),

            // Host is

            Self::HostIs        (x) => task_state.url.host_str() == get!(?&x),
            Self::DomainPrefixIs(x) => task_state.url.domain_prefix().map(DomainSegments::decode) == get!(?x),
            Self::DomainMiddleIs(x) => task_state.url.domain_middle().map(DomainSegment ::decode) == get!(?x),
            Self::DomainSuffixIs(x) => task_state.url.domain_suffix().map(DomainSegments::decode) == get!(?x),
            Self::DomainOriginIs(x) => task_state.url.domain_origin().map(DomainSegments::decode) == get!(?x),
            Self::DomainNormalIs(x) => task_state.url.domain_normal().map(DomainSegments::decode) == get!(?x),

            Self::DomainSegmentIs       {index, value} => task_state.url.domain_segment       (*index).map(DomainSegment::decode) == get!(?value),
            Self::DomainPrefixSegmentIs {index, value} => task_state.url.domain_prefix_segment(*index).map(DomainSegment::decode) == get!(?value),
            Self::DomainSuffixSegmentIs {index, value} => task_state.url.domain_suffix_segment(*index).map(DomainSegment::decode) == get!(?value),
            Self::DomainOriginSegmentIs {index, value} => task_state.url.domain_origin_segment(*index).map(DomainSegment::decode) == get!(?value),

            // Host is in set

            Self::HostIsInSet        (set) => get!(set).contains(task_state.url.host_str()),
            Self::DomainPrefixIsInSet(set) => get!(set).contains(task_state.url.domain_prefix().map(DomainSegments::decode).as_deref()),
            Self::DomainMiddleIsInSet(set) => get!(set).contains(task_state.url.domain_middle().map(DomainSegment ::decode).as_deref()),
            Self::DomainSuffixIsInSet(set) => get!(set).contains(task_state.url.domain_suffix().map(DomainSegments::decode).as_deref()),
            Self::DomainOriginIsInSet(set) => get!(set).contains(task_state.url.domain_origin().map(DomainSegments::decode).as_deref()),
            Self::DomainNormalIsInSet(set) => get!(set).contains(task_state.url.domain_normal().map(DomainSegments::decode).as_deref()),

            Self::DomainSegmentIsInSet       {index, set} => get!(set).contains(task_state.url.domain_segment       (*index).map(DomainSegment::decode).as_deref()),
            Self::DomainPrefixSegmentIsInSet {index, set} => get!(set).contains(task_state.url.domain_prefix_segment(*index).map(DomainSegment::decode).as_deref()),
            Self::DomainSuffixSegmentIsInSet {index, set} => get!(set).contains(task_state.url.domain_suffix_segment(*index).map(DomainSegment::decode).as_deref()),
            Self::DomainOriginSegmentIsInSet {index, set} => get!(set).contains(task_state.url.domain_origin_segment(*index).map(DomainSegment::decode).as_deref()),

            // Misc. host

            Self::UrlHasHost   => task_state.url.has_host(),
            Self::HostIsDomain => task_state.url.host_is_domain(),
            Self::HostIsIp     => task_state.url.host_is_ip    (),
            Self::HostIsIpv4   => task_state.url.host_is_ipv4  (),
            Self::HostIsIpv6   => task_state.url.host_is_ipv6  (),
            Self::HostIsOpaque => task_state.url.host_is_opaque(),
            Self::HostIsEmpty  => task_state.url.host_is_empty (),

            Self::UrlHasDomainPrefix => task_state.url.has_domain_prefix(),
            Self::UrlHasDomainMiddle => task_state.url.has_domain_middle(),
            Self::UrlHasDomainSuffix => task_state.url.has_domain_suffix(),
            Self::UrlHasDomainOrigin => task_state.url.has_domain_origin(),
            Self::UrlHasDomainLabels => task_state.url.has_domain_labels(),
            Self::UrlHasDomainNormal => task_state.url.has_domain_normal(),

            Self::HostIsLoopbackIp    => task_state.url.ip_details().is_some_and(IpDetails::is_loopback   ),
            Self::HostIsMulticastIp   => task_state.url.ip_details().is_some_and(IpDetails::is_multicast  ),
            Self::HostIsUnspecifiedIp => task_state.url.ip_details().is_some_and(IpDetails::is_unspecified),

            Self::HostIsBroadcastIpv4     => task_state.url.ipv4_details().is_some_and(Ipv4Details::is_broadcast    ),
            Self::HostIsDocumentationIpv4 => task_state.url.ipv4_details().is_some_and(Ipv4Details::is_documentation),
            Self::HostIsLinkLocalIpv4     => task_state.url.ipv4_details().is_some_and(Ipv4Details::is_link_local   ),
            Self::HostIsLoopbackIpv4      => task_state.url.ipv4_details().is_some_and(Ipv4Details::is_loopback     ),
            Self::HostIsMulticastIpv4     => task_state.url.ipv4_details().is_some_and(Ipv4Details::is_multicast    ),
            Self::HostIsPrivateIpv4       => task_state.url.ipv4_details().is_some_and(Ipv4Details::is_private      ),
            Self::HostIsUnspecifiedIpv4   => task_state.url.ipv4_details().is_some_and(Ipv4Details::is_unspecified  ),

            Self::HostIsLoopbackIpv6         => task_state.url.ipv6_details().is_some_and(Ipv6Details::is_loopback          ),
            Self::HostIsUnicastLinkLocalIpv6 => task_state.url.ipv6_details().is_some_and(Ipv6Details::is_unicast_link_local),
            Self::HostIsMulticastIpv6        => task_state.url.ipv6_details().is_some_and(Ipv6Details::is_multicast         ),
            Self::HostIsUniqueLocalIpv6      => task_state.url.ipv6_details().is_some_and(Ipv6Details::is_unique_local      ),
            Self::HostIsUnspecifiedIpv6      => task_state.url.ipv6_details().is_some_and(Ipv6Details::is_unspecified       ),

            // Path

            Self::PathIsSegmented       => task_state.url.path_is_segmented(),
            Self::PathIsOpaque          => task_state.url.path_is_opaque   (),
            Self::PathHasSegment(index) => task_state.url.has_path_segment (*index),

            Self::PathIs        (value        ) => task_state.url.path_str() == get!(value),
            Self::PathIsInSet   (set          ) => get!(set).contains_some(task_state.url.path_str()),
            Self::PathStartsWith(prefix       ) => task_state.url.path_str().starts_with(get!(&prefix   )),
            Self::PathEndsWith  (suffix       ) => task_state.url.path_str().ends_with  (get!(&suffix   )),
            Self::PathContains  {substring, at} => at.check(task_state.url.path_str(),   get!(&substring))?,
            Self::PathMatches   (matcher      ) => matcher.check(task_state, args, Some(task_state.url.path_str()))?,

            Self::PathSegmentIs        {index, value         } => task_state.url.path_segment(*index).map(PathSegment::decode) == get!(?value),
            Self::PathSegmentIsInSet   {index, set           } => set.get_some(task_state, args)??.contains(task_state.url.path_segment(*index).map(PathSegment::decode).as_deref()),
            Self::PathSegmentStartsWith{index, prefix        } => task_state.url.path_segment(*index).ok_or(ConditionError::PathSegmentNotFound)?.decode().starts_with(get!(&prefix   )),
            Self::PathSegmentEndsWith  {index, suffix        } => task_state.url.path_segment(*index).ok_or(ConditionError::PathSegmentNotFound)?.decode().ends_with  (get!(&suffix   )),
            Self::PathSegmentContains  {index, substring , at} => at.check(&task_state.url.path_segment(*index).ok_or(ConditionError::PathSegmentNotFound)?.decode(),  get!(&substring))?,
            Self::PathSegmentMatches   {index, matcher       } => matcher.check(task_state, args, task_state.url.path_segment(*index).map(PathSegment::decode).as_deref())?,

            Self::RawPathSegmentIs        {index, value        } => task_state.url.path_segment(*index).map(PathSegment::into_inner) == get!(?value),
            Self::RawPathSegmentIsInSet   {index, set          } => get!(set).contains(task_state.url.path_segment(*index).map(PathSegment::into_inner).as_deref()),
            Self::RawPathSegmentStartsWith{index, prefix       } => task_state.url.path_segment(*index).ok_or(ConditionError::PathSegmentNotFound)?.into_inner().starts_with(get!(&prefix   )),
            Self::RawPathSegmentEndsWith  {index, suffix       } => task_state.url.path_segment(*index).ok_or(ConditionError::PathSegmentNotFound)?.into_inner().ends_with  (get!(&suffix   )),
            Self::RawPathSegmentContains  {index, substring, at} => at.check(&task_state.url.path_segment(*index).ok_or(ConditionError::PathSegmentNotFound)?.into_inner(),  get!(&substring))?,
            Self::RawPathSegmentMatches   {index, matcher      } => matcher.check(task_state, args, task_state.url.path_segment(*index).map(PathSegment::into_inner).as_deref())?,

            // Query

            Self::QueryIs        (value        ) => task_state.url.query_str() == get!(?&value),
            Self::QueryIsInSet   (set          ) => get!(set).contains(task_state.url.query_str()),
            Self::QueryStartsWith(prefix       ) => task_state.url.query_str().ok_or(ConditionError::QueryIsNone)?.starts_with(get!(&prefix   )),
            Self::QueryEndsWith  (suffix       ) => task_state.url.query_str().ok_or(ConditionError::QueryIsNone)?.ends_with  (get!(&suffix   )),
            Self::QueryContains  {substring, at} => at.check(task_state.url.query_str().ok_or(ConditionError::QueryIsNone)?,   get!(&substring))?,
            Self::QueryMatches   (matcher      ) => matcher.check(task_state, args, task_state.url.query_str())?,

            Self::QueryHasParam(param) => task_state.url.has_query_param(&param.name, param.index),

            Self::QueryParamIs         {param, value        } => task_state.url.query_param(&param.name, param.index).and_then(QuerySegment::into_value) == get!(?value),
            Self::QueryParamIsInSet    {param, set          } => get!(set).contains(task_state.url.query_param(&param.name, param.index).and_then(QuerySegment::into_value).as_deref()),
            Self::QueryParamStartsWith {param, prefix       } => task_state.url.query_param(&param.name, param.index).ok_or(ConditionError::QueryParamNotFound)?.into_value().ok_or(ConditionError::QueryParamNoValue)?.starts_with(get!(&prefix   )),
            Self::QueryParamEndsWith   {param, suffix       } => task_state.url.query_param(&param.name, param.index).ok_or(ConditionError::QueryParamNotFound)?.into_value().ok_or(ConditionError::QueryParamNoValue)?.ends_with  (get!(&suffix   )),
            Self::QueryParamContains   {param, substring, at} => at.check(&task_state.url.query_param(&param.name, param.index).ok_or(ConditionError::QueryParamNotFound)?.into_value().ok_or(ConditionError::QueryParamNoValue)?,  get!(&substring))?,
            Self::QueryParamMatches    {param, matcher      } => matcher.check(task_state, args, task_state.url.query_param(&param.name, param.index).and_then(QuerySegment::into_value).as_deref())?,

            Self::RawQueryParamIs         {param, value        } => task_state.url.query_param(&param.name, param.index).and_then(QuerySegment::into_raw_value) == get!(?value),
            Self::RawQueryParamIsInSet    {param, set          } => get!(set).contains(task_state.url.query_param(&param.name, param.index).and_then(QuerySegment::into_raw_value).as_deref()),
            Self::RawQueryParamStartsWith {param, prefix       } => task_state.url.query_param(&param.name, param.index).ok_or(ConditionError::QueryParamNotFound)?.into_raw_value().ok_or(ConditionError::QueryParamNoValue)?.starts_with(get!(&prefix   )),
            Self::RawQueryParamEndsWith   {param, suffix       } => task_state.url.query_param(&param.name, param.index).ok_or(ConditionError::QueryParamNotFound)?.into_raw_value().ok_or(ConditionError::QueryParamNoValue)?.ends_with  (get!(&suffix   )),
            Self::RawQueryParamContains   {param, substring, at} => at.check(&task_state.url.query_param(&param.name, param.index).ok_or(ConditionError::QueryParamNotFound)?.into_raw_value().ok_or(ConditionError::QueryParamNoValue)?,  get!(&substring))?,
            Self::RawQueryParamMatches    {param, matcher      } => matcher.check(task_state, args, task_state.url.query_param(&param.name, param.index).and_then(QuerySegment::into_raw_value).as_deref())?,

            // Fragment

            Self::FragmentIs     (value  ) => task_state.url.fragment_str() == get!(?&value),
            Self::FragmentIsInSet(set    ) => get!(set).contains(task_state.url.fragment_str()),
            Self::FragmentMatches(matcher) => matcher.check(task_state, args, task_state.url.fragment_str())?,

            // Misc

            Self::Function(call) => task_state.job.cleaner.functions.conditions
                .get(&call.name).ok_or(FunctionNotFound)?
                .check(task_state, Some(&call.args))?,

            Self::FunctionArg(name) => args.ok_or(NotInFunction)?.conditions
                .get(get!(&name)).ok_or(FunctionArgFunctionNotFound)?
                .check(task_state, args)?,

            Self::Extern(function) => function(task_state, args)?
        })
    }
}
