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
        /// Defaulted.
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
        r#else: Box<Self>,
    },
    /// The first contained [`Self`] to return [`Ok`].
    /// # Errors
    /// If no contained [`Self`] returns [`Ok`], returns the error [`FirstNotErrorErrors`] containing every error.
    FirstNotError(Vec<Self>),

    // Maps

    /// [`UrlPart::get`] + [`Map::get`].
    PartMap {
        /// The [`UrlPart`].
        part: UrlPart,
        /// The [`Map`].
        ///
        /// Flattened.
        #[serde(flatten)]
        map: Box<Map<Self>>,
        /// The else.
        ///
        /// Defaulted.
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Box<Self>,
    },
    /// [`StringSource::get`] + [`Map::get`].
    StringMap {
        /// The [`StringSource`].
        value: StringSource,
        /// The [`Map`].
        ///
        /// Flattened
        #[serde(flatten)]
        map: Box<Map<Self>>,
        /// The else.
        ///
        /// Defaulted.
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Box<Self>,
    },
    /// [`PartitioningSource::get`] + [`Partitioning::get`] + [`UrlPart::get`] + [`Map::get`].
    PartPartitioning {
        /// The [`Partitioning`].
        partitioning: PartitioningSource,
        /// The [`UrlPart`].
        part: UrlPart,
        /// The [`Map`].
        ///
        /// Flattened.
        #[serde(flatten)]
        map: Box<Map<Self>>,
        /// The else.
        ///
        /// Defaulted.
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Box<Self>,
    },
    /// [`PartitioningSource::get`] + [`Partitioning::get`] + [`StringSource::get`] + [`Map::get`].
    StringPartitioning {
        /// The [`Partitioning`].
        partitioning: PartitioningSource,
        /// The [`StringSource`].
        value: StringSource,
        /// The [`Map`].
        ///
        /// Flattened
        #[serde(flatten)]
        map: Box<Map<Self>>,
        /// The else.
        ///
        /// Defaulted.
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Box<Self>,
    },

    // Params

    /// [`FlagSource::get`].
    FlagIsSet(FlagSource),
    /// [`FlagSource::get`] inverted.
    FlagIsNotSet(FlagSource),

    /// [`VarSource::get`] + [`Option::is_some`].
    VarIsSome(VarSource),

    // String source

    /// If [`Self::StringIs::left`] is [`Self::StringIs::right`].
    StringIs {
        /// The left side.
        left: StringSource,
        /// The right side.
        right: StringSource,
    },
    /// If [`Self::StringIsInSet::value`] is in [`Self::StringIsInSet::set`].
    StringIsInSet {
        /// The [`StringSource`].
        value: StringSource,
        /// The [`SetSource`].
        set: SetSource,
    },
    /// If [`Self::StringStartsWith::value`] starts with [`Self::StringStartsWith::prefix`].
    StringStartsWith {
        /// The [`StringSource`] to search in.
        value: StringSource,
        /// The [`StringSource`] to search for.
        prefix: StringSource,
    },
    /// If [`Self::StringEndsWith::value`] starts with [`Self::StringEndsWith::suffix`].
    StringEndsWith {
        /// The [`StringSource`] to search in.
        value: StringSource,
        /// The [`StringSource`] to search for.
        suffix: StringSource,
    },
    /// [`StringLocation::check`].
    StringContains {
        /// The value to search in.
        value: StringSource,
        /// The value to search for.
        substr: StringSource,
        /// The [`StringLocation`].
        ///
        /// Defaulted.
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation,
    },
    /// [`StringMatcher::check`].
    StringMatches {
        /// The value to check.
        value: StringSource,
        /// The [`StringMatcher`].
        matcher: StringMatcher,
    },

    // Whole

    /** [`BetterUrl::is_special`].          */ UrlIsSpecial,
    /** [`BetterUrl::is_special_not_file`]. */ UrlIsSpecialNotFile,
    /** [`BetterUrl::is_file`].             */ UrlIsFile,
    /** [`BetterUrl::is_non_special`].      */ UrlIsNonSpecial,

    // Scheme

    /** [`BetterUrl::scheme_str`] + [`PartialEq::eq`].      */ SchemeIs(StringSource),
    /** [`BetterUrl::scheme_str`] + [`Set::contains_some`]. */ SchemeIsInSet(SetSource),
    /** [`SchemeDetails::is_http`].                         */ SchemeIsHttp,
    /** [`SchemeDetails::is_https`].                        */ SchemeIsHttps,
    /** [`SchemeDetails::is_http_or_https`].                */ SchemeIsHttpOrHttps,

    // Host is

    /** [`BetterUrl::host_str`]          + [`PartialEq::eq`]. */ HostIs        (StringSource),
    /** [`BetterUrl::domain_normal_str`] + [`PartialEq::eq`]. */ DomainNormalIs(StringSource),
    /** [`BetterUrl::domain_origin_str`] + [`PartialEq::eq`]. */ DomainOriginIs(StringSource),
    /** [`BetterUrl::domain_prefix_str`] + [`PartialEq::eq`]. */ DomainPrefixIs(StringSource),
    /** [`BetterUrl::domain_middle_str`] + [`PartialEq::eq`]. */ DomainMiddleIs(StringSource),
    /** [`BetterUrl::domain_suffix_str`] + [`PartialEq::eq`]. */ DomainSuffixIs(StringSource),



    /// If the [`BetterUrl::domain_segment`] is the specified value.
    DomainSegmentIs {
        /// The segment to check.
        index: isize,
        /// The value to check for.
        value: StringSource,
    },
    /// If the [`BetterUrl::domain_prefix_segment`] is the specified value.
    DomainPrefixSegmentIs {
        /// The segment to check.
        index: isize,
        /// The value to check for.
        value: StringSource,
    },
    /// If the [`BetterUrl::domain_suffix_segment`] is the specified value.
    DomainSuffixSegmentIs {
        /// The segment to check.
        index: isize,
        /// The value to check for.
        value: StringSource,
    },
    /// If the [`BetterUrl::domain_origin_segment`] is the specified value.
    DomainOriginSegmentIs {
        /// The segment to check.
        index: isize,
        /// The value to check for.
        value: StringSource,
    },

    // Host is in set

    /** [`BetterUrl::host_str`]          + [`Set::contains`]. */ HostIsInSet        (SetSource),
    /** [`BetterUrl::domain_normal_str`] + [`Set::contains`]. */ DomainNormalIsInSet(SetSource),
    /** [`BetterUrl::domain_prefix_str`] + [`Set::contains`]. */ DomainPrefixIsInSet(SetSource),
    /** [`BetterUrl::domain_origin_str`] + [`Set::contains`]. */ DomainOriginIsInSet(SetSource),
    /** [`BetterUrl::domain_middle_str`] + [`Set::contains`]. */ DomainMiddleIsInSet(SetSource),
    /** [`BetterUrl::domain_suffix_str`] + [`Set::contains`]. */ DomainSuffixIsInSet(SetSource),



    /// If the [`BetterUrl::domain_segment`] is in the specified [`Set`].
    DomainSegmentIsInSet {
        /// The segment to check.
        index: isize,
        /// The [`Set`] to check in.
        set: SetSource,
    },
    /// If the [`BetterUrl::domain_origin_segment`] is in the specified [`Set`].
    DomainOriginSegmentIsInSet {
        /// The segment to check.
        index: isize,
        /// The [`Set`] to check in.
        set: SetSource,
    },
    /// If the [`BetterUrl::domain_prefix_segment`] is in the specified [`Set`].
    DomainPrefixSegmentIsInSet {
        /// The segment to check.
        index: isize,
        /// The [`Set`] to check in.
        set: SetSource,
    },
    /// If the [`BetterUrl::domain_suffix_segment`] is in the specified [`Set`].
    DomainSuffixSegmentIsInSet {
        /// The segment to check.
        index: isize,
        /// The [`Set`] to check in.
        set: SetSource,
    },

    // Misc. host

    /** [`BetterUrl::has_host`].       */ UrlHasHost,
    /** [`BetterUrl::host_is_domain`]. */ HostIsDomain,
    /** [`BetterUrl::host_is_ipv4`].   */ HostIsIpv4,
    /** [`BetterUrl::host_is_ipv6`].   */ HostIsIpv6,
    /** [`BetterUrl::host_is_opaque`]. */ HostIsOpaque,
    /** [`BetterUrl::host_is_empty`].  */ HostIsEmpty,

    /** [`BetterUrl::has_domain_prefix`]. */ UrlHasDomainPrefix,
    /** [`BetterUrl::has_domain_middle`]. */ UrlHasDomainMiddle,
    /** [`BetterUrl::has_domain_suffix`]. */ UrlHasDomainSuffix,
    /** [`BetterUrl::has_domain_origin`]. */ UrlHasDomainOrigin,
    /** [`BetterUrl::has_domain_labels`]. */ UrlHasDomainLabels,
    /** [`BetterUrl::has_domain_normal`]. */ UrlHasDomainNormal,

    /** [`Ipv4HostDetails::is_broadcast`].     */ HostIsBroadcastIpv4,
    /** [`Ipv4HostDetails::is_documentation`]. */ HostIsDocumentationIpv4,
    /** [`Ipv4HostDetails::is_link_local`].    */ HostIsLinkLocalIpv4,
    /** [`Ipv4HostDetails::is_loopback`].      */ HostIsLoopbackIpv4,
    /** [`Ipv4HostDetails::is_multicast`].     */ HostIsMulticastIpv4,
    /** [`Ipv4HostDetails::is_private`].       */ HostIsPrivateIpv4,
    /** [`Ipv4HostDetails::is_unspecified`].   */ HostIsUnspecifiedIpv4,

    /** [`Ipv6HostDetails::is_loopback`].           */ HostIsLoopbackIpv6,
    /** [`Ipv6HostDetails::is_unicast_link_local`]. */ HostIsUnicastLinkLocalIpv6,
    /** [`Ipv6HostDetails::is_multicast`].          */ HostIsMulticastIpv6,
    /** [`Ipv6HostDetails::is_unique_local`].       */ HostIsUniqueLocalIpv6,
    /** [`Ipv6HostDetails::is_unspecified`].        */ HostIsUnspecifiedIpv6,

    // Path

    /** [`BetterUrl::path_is_segmented`]. */ PathIsSegmented,
    /** [`BetterUrl::path_is_opaque`].    */ PathIsOpaque,
    /** [`BetterUrl::has_path_segment`].  */ PathHasSegment(isize),

    /** If [`BetterUrl::path_str`] + [`PartialEq::eq`].      */ PathIs        (StringSource),
    /** If [`BetterUrl::path_str`] + [`Set::contains_some`]. */ PathIsInSet   (SetSource),
    /** If [`BetterUrl::path_str`] + [`str::starts_with`].   */ PathStartsWith(StringSource),
    /** If [`BetterUrl::path_str`] + [`str::ends_with`].     */ PathEndsWith  (StringSource),
    /// If [`BetterUrl::path_str`] + [`StringLocation::check`].
    PathContains {
        /// The value to check for.
        substr: StringSource,
        /// The [`StringLocation`].
        ///
        /// Defaulted.
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation,
    },
    /// If [`BetterUrl::path_str`] satisfies the specified [`StringMatcher`].
    PathMatches(StringMatcher),

    // Path segment

    /// [`BetterUrl::path_segment`] + [`PathSegment::lossy_decode`] + [`PartialEq::eq`].
    PathSegmentIs {
        /// The segment to check.
        index: isize,
        /// The value to check for.
        value: StringSource,
    },
    /// [`BetterUrl::path_segment`] + [`PathSegment::lossy_decode`] + [`Set::contains`].
    PathSegmentIsInSet {
        /// The segment to check.
        index: isize,
        /// The [`Set`] to check in.
        set: SetSource,
    },
    /// [`BetterUrl::path_segment`] + [`PathSegment::lossy_decode`] + [`str::starts_with`].
    /// # Errors
    /// If [`BetterUrl::path_segment`] reutrns [`None`], returns the error [`PathSegmentNotFound`].
    PathSegmentStartsWith {
        /// The segment to check.
        index: isize,
        /// The value to check for.
        prefix: StringSource,
    },
    /// [`BetterUrl::path_segment`] + [`PathSegment::lossy_decode`] + [`str::ends_with`].
    /// # Errors
    /// If [`BetterUrl::path_segment`] reutrns [`None`], returns the error [`PathSegmentNotFound`].
    PathSegmentEndsWith {
        /// The segment to check.
        index: isize,
        /// The value to check for.
        suffix: StringSource,
    },
    /// [`BetterUrl::path_segment`] + [`PathSegment::lossy_decode`] + [`StringLocation::check`].
    /// # Errors
    /// If [`BetterUrl::path_segment`] reutrns [`None`], returns the error [`PathSegmentNotFound`].
    PathSegmentContains {
        /// The path segment to get.
        index: isize,
        /// The value to check for.
        substr: StringSource,
        /// The location to cehck at.
        ///
        /// Defaulted.
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation,
    },
    /// If the [`BetterUrl::path_segment`] + [`PathSegment::lossy_decode`] satisfies the [`StringMatcher`].
    PathSegmentMatches {
        /// The segment to check.
        index: isize,
        /// The [`StringMatcher`].
        matcher: StringMatcher,
    },

    // Query

    /// [`BetterUrl::has_query_param`].
    QueryHasParam(QueryParamSelector),

    // Query params

    /// [`BetterUrl::query_param`] + [`QuerySegment::into_value`] ([`Option::flatten`]ed) + [`MaybeQueryValue::lossy_decode`] + [`PartialEq::eq`].
    QueryParamIs {
        /// The [`QueryParamSelector`].
        param: QueryParamSelector,
        /// The value to check for.
        value: StringSource,
    },
    /// [`BetterUrl::query_param`] + [`QuerySegment::into_value`] ([`Option::flatten`]ed) + [`MaybeQueryValue::lossy_decode`] + [`Set::contains`].
    QueryParamIsInSet {
        /// The [`QueryParamSelector`].
        param: QueryParamSelector,
        /// The [`Set`].
        set: SetSource,
    },
    /// If the [`BetterUrl::path_segment`] + [`QuerySegment::into_value`] ([`Option::flatten`]ed) + [`MaybeQueryValue::lossy_decode`] starts with the specified value.
    /// # Errors
    /// If [`BetterUrl::path_segment`] reutrns [`None`], returns the error [`QueryParamNotFound`].
    ///
    /// If [`QuerySegment::into_value`] reutrns [`None`], returns the error [`QueryParamNotFound`].
    QueryParamStartsWith {
        /// The [`QueryParamSelector`].
        param: QueryParamSelector,
        /// The value to check for.
        prefix: StringSource,
    },
    /// If the [`BetterUrl::path_segment`] + [`QuerySegment::into_value`] ([`Option::flatten`]ed) + [`MaybeQueryValue::lossy_decode`] ends with the specified value.
    /// # Errors
    /// If [`BetterUrl::path_segment`] reutrns [`None`], returns the error [`QueryParamNotFound`].
    ///
    /// If [`QuerySegment::into_value`] reutrns [`None`], returns the error [`QueryParamNotFound`].
    QueryParamEndsWith {
        /// The [`QueryParamSelector`].
        param: QueryParamSelector,
        /// The value to check for.
        suffix: StringSource,
    },
    /// If the [`BetterUrl::path_segment`] + [`QuerySegment::into_value`] ([`Option::flatten`]ed) + [`MaybeQueryValue::lossy_decode`] contains the specified value.
    /// # Errors
    /// If [`BetterUrl::path_segment`] reutrns [`None`], returns the error [`QueryParamNotFound`].
    ///
    /// If [`QuerySegment::into_value`] reutrns [`None`], returns the error [`QueryParamNotFound`].
    QueryParamContains {
        /// The [`QueryParamSelectorget.
        param: QueryParamSelector,
        /// The value to check for.
        substr: StringSource,
        /// The location to cehck at.
        ///
        /// Defaulted.
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation,
    },
    /// [`BetterUrl::query_param`] + [`QuerySegment::into_value`] ([`Option::flatten`]) + [`MaybeQueryValue::lossy_decode`] +ed [`StringMatcher::check`]
    /// # Errors
    /// If [`BetterUrl::query_param`] reutrns [`None`], returns the error [`QueryParamNotFound`].
    ///
    /// If [`QuerySegment::into_value`] reutrns [`None`], returns the error [`QueryParamNotFound`].
    QueryParamMatches {
        /// The [`QueryParamSelector`].
        param: QueryParamSelector,
        /// The [`StringMatcher`]
        matcher: StringMatcher,
    },

    // Fragment

    /// [`BetterUrl::fragment_str`] + [`PartialEq::eq`].
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
        value: StringSource,
    },
    /// If [`Self::PartIsInSet::part`] is in [`Self::PartIsInSet::set`].
    PartIsInSet {
        /// The [`UrlPart`].
        part: UrlPart,
        /// The [`SetSource`].
        set: SetSource,
    },
    /// If [`Self::PartStartsWith::part`] starts with [`Self::PartStartsWith::prefix`].
    PartStartsWith {
        /// The [`UrlPart`].
        part: UrlPart,
        /// The [`StringSource`].
        prefix: StringSource,
    },
    /// If [`Self::PartEndsWith::part`] starts with [`Self::PartEndsWith::suffix`].
    PartEndsWith {
        /// The [`UrlPart`].
        part: UrlPart,
        /// The [`StringSource`].
        suffix: StringSource,
    },
    /// If [`Self::PartContains::part`] contains [`Self::PartContains::substr`] at [`Self::PartContains::at`].
    PartContains {
        /// The [`UrlPart`].
        part: UrlPart,
        /// The [`StringSource`].
        substr: StringSource,
        /// The [`StringLocation`].
        ///
        /// Defaulted.
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation,
    },
    /// If [`Self::PartMatches::part`] matches [`Self::PartMatches::matcher`].
    PartMatches {
        /// The [`UrlPart`].
        part: UrlPart,
        /// The [`StringMatcher`].
        matcher: StringMatcher,
    },

    /// If [`Self::PartIsSomeAndStartsWith::part`] is [`Some`] and starts with [`Self::PartIsSomeAndStartsWith::prefix`].
    PartIsSomeAndStartsWith {
        /// The [`UrlPart`].
        part: UrlPart,
        /// the [`StringSource`].
        prefix: StringSource,
    },
    /// If [`Self::PartIsSomeAndEndsWith::part`] is [`Some`] and starts with [`Self::PartIsSomeAndEndsWith::suffix`].
    PartIsSomeAndEndsWith {
        /// The [`UrlPart`].
        part: UrlPart,
        /// the [`StringSource`].
        suffix: StringSource,
    },
    /// If [`Self::PartIsSomeAndContains::part`] is [`Some`] and contains [`Self::PartIsSomeAndContains::substr`] at [`Self::PartIsSomeAndContains::at`].
    PartIsSomeAndContains {
        /// The [`UrlPart`].
        part: UrlPart,
        /// the [`StringSource`].
        substr: StringSource,
        /// The [`StringLocation`].
        ///
        /// Defaulted.
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation,
    },

    // Misc.

    /// Uses a [`Self`] from [`Cleaner::functions`].
    Function(Box<FunctionCall>),
    /// Uses a [`Self`] from [`FunctionArgs`].
    FunctionArg(StringSource),
    /// Calls the specified function and returns its value.
    ///
    /// Because this uses function pointers, this plays weirdly with [`PartialEq`]/[`Eq`].
    ///
    /// Additionally, using a function pointer means this variant cannot be [`Serialize`]d or [`Deserialize`]d.
    #[suitable(never)]
    #[serde(skip)]
    Extern(ConditionExtern),
}

