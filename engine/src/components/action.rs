//! [`Action`].

use crate::prelude::*;

/// How to modify a [`TaskState`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub enum Action {
    /// Does nothing.
    #[default]
    None,
    /// [`ExplicitError`].
    /// # Errors
    /// [`ExplicitError`].
    Error(String),

    /// If [`Self::If::if`] then [`Self::If::then`], otherwise [`Self::If::else`].
    If {
        /// The if.
        r#if: Condition,
        /// The then
        then: Box<Self>,
        /// The else.
        ///
        /// Defaulted.
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Box<Self>,
    },
    /// All contained [`Self`]s.
    All(Vec<Self>),
    /// Index the [`Map`] with the [`UrlPart`] and use that or [`Self::PartMap::else`].
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
    /// Index the [`Map`] with the [`StringSource`] and use that or [`Self::StringMap::else`].
    StringMap {
        /// The [`StringSource`].
        value: StringSource,
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
    /// Index the [`Partitioning`] with the [`UrlPart`], index the [`Map`], and use that or [`Self::PartPartitioning::else`].
    PartPartitioning {
        /// The [`PartitioningSource`].
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
    /// Index the [`Partitioning`] with the [`StringSource`], index the [`Map`], and use that or [`Self::StringPartitioning::else`].
    StringPartitioning {
        /// The [`PartitioningSource`].
        partitioning: PartitioningSource,
        /// The [`StringSource`].
        value: StringSource,
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
    /// Index the [`Map`] with the first contained [`Partitioning`] name of the [`UrlPart`]s.
    ///
    /// All calls to [`Map::get`] return [`None`], returns [`false`].
    FirstMappedPartPartitioning {
        /// The [`PartitioningSource`].
        partitioning: PartitioningSource,
        /// The [`UrlPart`]s.
        parts: Vec<UrlPart>,
        /// The [`Map`].
        ///
        /// Flattened.
        #[serde(flatten)]
        map: Box<Map<Self>>,
    },
    /// Index the [`Map`] with the first contained [`Partitioning`] name of the [`StringSource`]s.
    ///
    /// All calls to [`Map::get`] return [`None`], returns [`false`].
    FirstMappedStringPartitioning {
        /// The [`PartitioningSource`].
        partitioning: PartitioningSource,
        /// The [`StringSource`]s.
        values: Vec<StringSource>,
        /// The [`Map`].
        ///
        /// Flattened.
        #[serde(flatten)]
        map: Box<Map<Self>>,
    },



    /// Repeat the contained [`Self`]s until all calls to [`Self::apply`] return [`false`] or the limit is reached.
    Repeat {
        /// The [`Self`]s to repeat.
        actions: Vec<Action>,
        /// The maximum amount of times to repeat.
        ///
        /// Defaults to 10.
        #[serde(default = "get_10_u64")]
        limit: u64,
    },



    /// Map [`Err`] to [`true`] and keeps modifications.
    IgnoreError(Box<Self>),
    /// [`Self::IgnoreError`] + [`Self::RevertOnError`].
    IgnoreAndRevertOnError(Box<Self>),
    /// Reverts the inner [`Self`] if it returns [`Err`], then returns the error.
    RevertOnError(Box<Self>),
    /// [`Self::TryElse::try`], or [`Self::TryElse::else`] if it returns [`Err`].
    TryElse {
        /** The try.  **/ r#try: Box<Self>,
        /** The else. **/ r#else: Box<Self>,
    },
    /// [`Self::All`] but stops after the first [`Ok`].
    /// # Errors
    /// If all return [`Err`], returns the error [`FirstNotErrorErrors`].
    FirstNotError(Vec<Self>),

    // Whole

    /// Sets the whole URL.
    SetWhole(StringSource),
    /// [`BetterUrl::join`].
    Join(StringSource),

    // Scheme

    /// [`BetterUrl::set_scheme`].
    SetScheme(StringSource),

    // Host

    /** [`BetterUrl::set_host`].          **/ SetHost        (StringSource),
    /** [`BetterUrl::set_domain_prefix`]. **/ SetDomainPrefix(StringSource),
    /** [`BetterUrl::set_domain_middle`]. **/ SetDomainMiddle(StringSource),
    /** [`BetterUrl::set_domain_suffix`]. **/ SetDomainSuffix(StringSource),
    /** [`BetterUrl::set_domain_origin`]. **/ SetDomainOrigin(StringSource),

    /// [`BetterUrl::set_domain_segment`].
    SetDomainSegment {
        /** The index. **/ index: isize,
        /** The value. **/ value: StringSource,
    },
    /// [`BetterUrl::set_domain_prefix_segment`].
    SetDomainPrefixSegment {
        /** The index. **/ index: isize,
        /** The value. **/ value: StringSource,
    },
    /// [`BetterUrl::set_domain_suffix_segment`].
    SetDomainSuffixSegment {
        /** The index. **/ index: isize,
        /** The value. **/ value: StringSource,
    },
    /// [`BetterUrl::set_domain_origin_segment`].
    SetDomainOriginSegment {
        /** The index. **/ index: isize,
        /** The value. **/ value: StringSource,
    },



    /** [`BetterUrl::host_str`] + [`StringModification::apply`] + [`BetterUrl::set_host`].               **/ ModifyHost        (StringModification),
    /** [`BetterUrl::domain_prefix`] + [`StringModification::apply`] + [`BetterUrl::set_domain_prefix`]. **/ ModifyDomainPrefix(StringModification),
    /** [`BetterUrl::domain_middle`] + [`StringModification::apply`] + [`BetterUrl::set_domain_middle`]. **/ ModifyDomainMiddle(StringModification),
    /** [`BetterUrl::domain_suffix`] + [`StringModification::apply`] + [`BetterUrl::set_domain_suffix`]. **/ ModifyDomainSuffix(StringModification),
    /** [`BetterUrl::domain_origin`] + [`StringModification::apply`] + [`BetterUrl::set_domain_origin`]. **/ ModifyDomainOrigin(StringModification),

    /// [`BetterUrl::domain_segment`] + [`StringModification::apply`] + [`BetterUrl::set_domain_segment`].
    ModifyDomainSegment {
        /** The index.                  **/ index       : isize,
        /** The [`StringModification`]. **/ modification: StringModification,
    },
    /// [`BetterUrl::domain_prefix_segment`] + [`StringModification::apply`] + [`BetterUrl::set_domain_prefix_segment`].
    ModifyDomainPrefixSegment {
        /** The index.                  **/ index       : isize,
        /** The [`StringModification`]. **/ modification: StringModification,
    },
    /// [`BetterUrl::domain_suffix_segment`] + [`StringModification::apply`] + [`BetterUrl::set_domain_suffix_segment`].
    ModifyDomainSuffixSegment {
        /** The index.                  **/ index       : isize,
        /** The [`StringModification`]. **/ modification: StringModification,
    },
    /// [`BetterUrl::domain_origin_segment`] + [`StringModification::apply`] + [`BetterUrl::set_domain_origin_segment`].
    ModifyDomainOriginSegment {
        /** The index.                  **/ index       : isize,
        /** The [`StringModification`]. **/ modification: StringModification,
    },



    /// [`BetterUrl::insert_domain_segment`].
    InsertDomainSegment {
        /** The index. **/ index: isize,
        /** The value. **/ value: StringSource,
    },
    /// [`BetterUrl::insert_domain_prefix_segment`].
    InsertDomainPrefixSegment {
        /** The index. **/ index: isize,
        /** The value. **/ value: StringSource,
    },
    /// [`BetterUrl::insert_domain_suffix_segment`].
    InsertDomainSuffixSegment {
        /** The index. **/ index: isize,
        /** The value. **/ value: StringSource,
    },
    /// [`BetterUrl::insert_domain_origin_segment`].
    InsertDomainOriginSegment {
        /** The index. **/ index: isize,
        /** The value. **/ value: StringSource,
    },

    /** [`BetterUrl::set_fqdn`] to [`true`].  **/ EnsureFqdnPeriod,
    /** [`BetterUrl::set_fqdn`] to [`false`]. **/ RemoveFqdnPeriod,

    /// [`BetterUrl::set_path`].
    SetPath(StringSource),
    /// [`BetterUrl::path_str`] + [`StringModification::apply`] + [`BetterUrl::set_path`].
    ModifyPath(StringModification),
    /// [`BetterUrl::remove_path_segment`].
    RemovePathSegment(isize),
    /// [`BetterUrl::set_path_segment`].
    SetPathSegment {
        /** The index. **/ index: isize,
        /** The value. **/ value: StringSource,
    },
    /// [`BetterUrl::path_segment`] + [`PathSegment::lossy_decode`] + [`StringModification::apply`] + [`BetterUrl::set_path_segment`].
    ModifyPathSegment {
        /** The index.                  **/ index: isize,
        /** The [`StringModification`]. **/ modification: StringModification,
    },
    /// [`BetterUrl::insert_path_segment`].
    InsertPathSegment {
        /** The index. **/ index: isize,
        /** The value. **/ value: StringSource,
    },
    /// [`BetterUrl::pop_path`].
    PopPath,
    /// [`BetterUrl::pop_path_if_empty`].
    PopPathIfEmpty,



    /// [`BetterUrl::set_query`].
    SetQuery(StringSource),
    /// [`BetterUrl::set_query_param`].
    SetQueryParam {
        /** The [`QueryParamSelector`]. **/ param: QueryParamSelector,
        /** The [`StringSource`].       **/ value: StringSource,
    },
    /** [`BetterUrl::remove_query`].                                                             **/ RemoveQuery                                 ,
    /** [`BetterUrl::remove_empty_query`].                                                       **/ RemoveEmptyQuery                            ,
    /** [`MaybeQuery::filter`], removing params whose names are the specified value.             **/ RemoveQueryParam         (StringSource     ),
    /** [`MaybeQuery::filter`], removing params whose names are not the specified value.         **/ AllowQueryParam          (StringSource     ),
    /** [`MaybeQuery::filter`], removing params whose names are in the set.                      **/ RemoveQueryParams        (FxHashSet<String>),
    /** [`MaybeQuery::filter`], removing params whose names are not in the set.                  **/ AllowQueryParams         (FxHashSet<String>),
    /** [`MaybeQuery::filter`], removing params whose names satisfy the [`StringMatcher`].       **/ RemoveQueryParamsMatching(StringMatcher    ),
    /** [`MaybeQuery::filter`], removing params whose names don't satisfy the [`StringMatcher`]. **/ AllowQueryParamsMatching (StringMatcher    ),

    /// Set the URL to the value of the query param.
    /// # Errors
    /// If [`BetterUrl::query_param`] reutrns [`None`], returns the error [`QueryParamNotFound`].
    ///
    /// If [`QuerySegment::into_value`] reutrns [`None`], returns the error [`QueryParamNotFound`].
    GetUrlFromQueryParam(StringSource),

    // Fragment

    /// [`BetterUrl::set_fragment`].
    SetFragment(StringSource),
    /// [`BetterUrl::set_fragment_query_param`].
    SetFragmentParam {
        /** The fragment param to set. **/ param: QueryParamSelector,
        /** The value to set it to.    **/ value: StringSource,
    },
    /** [`BetterUrl::remove_fragment`].                                                          **/ RemoveFragment                                 ,
    /** [`BetterUrl::remove_empty_fragment`].                                                    **/ RemoveEmptyFragment                            ,
    /** [`MaybeQuery::filter`], removing params whose names are the specified value.             **/ RemoveFragmentParam         (StringSource     ),
    /** [`MaybeQuery::filter`], removing params whose names are not the specified value.         **/ AllowFragmentParam          (StringSource     ),
    /** [`MaybeQuery::filter`], removing params whose names are in the set.                      **/ RemoveFragmentParams        (FxHashSet<String>),
    /** [`MaybeQuery::filter`], removing params whose names are not in the set.                  **/ AllowFragmentParams         (FxHashSet<String>),
    /** [`MaybeQuery::filter`], removing params whose names satisfy the [`StringMatcher`].       **/ RemoveFragmentParamsMatching(StringMatcher    ),
    /** [`MaybeQuery::filter`], removing params whose names don't satisfy the [`StringMatcher`]. **/ AllowFragmentParamsMatching (StringMatcher    ),

    // Misc.

    /** See [`ThreadHider`] for info. **/ AcquireThreadLock,
    /** See [`ThreadHider`] for info. **/ ReleaseThreadLock,

    /// [`MaybeQuery::filter`] by set and prefix.
    ///
    /// If [`Self::HandleParams::query`], filters the query.
    ///
    /// If [`Self::HandleParams::fragment`], filters the fragment.
    ///
    /// A [`QuerySegment`] matches if its [`QuerySegment::name`]
    ///
    /// 1. It is in [`Self::HandleParams::names`] or it starts with a value in [`Self::HandleParams::prefixes`].
    ///
    /// 2. It is not in [`Self::HandleParams::except_names`] and it does not start with any value in [`Self::HandleParams::except_prefixes`].
    ///
    /// If [`Self::HandleParams::mode`] is [`HandleParamsMode::Keep`], matching params are kept.
    ///
    /// If [`Self::HandleParams::mode`] is [`HandleParamsMode::Remove`], matching params are moved.
    HandleParams {
        /// The [`HandleParamsMode`].
        ///
        /// Defaulted.
        #[serde(default, skip_serializing_if = "is_default")]
        mode: HandleParamsMode,
        /// If [`true`], handle query parameters.
        ///
        /// Defaults to [`true`].
        #[serde(default = "get_true", skip_serializing_if = "is_true")]
        query: bool,
        /// If [`true`], handle fragment parameters.
        ///
        /// Defaults to [`false`].
        #[serde(default, skip_serializing_if = "is_default")]
        fragment: bool,
        /// The names of segments to match.
        ///
        /// Defaulted.
        #[serde(default, skip_serializing_if = "is_default")]
        names: Box<SetSource>,
        /// The prefixes of segments to match.
        ///
        /// Defaulted.
        #[serde(default, skip_serializing_if = "is_default")]
        prefixes: Box<ListSource>,
        /// The names of segments to not match.
        ///
        /// Defaulted.
        #[serde(default, skip_serializing_if = "is_default")]
        except_names: Box<SetSource>,
        /// The prefixes of segments to not match.
        ///
        /// Defaulted.
        #[serde(default, skip_serializing_if = "is_default")]
        except_prefixes: Box<ListSource>,
    },



    /// Read and return or create a [`CacheClient`] entry.
    ///
    /// PLEASE note that before using this, [`Self::AcquireThreadLock`] should be used.
    #[cfg(feature = "cache")]
    Cache {
        /** The subject.  **/ subject: StringSource,
        /** The [`Self`]. **/ action : Box<Self>,
    },



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
    Extern(ActionExtern),
}

/// If [`HandleParamsMode`] should remove or keep matching parameters.
///
/// Defaults to [`Self::Remove`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub enum HandleParamsMode {
    /// Remove matching parameters.
    ///
    /// The default.
    #[default]
    Remove,
    /// Keep only matching parameters.
    Keep,
}

/// Helper function to get the default [`Action::Repeat::limit`].
const fn get_10_u64() -> u64 {10}

/// Generate the "modify {part}" [`Action`]s.
macro_rules! modify_part {
    ($ts:expr, $args:expr, $mod:expr, $get:ident, $set:ident$(, $arg:expr)*) => {{
        let mut value = $ts.url.$get($($arg),*).map(Cow::Borrowed);

        if $mod.apply($ts, $args, &mut value)? {
            $ts.url.$set($($arg,)* value.map(Cow::into_owned).as_deref())?
        } else {
            false
        }
    }};
}

impl Action {
    /// Applies the specified variant of [`Self`].
    ///
    /// Returns [`true`] if [`TaskState::url`] may have been changed and [`false`] if it definitely hasn't.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn apply<'j>(&'j self, ts: &mut TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<bool, ActionError> {
        Ok(match self {
            // Debug/constants

            Self::None => false,
            Self::Error(msg) => Err(ExplicitError(msg.clone()))?,

            // Error handling

            Self::IgnoreError(action) => action.apply(ts, args).unwrap_or(true),
            Self::IgnoreAndRevertOnError(action) => action.apply(ts, args).unwrap_or(false),
            Self::TryElse {r#try, r#else} => match r#try.apply(ts, args) {
                Ok(x) => x,
                Err(try_error) => match r#else.apply(ts, args) {
                    Ok(_) => true,
                    Err(else_error) => Err(TryElseError {try_error, else_error})?
                }
            },
            Self::FirstNotError(actions) => {
                let mut errors = Vec::new();

                for action in actions {
                    match action.apply(ts, args) {
                        Ok (x) => return Ok(x),
                        Err(e) => errors.push(e)
                    }
                }

                Err(FirstNotErrorErrors(errors))?
            },
            Self::RevertOnError(action) => {
                let old_url = ts.url.clone();

                match action.apply(ts, args) {
                    Ok (x) => x,
                    Err(e) => {
                        ts.url = old_url;
                        Err(e)?
                    }
                }
            },

            // Logic

            Self::If {r#if, then, r#else} => match r#if.check(ts, args)? {
                true  => then  .apply(ts, args)?,
                false => r#else.apply(ts, args)?,
            },
            Self::All(actions) => {
                let mut changed = false;

                for action in actions {
                    changed |= action.apply(ts, args)?;
                }

                changed
            },
            Self::Repeat{actions, limit} => {
                for i in 0..*limit {
                    let mut changed = false;

                    for action in actions {
                        changed |= action.apply(ts, args)?;
                    }

                    if !changed {
                        return Ok(i != 0);
                    }
                }

                *limit != 0
            },

            // Maps

            Self::PartMap   {part , map, r#else} => map.get(part .get(&ts.url ) .as_deref()).unwrap_or(r#else).apply(ts, args)?,
            Self::StringMap {value, map, r#else} => map.get(value.get(ts, args)?.as_deref()).unwrap_or(r#else).apply(ts, args)?,

            Self::PartPartitioning   {partitioning, part , map, r#else} => map.get(partitioning.get_some(ts, args)??.get(part .get(&ts.url ) .as_deref())).unwrap_or(r#else).apply(ts, args)?,
            Self::StringPartitioning {partitioning, value, map, r#else} => map.get(partitioning.get_some(ts, args)??.get(value.get(ts, args)?.as_deref())).unwrap_or(r#else).apply(ts, args)?,

            Self::FirstMappedPartPartitioning {partitioning, parts, map} => {
                let partitioning = partitioning.get_some(ts, args)??;
                for part in parts.iter() {
                    if let Some(action) = map.get(partitioning.get(part.get(&ts.url).as_deref())) {
                        return action.apply(ts, args);
                    }
                }
                false
            },
            Self::FirstMappedStringPartitioning {partitioning, values, map} => {
                let partitioning = partitioning.get_some(ts, args)??;
                for value in values.iter() {
                    if let Some(action) = map.get(partitioning.get(value.get(ts, args)?.as_deref())) {
                        return action.apply(ts, args);
                    }
                }
                false
            },

            // Whole

            Self::SetWhole(new) => {
                let new = new.get_some(ts, args)??;

                if ts.url == new {
                    return Ok(false);
                }

                let new = BetterUrl::new(new)?;

                if ts.url == new {
                    return Ok(false);
                }

                ts.url = new;

                true
            },

            Self::Join(with) => {ts.url.join(&*with.get_some_part(ts, args)??)?; true},

            // Scheme

            Self::SetScheme(to) => {ts.url.set_scheme(to.get_some_part(ts, args)??)?; true},

            // Domain

            Self::SetHost               (       value) => {ts.url.set_host         (value.get_some_part(ts, args)??)?; true},
            Self::SetDomainPrefix       (       value) =>  ts.url.set_domain_prefix(value.get_part     (ts, args)? )?,
            Self::SetDomainMiddle       (       value) =>  ts.url.set_domain_middle(value.get_part     (ts, args)? )?,
            Self::SetDomainSuffix       (       value) =>  ts.url.set_domain_suffix(value.get_part     (ts, args)? )?,
            Self::SetDomainOrigin       (       value) =>  ts.url.set_domain_origin(value.get_part     (ts, args)? )?,

            Self::SetDomainSegment      {index, value} =>  ts.url.set_domain_segment       (*index, value.get_part(ts, args)?)?,
            Self::SetDomainOriginSegment{index, value} =>  ts.url.set_domain_origin_segment(*index, value.get_part(ts, args)?)?,
            Self::SetDomainPrefixSegment{index, value} =>  ts.url.set_domain_prefix_segment(*index, value.get_part(ts, args)?)?,
            Self::SetDomainSuffixSegment{index, value} =>  ts.url.set_domain_suffix_segment(*index, value.get_part(ts, args)?)?,

            Self::ModifyHost               (       modification) => {
                let mut host = ts.url.host_str().map(Cow::from);

                if modification.apply(ts, args, &mut host)? {
                    ts.url.set_host(host.ok_or(StringNotFound)?.into_owned())?;
                    true
                } else {
                    false
                }
            },
            Self::ModifyDomainPrefix       (       modification) => modify_part!(ts, args, modification, domain_prefix_str        , set_domain_prefix),
            Self::ModifyDomainMiddle       (       modification) => modify_part!(ts, args, modification, domain_middle_str        , set_domain_middle),
            Self::ModifyDomainSuffix       (       modification) => modify_part!(ts, args, modification, domain_suffix_str        , set_domain_suffix),
            Self::ModifyDomainOrigin       (       modification) => modify_part!(ts, args, modification, domain_origin_str        , set_domain_origin),

            Self::ModifyDomainSegment      {index, modification} => modify_part!(ts, args, modification, domain_segment_str       , set_domain_segment       , *index),
            Self::ModifyDomainOriginSegment{index, modification} => modify_part!(ts, args, modification, domain_origin_segment_str, set_domain_origin_segment, *index),
            Self::ModifyDomainPrefixSegment{index, modification} => modify_part!(ts, args, modification, domain_prefix_segment_str, set_domain_prefix_segment, *index),
            Self::ModifyDomainSuffixSegment{index, modification} => modify_part!(ts, args, modification, domain_suffix_segment_str, set_domain_suffix_segment, *index),

            Self::InsertDomainSegment       {index, value} => {ts.url.insert_domain_segment       (*index, value.get_some_part(ts, args)??)?; true},
            Self::InsertDomainOriginSegment {index, value} => {ts.url.insert_domain_origin_segment(*index, value.get_some_part(ts, args)??)?; true},
            Self::InsertDomainPrefixSegment {index, value} => {ts.url.insert_domain_prefix_segment(*index, value.get_some_part(ts, args)??)?; true},
            Self::InsertDomainSuffixSegment {index, value} => {ts.url.insert_domain_suffix_segment(*index, value.get_some_part(ts, args)??)?; true},

            Self::EnsureFqdnPeriod => ts.url.set_fqdn(true )?,
            Self::RemoveFqdnPeriod => ts.url.set_fqdn(false)?,

            // Path

            Self::SetPath(to) => {ts.url.set_path(to.get_some_part(ts, args)??)?; true},
            Self::ModifyPath(modification) => {
                let mut path = Some(ts.url.path_str().into());

                if modification.apply(ts, args, &mut path)? {
                    ts.url.set_path(path.ok_or(StringNotFound)?.into_owned())?;
                    true
                } else {
                    false
                }
            },

            Self::RemovePathSegment(index) => ts.url.remove_path_segment(*index)?,
            Self::PopPath                  => ts.url.pop_path           (      )?,
            Self::PopPathIfEmpty           => ts.url.pop_path_if_empty  (      )?,

            Self::SetPathSegment {index, value} => ts.url.set_path_segment(*index, value.get_part(ts, args)?)?,
            Self::ModifyPathSegment {index, modification} => {
                let mut value = ts.url.path_segment(*index).map(PathSegment::lossy_decode);

                match modification.apply(ts, args, &mut value)? {
                    true  => ts.url.set_path_segment(*index, value.map(Cow::into_owned).as_deref())?,
                    false => false
                }
            },
            Self::InsertPathSegment {index, value} => ts.url.insert_path_segment(*index, value.get_some_part(ts, args)??)?,

            // Query

            Self::SetQuery                 (to)           => {ts.url.set_query(to.get_part(ts, args)?)?; true},
            Self::SetQueryParam            {param, value} => ts.url.set_query_param(&param.name, param.index, value.get_part(ts, args)?.map(Some))?,
            Self::RemoveQuery                             => ts.url.remove_query      (),
            Self::RemoveEmptyQuery                        => ts.url.remove_empty_query(),
            Self::RemoveQueryParam         (name   )      => {let name = name.get_some_part(ts, args)??; ts.url.filter_query(|s| s.name() != name)},
            Self::AllowQueryParam          (name   )      => {let name = name.get_some_part(ts, args)??; ts.url.filter_query(|s| s.name() == name)},
            Self::RemoveQueryParams        (names  )      => ts.url.filter_query(|s| !names.contains(&*s.into_name().lossy_decode())),
            Self::AllowQueryParams         (names  )      => ts.url.filter_query(|s|  names.contains(&*s.into_name().lossy_decode())),
            Self::AllowQueryParamsMatching (matcher)      => {
                match ts.url.query().try_filtered(|s| matcher.check(ts, args, Some(&s.into_name().lossy_decode())))? {
                    (true , query) => {ts.url.set_query(query.into_owned())?; true},
                    (false, _    ) => false
                }
            },
            Self::RemoveQueryParamsMatching (matcher) => {
                match ts.url.query().try_filtered(|s| matcher.check(ts, args, Some(&s.into_name().lossy_decode())).map(|x| !x))? {
                    (true , query) => {ts.url.set_query(query.into_owned())?; true},
                    (false, _    ) => false
                }
            },

            Self::GetUrlFromQueryParam(name) => {
                ts.url = ts.url.query_param(&name.get_some(ts, args)??, 0).and_then(|x| x.into_value().lossy_decode()).ok_or(QueryParamNotFound)?.parse()?;
                true
            },

            // Fragment

            Self::SetFragment                 (to)           => {ts.url.set_fragment(to.get_part(ts, args)?)?; true},
            Self::SetFragmentParam            {param, value} => ts.url.set_fragment_query_param(&param.name, param.index, value.get_part(ts, args)?.map(Some))?,
            Self::RemoveFragment                             => ts.url.remove_fragment      (),
            Self::RemoveEmptyFragment                        => ts.url.remove_empty_fragment(),
            Self::RemoveFragmentParam         (name   )      => {let name = name.get_some_part(ts, args)??; ts.url.filter_fragment_query(|s| s.into_name().lossy_decode() != name)},
            Self::AllowFragmentParam          (name   )      => {let name = name.get_some_part(ts, args)??; ts.url.filter_fragment_query(|s| s.into_name().lossy_decode() == name)},
            Self::RemoveFragmentParams        (names  )      => ts.url.filter_fragment_query(|s| !names.contains(&*s.into_name().lossy_decode())),
            Self::AllowFragmentParams         (names  )      => ts.url.filter_fragment_query(|s|  names.contains(&*s.into_name().lossy_decode())),
            Self::AllowFragmentParamsMatching (matcher)      => {
                match ts.url.fragment_query().try_filtered(|s| matcher.check(ts, args, Some(&s.into_name().lossy_decode())))? {
                    (true , fragment) => {ts.url.set_fragment(fragment.into_owned())?; true},
                    (false, _       ) => false
                }
            },
            Self::RemoveFragmentParamsMatching (matcher) => {
                match ts.url.fragment_query().try_filtered(|s| matcher.check(ts, args, Some(&s.into_name().lossy_decode())).map(|x| !x))? {
                    (true , fragment) => {ts.url.set_fragment(fragment.into_owned())?; true},
                    (false, _       ) => false
                }
            },

            // Misc.

            Self::HandleParams {mode, query, fragment, names, prefixes, except_names, except_prefixes} => {
                if !(*fragment && ts.url.fragment_str().is_some() || *query && ts.url.query_str().is_some()) {
                    return Ok(false);
                }

                let mn = names          .get(ts, args)?;
                let mp = prefixes       .get(ts, args)?;
                let en = except_names   .get(ts, args)?;
                let ep = except_prefixes.get(ts, args)?;

                let filter = |segment: QueryLikeSegment<'_>| -> bool {
                    let name = segment.into_name().lossy_decode();

                    let mn = || {mn.is_some_and(|x| x.contains_some(&*name))};
                    let mp = || {mp.is_some_and(|x| x.iter().any(|prefix| name.starts_with(prefix)))};
                    let en = || {en.is_some_and(|x| x.contains_some(&*name))};
                    let ep = || {ep.is_some_and(|x| x.iter().any(|prefix| name.starts_with(prefix)))};

                    let matches = (mn() || mp()) && !(en() || ep());

                    matches!((mode, matches), (HandleParamsMode::Keep, true) | (HandleParamsMode::Remove, false))
                };

                let mut changed = false;

                if *fragment {
                    changed |= ts.url.filter_fragment_query(|x| filter(x.into()));
                }

                if *query {
                    changed |= ts.url.filter_query(|x| filter(x.into()));
                }

                changed
            },

            #[cfg(feature = "cache")]
            Self::Cache {subject, action} => {
                #[cold]
                fn outline<'j: 't, 't>(ts: &'t mut TaskState<'j>, args: Option<&'j FunctionArgs>, subject: &'j StringSource, action: &'j Action) -> Result<bool, ActionError> {
                    let subject = subject.get_some_part(ts, args)??;
                    let key = ts.url.as_str();

                    if let Some(value) = ts.job.cache_client.read_sync(&subject, key, ts.job.cache_config)? {
                        ts.url = BetterUrl::new(value.ok_or(StringNotFound)?)?;
                        return Ok(true);
                    }

                    let key = key.to_string();

                    let start = std::time::Instant::now();
                    let ret = action.apply(ts, args)?;
                    let duration = start.elapsed();

                    ts.job.cache_client.write_sync(&subject, &key, Some(ts.url.as_str()), duration, ts.job.cache_config)?;

                    Ok(ret)
                }

                outline(ts, args, subject, action)?
            },

            Self::AcquireThreadLock => {ts.thread_lock = ts.job.thread_hider.as_deref().map(ThreadHider::lock); false},
            Self::ReleaseThreadLock => {ts.thread_lock = None                                                         ; false},

            Self::Function   (call    ) => ts.job.cleaner.functions  .actions.get(&call.name                 ).ok_or(FunctionNotFound           )?.apply(ts, Some(&call.args))?,
            Self::FunctionArg(name    ) => args.ok_or(NotInFunction)?.actions.get(&*name.get_some(ts, args)??).ok_or(FunctionArgFunctionNotFound)?.apply(ts, args            )?,
            Self::Extern     (function) => function(ts, args)?,
        })
    }
}