impl Condition {
    /// If it's satisfied.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn check<'j>(&'j self, ts: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<bool, ConditionError> {
        Ok(match self {
            // Debug/constants

            Self::Always => true,
            Self::Never => false,
            Self::Error(msg) => Err(ExplicitError(msg.clone()))?,

            // Logic

            Self::If {r#if, then, r#else} => match r#if.check(ts, args)? {
                true  => then  .check(ts, args)?,
                false => r#else.check(ts, args)?
            },
            Self::Not(condition) => !condition.check(ts, args)?,
            Self::All(conditions) => {
                for condition in conditions {
                    if !condition.check(ts, args)? {
                        return Ok(false);
                    }
                }
                true
            },
            Self::Any(conditions) => {
                for condition in conditions {
                    if condition.check(ts, args)? {
                        return Ok(true);
                    }
                }
                false
            },

            // Error handling

            Self::ErrorToSatisfied  (condition) => condition.check(ts, args).unwrap_or(true ),
            Self::ErrorToUnsatisfied(condition) => condition.check(ts, args).unwrap_or(false),
            Self::TryElse {r#try, r#else} => match r#try.check(ts, args) {
                Ok(x) => x,
                Err(try_error) => match r#else.check(ts, args) {
                    Ok(x) => x,
                    Err(else_error) => Err(TryElseError {try_error, else_error})?
                }
            },
            Self::FirstNotError(matchers) => {
                let mut errors = Vec::new();

                for matcher in matchers {
                    match matcher.check(ts, args) {
                        Ok (x) => return Ok(x),
                        Err(e) => errors.push(e)
                    }
                }

                Err(FirstNotErrorErrors(errors))?
            },

            // Maps

            Self::PartMap  {part , map, r#else} => map.get(part .get(&ts.url ) .as_deref()).unwrap_or(r#else).check(ts, args)?,
            Self::StringMap{value, map, r#else} => map.get(value.get(ts, args)?.as_deref()).unwrap_or(r#else).check(ts, args)?,

            Self::PartPartitioning   {partitioning, part , map, r#else} => map.get(partitioning.get_some(ts, args)??.get(part .get(&ts.url ) .as_deref())).unwrap_or(r#else).check(ts, args)?,
            Self::StringPartitioning {partitioning, value, map, r#else} => map.get(partitioning.get_some(ts, args)??.get(value.get(ts, args)?.as_deref())).unwrap_or(r#else).check(ts, args)?,

            // Params

            Self::FlagIsSet   (flag) =>  flag.get(ts, args)?,
            Self::FlagIsNotSet(flag) => !flag.get(ts, args)?,

            Self::VarIsSome(var) => var.get(ts, args)?.is_some(),

            // Strings

            Self::StringIs         {left , right     } =>                                    left .get     (ts, args)? == right.get(ts, args)?,
            Self::StringStartsWith {value, prefix    } =>                                    value.get_some(ts, args)??.starts_with(&*prefix.get_some(ts, args)??) ,
            Self::StringEndsWith   {value, suffix    } =>                                    value.get_some(ts, args)??.ends_with  (&*suffix.get_some(ts, args)??) ,
            Self::StringContains   {value, substr, at} => at.check(                         &value.get_some(ts, args)??,            & substr.get_some(ts, args)??)?,
            Self::StringIsInSet    {value, set       } => set.get_some(ts, args)??.contains( value.get     (ts, args)?.as_deref()) ,
            Self::StringMatches    {value, matcher   } => matcher.check(ts, args,            value.get     (ts, args)?.as_deref())?,

            // Parts

            Self::PartIs         {part, value     } =>                                    part.get     (&ts.url) == value.get(ts, args)?,
            Self::PartStartsWith {part, prefix    } =>                                    part.get_some(&ts.url)?.starts_with(&*prefix.get_some(ts, args)??),
            Self::PartEndsWith   {part, suffix    } =>                                    part.get_some(&ts.url)?.ends_with  (&*suffix.get_some(ts, args)??),
            Self::PartContains   {part, substr, at} => at.check(                         &part.get_some(&ts.url)?,            & substr.get_some(ts, args)??)?,
            Self::PartIsInSet    {part, set       } => set.get_some(ts, args)??.contains( part.get     (&ts.url).as_deref()) ,
            Self::PartMatches    {part, matcher   } => matcher.check(ts, args,            part.get     (&ts.url).as_deref())?,

            Self::PartIsSomeAndStartsWith {part, prefix    } => if let Some(x) = part.get(&ts.url) {x.starts_with(&*prefix.get_some(ts, args)??) } else {false},
            Self::PartIsSomeAndEndsWith   {part, suffix    } => if let Some(x) = part.get(&ts.url) {x.ends_with  (&*suffix.get_some(ts, args)??) } else {false},
            Self::PartIsSomeAndContains   {part, substr, at} => if let Some(x) = part.get(&ts.url) {at.check(&x,  & substr.get_some(ts, args)??)?} else {false},

            // Whole

            Self::UrlIsSpecial        => ts.url.is_special    (),
            Self::UrlIsSpecialNotFile => ts.url.is_special    (),
            Self::UrlIsFile           => ts.url.is_file       (),
            Self::UrlIsNonSpecial     => ts.url.is_non_special(),

            // Scheme

            Self::SchemeIs     (value) => ts.url.scheme_str() == value.get_some(ts, args)??,
            Self::SchemeIsInSet(set  ) => set.get_some(ts, args)??.contains_some(ts.url.scheme_str()),
            Self::SchemeIsHttp         => ts.url.scheme_details().is_http         (),
            Self::SchemeIsHttps        => ts.url.scheme_details().is_https        (),
            Self::SchemeIsHttpOrHttps  => ts.url.scheme_details().is_http_or_https(),

            // Host is

            Self::HostIs        (x) => ts.url.host_str         () == x.get(ts, args)?.as_deref(),
            Self::DomainPrefixIs(x) => ts.url.domain_prefix_str() == x.get(ts, args)?.as_deref(),
            Self::DomainMiddleIs(x) => ts.url.domain_middle_str() == x.get(ts, args)?.as_deref(),
            Self::DomainSuffixIs(x) => ts.url.domain_suffix_str() == x.get(ts, args)?.as_deref(),
            Self::DomainOriginIs(x) => ts.url.domain_origin_str() == x.get(ts, args)?.as_deref(),
            Self::DomainNormalIs(x) => ts.url.domain_normal_str() == x.get(ts, args)?.as_deref(),

            Self::DomainSegmentIs       {index, value} => ts.url.domain_segment_str       (*index) == value.get(ts, args)?.as_deref(),
            Self::DomainPrefixSegmentIs {index, value} => ts.url.domain_prefix_segment_str(*index) == value.get(ts, args)?.as_deref(),
            Self::DomainSuffixSegmentIs {index, value} => ts.url.domain_suffix_segment_str(*index) == value.get(ts, args)?.as_deref(),
            Self::DomainOriginSegmentIs {index, value} => ts.url.domain_origin_segment_str(*index) == value.get(ts, args)?.as_deref(),

            // Host is in set

            Self::HostIsInSet        (set) => set.get_some(ts, args)??.contains(ts.url.host_str         ()),
            Self::DomainPrefixIsInSet(set) => set.get_some(ts, args)??.contains(ts.url.domain_prefix_str()),
            Self::DomainMiddleIsInSet(set) => set.get_some(ts, args)??.contains(ts.url.domain_middle_str()),
            Self::DomainSuffixIsInSet(set) => set.get_some(ts, args)??.contains(ts.url.domain_suffix_str()),
            Self::DomainOriginIsInSet(set) => set.get_some(ts, args)??.contains(ts.url.domain_origin_str()),
            Self::DomainNormalIsInSet(set) => set.get_some(ts, args)??.contains(ts.url.domain_normal_str()),

            Self::DomainSegmentIsInSet       {index, set} => set.get_some(ts, args)??.contains(ts.url.domain_segment_str       (*index)),
            Self::DomainPrefixSegmentIsInSet {index, set} => set.get_some(ts, args)??.contains(ts.url.domain_prefix_segment_str(*index)),
            Self::DomainSuffixSegmentIsInSet {index, set} => set.get_some(ts, args)??.contains(ts.url.domain_suffix_segment_str(*index)),
            Self::DomainOriginSegmentIsInSet {index, set} => set.get_some(ts, args)??.contains(ts.url.domain_origin_segment_str(*index)),

            // Misc. host

            Self::UrlHasHost   => ts.url.has_host(),
            Self::HostIsDomain => ts.url.host_is_domain(),
            Self::HostIsIpv4   => ts.url.host_is_ipv4  (),
            Self::HostIsIpv6   => ts.url.host_is_ipv6  (),
            Self::HostIsOpaque => ts.url.host_is_opaque(),
            Self::HostIsEmpty  => ts.url.host_is_empty (),

            Self::UrlHasDomainPrefix => ts.url.has_domain_prefix(),
            Self::UrlHasDomainMiddle => ts.url.has_domain_middle(),
            Self::UrlHasDomainSuffix => ts.url.has_domain_suffix(),
            Self::UrlHasDomainOrigin => ts.url.has_domain_origin(),
            Self::UrlHasDomainLabels => ts.url.has_domain_labels(),
            Self::UrlHasDomainNormal => ts.url.has_domain_normal(),

            Self::HostIsBroadcastIpv4     => ts.url.ipv4_details().is_some_and(Ipv4HostDetails::is_broadcast    ),
            Self::HostIsDocumentationIpv4 => ts.url.ipv4_details().is_some_and(Ipv4HostDetails::is_documentation),
            Self::HostIsLinkLocalIpv4     => ts.url.ipv4_details().is_some_and(Ipv4HostDetails::is_link_local   ),
            Self::HostIsLoopbackIpv4      => ts.url.ipv4_details().is_some_and(Ipv4HostDetails::is_loopback     ),
            Self::HostIsMulticastIpv4     => ts.url.ipv4_details().is_some_and(Ipv4HostDetails::is_multicast    ),
            Self::HostIsPrivateIpv4       => ts.url.ipv4_details().is_some_and(Ipv4HostDetails::is_private      ),
            Self::HostIsUnspecifiedIpv4   => ts.url.ipv4_details().is_some_and(Ipv4HostDetails::is_unspecified  ),

            Self::HostIsLoopbackIpv6         => ts.url.ipv6_details().is_some_and(Ipv6HostDetails::is_loopback          ),
            Self::HostIsUnicastLinkLocalIpv6 => ts.url.ipv6_details().is_some_and(Ipv6HostDetails::is_unicast_link_local),
            Self::HostIsMulticastIpv6        => ts.url.ipv6_details().is_some_and(Ipv6HostDetails::is_multicast         ),
            Self::HostIsUniqueLocalIpv6      => ts.url.ipv6_details().is_some_and(Ipv6HostDetails::is_unique_local      ),
            Self::HostIsUnspecifiedIpv6      => ts.url.ipv6_details().is_some_and(Ipv6HostDetails::is_unspecified       ),

            // Path

            Self::PathIsSegmented       => ts.url.path_is_segmented(),
            Self::PathIsOpaque          => ts.url.path_is_opaque   (),
            Self::PathHasSegment(index) => ts.url.has_path_segment (*index),

            Self::PathIs        (value     ) =>                                             ts.url.path_str() == value.get_some(ts, args)??,
            Self::PathStartsWith(prefix    ) =>                                             ts.url.path_str().starts_with(&*prefix.get_some(ts, args)??) ,
            Self::PathEndsWith  (suffix    ) =>                                             ts.url.path_str().ends_with  (&*suffix.get_some(ts, args)??) ,
            Self::PathContains  {substr, at} => at.check(                                   ts.url.path_str(),            & substr.get_some(ts, args)??)?,
            Self::PathIsInSet   (set       ) => set.get_some(ts, args)??.contains_some(     ts.url.path_str())  ,
            Self::PathMatches   (matcher   ) => matcher.check(ts, args,                Some(ts.url.path_str()))?,

            Self::PathSegmentIs        {index, value     } =>                                    ts.url.path_segment(*index).map(PathSegment::lossy_decode) == value.get(ts, args)?,
            Self::PathSegmentStartsWith{index, prefix    } =>                                    ts.url.path_segment(*index).map(PathSegment::lossy_decode).ok_or(PathSegmentNotFound)?.starts_with(&*prefix.get_some(ts, args)??),
            Self::PathSegmentEndsWith  {index, suffix    } =>                                    ts.url.path_segment(*index).map(PathSegment::lossy_decode).ok_or(PathSegmentNotFound)?.ends_with  (&*suffix.get_some(ts, args)??),
            Self::PathSegmentContains  {index, substr, at} => at.check(                         &ts.url.path_segment(*index).map(PathSegment::lossy_decode).ok_or(PathSegmentNotFound)?,            & substr.get_some(ts, args)??)?,
            Self::PathSegmentIsInSet   {index, set       } => set.get_some(ts, args)??.contains( ts.url.path_segment(*index).map(PathSegment::lossy_decode).as_deref()) ,
            Self::PathSegmentMatches   {index, matcher   } => matcher.check(ts, args,            ts.url.path_segment(*index).map(PathSegment::lossy_decode).as_deref())?,

            // Query

            Self::QueryHasParam(param) => ts.url.has_query_param(&param.name, param.index),

            Self::QueryParamIs         {param, value     } =>                                    ts.url.query_param(&param.name, param.index).and_then(|x| x.into_value().lossy_decode()) == value.get(ts, args)?,
            Self::QueryParamStartsWith {param, prefix    } =>                                    ts.url.query_param(&param.name, param.index).and_then(|x| x.into_value().lossy_decode()).ok_or(QueryParamNotFound)?.starts_with(&*prefix.get_some(ts, args)??),
            Self::QueryParamEndsWith   {param, suffix    } =>                                    ts.url.query_param(&param.name, param.index).and_then(|x| x.into_value().lossy_decode()).ok_or(QueryParamNotFound)?.ends_with  (&*suffix.get_some(ts, args)??),
            Self::QueryParamContains   {param, substr, at} => at.check(                         &ts.url.query_param(&param.name, param.index).and_then(|x| x.into_value().lossy_decode()).ok_or(QueryParamNotFound)?,            & substr.get_some(ts, args)??)?,
            Self::QueryParamIsInSet    {param, set       } => set.get_some(ts, args)??.contains( ts.url.query_param(&param.name, param.index).and_then(|x| x.into_value().lossy_decode()).as_deref()) ,
            Self::QueryParamMatches    {param, matcher   } => matcher.check(ts, args,            ts.url.query_param(&param.name, param.index).and_then(|x| x.into_value().lossy_decode()).as_deref())?,

            // Fragment

            Self::FragmentIs     (value  ) => ts.url.fragment_str() == value.get(ts, args)?.as_deref(),
            Self::FragmentIsInSet(set    ) => set.get_some(ts, args)??.contains(ts.url.fragment_str()),
            Self::FragmentMatches(matcher) => matcher.check(ts, args, ts.url.fragment_str())?,

            // Misc

            Self::Function   (call) => ts.job.cleaner.functions.conditions.get(&call.name).ok_or(FunctionNotFound)?.check(ts, Some(&call.args))?,
            Self::FunctionArg(name) => args.ok_or(NotInFunction)?.conditions.get(&*name.get_some(ts, args)??).ok_or(FunctionArgFunctionNotFound)?.check(ts, args)?,
            Self::Extern     (func) => func(ts, args)?
        })
    }
}
