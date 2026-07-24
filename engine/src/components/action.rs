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
        /// Defaults to [`Self::None`].
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Box<Self>
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
        map: Box<Map<Self>>
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
        map: Box<Map<Self>>
    },



    /// Repeat the contained [`Self`]s until all calls to [`Self::apply`] return [`false`] or the limit is reached.
    Repeat {
        /// The [`Self`]s to repeat.
        actions: Vec<Action>,
        /// The maximum amount of times to repeat.
        ///
        /// Defaults to 10.
        #[serde(default = "get_10_u64")]
        limit: u64
    },



    /// Map [`Err`] to [`false`].
    ///
    /// To revert unfinished changes, pair this with [`Self::RevertOnError`].
    IgnoreError(Box<Self>),
    /// [`Self::IgnoreError`] + [`Self::RevertOnError`].
    IgnoreAndRevertOnError(Box<Self>),
    /// Reverts the inner [`Self`] if it returns [`Err`], then returns the error.
    ///
    /// To ignore errors, pair this with [`Self::IgnoreError`].
    RevertOnError(Box<Self>),
    /// [`Self::TryElse::try`], or [`Self::TryElse::else`] if it returns [`Err`].
    TryElse {
        /// The try.
        r#try: Box<Self>,
        /// The else.
        r#else: Box<Self>
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

    /// [`BetterUrl::set_host`].
    SetHost(StringSource),
    /// [`BetterUrl::set_domain_prefix`].
    SetDomainPrefix(StringSource),
    /// [`BetterUrl::set_domain_middle`].
    SetDomainMiddle(StringSource),
    /// [`BetterUrl::set_domain_suffix`].
    SetDomainSuffix(StringSource),
    /// [`BetterUrl::set_domain_origin`].
    SetDomainOrigin(StringSource),

    /// [`BetterUrl::set_domain_segment`].
    SetDomainSegment {
        /// The index.
        index: isize,
        /// The value.
        value: StringSource
    },
    /// [`BetterUrl::set_domain_prefix_segment`].
    SetDomainPrefixSegment {
        /// The index.
        index: isize,
        /// The value.
        value: StringSource
    },
    /// [`BetterUrl::set_domain_suffix_segment`].
    SetDomainSuffixSegment {
        /// The index.
        index: isize,
        /// The value.
        value: StringSource
    },
    /// [`BetterUrl::set_domain_origin_segment`].
    SetDomainOriginSegment {
        /// The index.
        index: isize,
        /// The value.
        value: StringSource
    },



    /// [`BetterUrl::host_str`] = [`StringModification::apply`] + [`BetterUrl::set_host`].
    ModifyHost(StringModification),
    /// [`BetterUrl::domain_prefix`] = [`StringModification::apply`] + [`BetterUrl::set_domain_prefix`].
    ModifyDomainPrefix(StringModification),
    /// [`BetterUrl::domain_middle`] = [`StringModification::apply`] + [`BetterUrl::set_domain_middle`].
    ModifyDomainMiddle(StringModification),
    /// [`BetterUrl::domain_suffix`] = [`StringModification::apply`] + [`BetterUrl::set_domain_suffix`].
    ModifyDomainSuffix(StringModification),
    /// [`BetterUrl::domain_origin`] = [`StringModification::apply`] + [`BetterUrl::set_domain_origin`].
    ModifyDomainOrigin(StringModification),

    /// [`BetterUrl::domain_segment`] = [`StringModification::apply`] + [`BetterUrl::set_domain_segment`].
    ModifyDomainSegment {
        /// The index.
        index: isize,
        /// The [`StringModification`].
        modification: StringModification
    },
    /// [`BetterUrl::domain_prefix_segment`] = [`StringModification::apply`] + [`BetterUrl::set_domain_prefix_segment`].
    ModifyDomainPrefixSegment {
        /// The index.
        index: isize,
        /// The [`StringModification`].
        modification: StringModification
    },
    /// [`BetterUrl::domain_suffix_segment`] = [`StringModification::apply`] + [`BetterUrl::set_domain_suffix_segment`].
    ModifyDomainSuffixSegment {
        /// The index.
        index: isize,
        /// The [`StringModification`].
        modification: StringModification
    },
    /// [`BetterUrl::domain_origin_segment`] = [`StringModification::apply`] + [`BetterUrl::set_domain_origin_segment`].
    ModifyDomainOriginSegment {
        /// The index.
        index: isize,
        /// The [`StringModification`].
        modification: StringModification
    },



    /// [`BetterUrl::insert_domain_segment`].
    InsertDomainSegment {
        /// The index.
        index: isize,
        /// The value.
        value: StringSource
    },
    /// [`BetterUrl::insert_domain_prefix_segment`].
    InsertDomainPrefixSegment {
        /// The index.
        index: isize,
        /// The value.
        value: StringSource
    },
    /// [`BetterUrl::insert_domain_suffix_segment`].
    InsertDomainSuffixSegment {
        /// The index.
        index: isize,
        /// The value.
        value: StringSource
    },
    /// [`BetterUrl::insert_domain_origin_segment`].
    InsertDomainOriginSegment {
        /// The index.
        index: isize,
        /// The value.
        value: StringSource
    },

    /// [`BetterUrl::set_fqdn`] to [`true`].
    EnsureFqdnPeriod,
    /// [`BetterUrl::set_fqdn`] to [`false`].
    RemoveFqdnPeriod,

    /// [`BetterUrl::set_path`].
    SetPath(StringSource),
    /// [`BetterUrl::path_str`] + [`StringModification::apply`] + [`BetterUrl::set_path`].
    ModifyPath(StringModification),
    /// [`BetterUrl::remove_path_segment`].
    RemovePathSegment(isize),
    /// [`BetterUrl::set_path_segment`].
    SetPathSegment {
        /// The index.
        index: isize,
        /// The value.
        value: StringSource
    },
    /// [`BetterUrl::path_segment`] + [`PathSegment::decode`] + [`StringModification::apply`] + [`BetterUrl::set_path_segment`].
    ModifyPathSegment {
        /// The index.
        index: isize,
        /// The [`StringModification`].
        modification: StringModification
    },
    /// [`BetterUrl::insert_path_segment`].
    InsertPathSegment {
        /// The index.
        index: isize,
        /// The value.
        value: StringSource
    },
    /// [`BetterUrl::pop_path`].
    ///
    /// Always returns either [`true`] or [`Err`].
    PopPath,
    /// [`BetterUrl::pop_path_if_empty`].
    PopPathIfEmpty,



    /// [`BetterUrl::set_query`].
    SetQuery(StringSource),
    /// [`BetterUrl::set_query_param`].
    SetQueryParam {
        /// The [`QueryParamSelector`].
        param: QueryParamSelector,
        /// The [`StringSource`].
        value: StringSource
    },
    /// [`BetterUrl::remove_query`].
    RemoveQuery,
    /// [`BetterUrl::remove_empty_query`].
    RemoveEmptyQuery,
    /// [`MaybeQuery::filter`], keeping only params whose names are not the specified value.
    RemoveQueryParam(StringSource),
    /// [`MaybeQuery::filter`], keeping only params whose names are the specified value.
    AllowQueryParam(StringSource),
    /// [`MaybeQuery::filter`], keeping only params whose names are not in the set.
    RemoveQueryParams(HashSet<String>),
    /// [`MaybeQuery::filter`], keeping only params whose names are in the set.
    AllowQueryParams(HashSet<String>),
    /// [`MaybeQuery::filter`], keeping only params whose names satisfy the [`StringMatcher`].
    RemoveQueryParamsMatching(StringMatcher),
    /// [`MaybeQuery::filter`], keeping only params whose names don't satisfy the [`StringMatcher`].
    AllowQueryParamsMatching(StringMatcher),

    /// Set the URL to the value of the query param.
    /// # Errors
    /// If the call to [`BetterUrl::query_param`] reutrns [`None`], returns the error [`QueryParamNotFound`].
    ///
    /// If the call to [`QuerySegment::into_value`] reutrns [`None`], returns the error [`QueryParamNotFound`].
    GetUrlFromQueryParam(StringSource),

    // Fragment

    /// [`BetterUrl::set_fragment`].
    SetFragment(StringSource),
    /// [`BetterUrl::set_fragment_query_param`].
    SetFragmentParam {
        /// The fragment param to set.
        param: QueryParamSelector,
        /// The value to set it to.
        value: StringSource
    },
    /// [`BetterUrl::remove_fragment`].
    RemoveFragment,
    /// [`BetterUrl::remove_empty_fragment`].
    RemoveEmptyFragment,
    /// [`MaybeQuery::filter`], keeping only params whose names are not the specified value.
    RemoveFragmentParam(StringSource),
    /// [`MaybeQuery::filter`], keeping only params whose names are the specified value.
    AllowFragmentParam(StringSource),
    /// [`MaybeQuery::filter`], keeping only params whose names are not in the set.
    RemoveFragmentParams(HashSet<String>),
    /// [`MaybeQuery::filter`], keeping only params whose names are in the set.
    AllowFragmentParams(HashSet<String>),
    /// [`MaybeQuery::filter`], keeping only params whose names satisfy the [`StringMatcher`].
    RemoveFragmentParamsMatching(StringMatcher),
    /// [`MaybeQuery::filter`], keeping only params whose names don't satisfy the [`StringMatcher`].
    AllowFragmentParamsMatching(StringMatcher),

    // Misc.

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
        /// Defaults to [`HandleParamsMode::Remove`].
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
        /// Defaults to [`SetSource::None`].
        #[serde(default, skip_serializing_if = "is_default")]
        names: SetSource,
        /// The prefixes of segments to match.
        ///
        /// Defaults to [`ListSource::None`].
        #[serde(default, skip_serializing_if = "is_default")]
        prefixes: ListSource,
        /// The names of segments to not match.
        ///
        /// Defaults to [`SetSource::None`].
        #[serde(default, skip_serializing_if = "is_default")]
        except_names: SetSource,
        /// The prefixes of segments to not match.
        ///
        /// Defaults to [`ListSource::None`].
        #[serde(default, skip_serializing_if = "is_default")]
        except_prefixes: ListSource
    },



    /// Get the [`Cache`] entry with a subject of [`Self::Cache::subject`] and a key of the current URL.
    ///
    /// - If an entry is found, set the URL to its value.
    ///
    /// - If no entry is found, apply [`Self::Cache::action`] and make the entry.
    #[cfg(feature = "cache")]
    Cache {
        /// The subject.
        subject: StringSource,
        /// The [`Self`].
        action: Box<Self>
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
    Extern(ActionExtern)
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
    Keep
}

/// Helper function to get the default [`Action::Repeat::limit`].
const fn get_10_u64() -> u64 {10}

/// Generate the "modify {part}" [`Action`]s.
macro_rules! modify_part {
    ($ts:expr, $args:expr, $mod:expr, $get:ident, $set:ident$(, $arg:expr)*) => {{
        let mut x = $ts.url.$get($($arg),*).map(Cow::Borrowed);
        match $mod.apply($ts, $args, &mut x)? {
            true  => $ts.url.$set($($arg,)* x.map(Cow::into_owned).as_deref())?,
            false => false
        }
    }};
}

impl Action {
    /// Applies the specified variant of [`Self`].
    ///
    /// Returns [`true`] if [`TaskState::url`] may have been changed and [`false`] if it definitely hasn't.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn apply<'j>(&'j self, task_state: &mut TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<bool, ActionError> {
        debug!(Acton::apply, self, task_state.url, args; self._apply(task_state, args))
    }

    /// [`Self::apply`].
    fn _apply<'j>(&'j self, task_state: &mut TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<bool, ActionError> {
        Ok(match self {
            // Debug/constants

            Self::None => false,
            Self::Error(msg) => Err(ExplicitError(msg.clone()))?,

            // Error handling

            Self::IgnoreError(action) => action.apply(task_state, args).unwrap_or(true),
            Self::IgnoreAndRevertOnError(action) => action.apply(task_state, args).unwrap_or(false),
            Self::TryElse {r#try, r#else} => match r#try.apply(task_state, args) {
                Ok(x) => x,
                Err(try_error) => match r#else.apply(task_state, args) {
                    Ok(_) => true,
                    Err(else_error) => Err(TryElseError {try_error, else_error})?
                }
            },
            Self::FirstNotError(actions) => {
                let mut errors = Vec::new();

                for action in actions {
                    match action.apply(task_state, args) {
                        Ok (x) => return Ok(x),
                        Err(e) => errors.push(e)
                    }
                }

                Err(FirstNotErrorErrors(errors))?
            },
            Self::RevertOnError(action) => {
                let old_url = task_state.url.clone();

                match action.apply(task_state, args) {
                    Ok (x) => x,
                    Err(e) => {
                        task_state.url = old_url;
                        Err(e)?
                    }
                }
            },

            // Logic

            Self::If {r#if, then, r#else} => match r#if.check(task_state, args)? {
                true  => then  .apply(task_state, args)?,
                false => r#else.apply(task_state, args)?,
            },
            Self::All(actions) => {
                let mut changed = false;

                for action in actions {
                    changed |= action.apply(task_state, args)?;
                }

                changed
            },
            Self::Repeat{actions, limit} => {
                for i in 0..*limit {
                    let mut changed = false;

                    for action in actions {
                        changed |= action.apply(task_state, args)?;
                    }

                    if !changed {
                        return Ok(i != 0);
                    }
                }

                *limit != 0
            },

            // Maps

            Self::PartMap   {part , map, r#else} => map.get(part.get(&task_state.url).as_deref()).unwrap_or(r#else).apply(task_state, args)?,
            Self::StringMap {value, map, r#else} => map.get(get!(?&value)                       ).unwrap_or(r#else).apply(task_state, args)?,

            Self::PartPartitioning   {partitioning, part , map, r#else} => map.get(get!(partitioning).get(part.get(&task_state.url).as_deref())).unwrap_or(r#else).apply(task_state, args)?,
            Self::StringPartitioning {partitioning, value, map, r#else} => map.get(get!(partitioning).get(get!(?&value)                       )).unwrap_or(r#else).apply(task_state, args)?,

            Self::FirstMappedPartPartitioning {partitioning, parts, map} => {
                let partitioning = get!(partitioning);
                for part in parts.iter() {
                    if let Some(action) = map.get(partitioning.get(part.get(&task_state.url).as_deref())) {
                        return action.apply(task_state, args);
                    }
                }
                false
            },
            Self::FirstMappedStringPartitioning {partitioning, values, map} => {
                let partitioning = get!(partitioning);
                for value in values.iter() {
                    if let Some(action) = map.get(partitioning.get(get!(?&value))) {
                        return action.apply(task_state, args);
                    }
                }
                false
            },

            // Whole

            Self::SetWhole(new) => {
                let new = get!(new);

                if task_state.url == new {
                    return Ok(false);
                }

                let new = BetterUrl::new(new)?;

                if task_state.url == new {
                    return Ok(false);
                }

                task_state.url = new;

                true
            },

            Self::Join(with) => {
                task_state.url.join(get!(&!with))?;
                true
            },

            // Scheme

            Self::SetScheme(to) => {task_state.url.set_scheme(get!(!to))?; true},

            // Domain

            Self::SetHost               (       value) => {task_state.url.set_host         (get!( &!value))?; true},
            Self::SetDomainPrefix       (       value) =>  task_state.url.set_domain_prefix(get!(?&!value))?,
            Self::SetDomainMiddle       (       value) =>  task_state.url.set_domain_middle(get!(?&!value))?,
            Self::SetDomainSuffix       (       value) =>  task_state.url.set_domain_suffix(get!(?&!value))?,
            Self::SetDomainOrigin       (       value) =>  task_state.url.set_domain_origin(get!(?&!value))?,

            Self::SetDomainSegment      {index, value} =>  task_state.url.set_domain_segment       (*index, get!(?&!value))?,
            Self::SetDomainOriginSegment{index, value} =>  task_state.url.set_domain_origin_segment(*index, get!(?&!value))?,
            Self::SetDomainPrefixSegment{index, value} =>  task_state.url.set_domain_prefix_segment(*index, get!(?&!value))?,
            Self::SetDomainSuffixSegment{index, value} =>  task_state.url.set_domain_suffix_segment(*index, get!(?&!value))?,

            Self::ModifyHost               (       modification) => {
                let mut host = task_state.url.host_str().map(Cow::from);

                if modification.apply(task_state, args, &mut host)? {
                    task_state.url.set_host(host.ok_or(StringNotFound)?.into_owned())?;
                    true
                } else {
                    false
                }
            },
            Self::ModifyDomainPrefix       (       modification) => modify_part!(task_state, args, modification, domain_prefix_str        , set_domain_prefix),
            Self::ModifyDomainMiddle       (       modification) => modify_part!(task_state, args, modification, domain_middle_str        , set_domain_middle),
            Self::ModifyDomainSuffix       (       modification) => modify_part!(task_state, args, modification, domain_suffix_str        , set_domain_suffix),
            Self::ModifyDomainOrigin       (       modification) => modify_part!(task_state, args, modification, domain_origin_str        , set_domain_origin),

            Self::ModifyDomainSegment      {index, modification} => modify_part!(task_state, args, modification, domain_segment_str       , set_domain_segment       , *index),
            Self::ModifyDomainOriginSegment{index, modification} => modify_part!(task_state, args, modification, domain_origin_segment_str, set_domain_origin_segment, *index),
            Self::ModifyDomainPrefixSegment{index, modification} => modify_part!(task_state, args, modification, domain_prefix_segment_str, set_domain_prefix_segment, *index),
            Self::ModifyDomainSuffixSegment{index, modification} => modify_part!(task_state, args, modification, domain_suffix_segment_str, set_domain_suffix_segment, *index),

            Self::InsertDomainSegment       {index, value} => {task_state.url.insert_domain_segment       (*index, get!(&!value))?; true},
            Self::InsertDomainOriginSegment {index, value} => {task_state.url.insert_domain_origin_segment(*index, get!(&!value))?; true},
            Self::InsertDomainPrefixSegment {index, value} => {task_state.url.insert_domain_prefix_segment(*index, get!(&!value))?; true},
            Self::InsertDomainSuffixSegment {index, value} => {task_state.url.insert_domain_suffix_segment(*index, get!(&!value))?; true},

            Self::EnsureFqdnPeriod => task_state.url.set_fqdn(true )?,
            Self::RemoveFqdnPeriod => task_state.url.set_fqdn(false)?,

            // Path

            Self::SetPath(to) => {task_state.url.set_path(get!(!to))?; true},
            Self::ModifyPath(modification) => {
                let mut path = Some(task_state.url.path_str().into());

                if modification.apply(task_state, args, &mut path)? {
                    task_state.url.set_path(path.ok_or(StringNotFound)?.into_owned())?;
                    true
                } else {
                    false
                }
            },

            Self::RemovePathSegment(index) => {task_state.url.remove_path_segment(*index)?; true},
            Self::PopPath                  => {task_state.url.pop_path           (      )?; true},
            Self::PopPathIfEmpty           =>  task_state.url.pop_path_if_empty  (      )?       ,

            Self::SetPathSegment {index, value} => task_state.url.set_path_segment(*index, get!(?&!value))?,
            Self::ModifyPathSegment {index, modification} => {
                let mut value = task_state.url.path_segment(*index).map(PathSegment::decode);

                match modification.apply(task_state, args, &mut value)? {
                    true  => task_state.url.set_path_segment(*index, value.map(Cow::into_owned).as_deref())?,
                    false => false
                }
            },
            Self::InsertPathSegment {index, value} => task_state.url.insert_path_segment(*index, get!(!value))?,

            // Query

            Self::SetQuery                 (to)           => {task_state.url.set_query(get!(?!to))?; true},
            Self::SetQueryParam            {param, value} => task_state.url.set_query_param(&param.name, param.index, get!(?&!value).map(Some))?,
            Self::RemoveQuery                             => task_state.url.remove_query      (),
            Self::RemoveEmptyQuery                        => task_state.url.remove_empty_query(),
            Self::RemoveQueryParam         (name   )      => {let name = get!(!name); task_state.url.filter_query(|s| s.name() != name)},
            Self::AllowQueryParam          (name   )      => {let name = get!(!name); task_state.url.filter_query(|s| s.name() == name)},
            Self::RemoveQueryParams        (names  )      => task_state.url.filter_query(|s| !names.contains(&*s.name())),
            Self::AllowQueryParams         (names  )      => task_state.url.filter_query(|s|  names.contains(&*s.name())),
            Self::AllowQueryParamsMatching (matcher)      => {
                match task_state.url.query().try_filtered(|s| matcher.check(task_state, args, Some(&s.name())))? {
                    (true , query) => {task_state.url.set_query(query.into_owned())?; true},
                    (false, _    ) => false
                }
            },
            Self::RemoveQueryParamsMatching (matcher) => {
                match task_state.url.query().try_filtered(|s| matcher.check(task_state, args, Some(&s.name())).map(|x| !x))? {
                    (true , query) => {task_state.url.set_query(query.into_owned())?; true},
                    (false, _    ) => false
                }
            },

            Self::GetUrlFromQueryParam(name) => {
                task_state.url = task_state.url.query_param(get!(&name), 0).and_then(QuerySegment::into_value).ok_or(QueryParamNotFound)?.parse()?;
                true
            },

            // Fragment

            Self::SetFragment                 (to)           => {task_state.url.set_fragment(get!(?!to))?; true},
            Self::SetFragmentParam            {param, value} => task_state.url.set_fragment_query_param(&param.name, param.index, get!(?&!value).map(Some))?,
            Self::RemoveFragment                             => task_state.url.remove_fragment      (),
            Self::RemoveEmptyFragment                        => task_state.url.remove_empty_fragment(),
            Self::RemoveFragmentParam         (name   )      => {let name = get!(!name); task_state.url.filter_fragment_query(|s| s.name() != name)},
            Self::AllowFragmentParam          (name   )      => {let name = get!(!name); task_state.url.filter_fragment_query(|s| s.name() == name)},
            Self::RemoveFragmentParams        (names  )      => task_state.url.filter_fragment_query(|s| !names.contains(&*s.name())),
            Self::AllowFragmentParams         (names  )      => task_state.url.filter_fragment_query(|s|  names.contains(&*s.name())),
            Self::AllowFragmentParamsMatching (matcher)      => {
                match task_state.url.fragment_query().try_filtered(|s| matcher.check(task_state, args, Some(&s.name())))? {
                    (true , fragment) => {task_state.url.set_fragment(fragment.into_owned())?; true},
                    (false, _       ) => false
                }
            },
            Self::RemoveFragmentParamsMatching (matcher) => {
                match task_state.url.fragment_query().try_filtered(|s| matcher.check(task_state, args, Some(&s.name())).map(|x| !x))? {
                    (true , fragment) => {task_state.url.set_fragment(fragment.into_owned())?; true},
                    (false, _       ) => false
                }
            },

            // Misc.

            Self::HandleParams {mode, query, fragment, names, prefixes, except_names, except_prefixes} => {
                if !(*fragment && task_state.url.fragment().is_some() || *query && task_state.url.query().is_some()) {
                    return Ok(false);
                }

                let ds = Default::default();
                let dl = Default::default();

                let names           = get!(?names          ).unwrap_or(&ds);
                let prefixes        = get!(?prefixes       ).unwrap_or(&dl);
                let except_names    = get!(?except_names   ).unwrap_or(&ds);
                let except_prefixes = get!(?except_prefixes).unwrap_or(&dl);

                let excepts = !except_names.is_empty() || !except_prefixes.is_empty();

                let filter = |segment: QueryLikeSegment<'_>| -> bool {
                    let name = segment.into_name();

                    let matches = (names.contains_some(&*name) || prefixes.iter().any(|prefix| name.starts_with(prefix)))
                        && !(excepts && (except_names.contains_some(&*name) || except_prefixes.iter().any(|prefix| name.starts_with(prefix))));

                    matches!((mode, matches), (HandleParamsMode::Keep, true) | (HandleParamsMode::Remove, false))
                };

                let mut changed = false;

                if *fragment {
                    changed |= task_state.url.filter_fragment_query(|x| filter(x.into()));
                }

                if *query {
                    changed |= task_state.url.filter_query(|x| filter(x.into()));
                }

                changed
            },



            #[cfg(feature = "cache")]
            Self::Cache {subject, action} => {
                let _unthread_handle = task_state.job.unthreader.unthread();
                let subject = get!(&!subject);

                if let Some(entry) = task_state.job.cache.read(CacheEntryKeys {subject, key: task_state.url.as_str()})? {
                    task_state.url = BetterUrl::new(entry.value.ok_or(StringNotFound)?)?;
                    return Ok(true);
                }

                let key = &task_state.url.to_string();
                let start = std::time::Instant::now();

                let ret = action.apply(task_state, args)?;

                task_state.job.cache.write(NewCacheEntry {
                    subject,
                    key,
                    value: Some(task_state.url.as_str()),
                    duration: start.elapsed()
                })?;

                ret
            },

            // Misc

            Self::Function   (call    ) => task_state.job.cleaner.functions.actions.get(&call.name ).ok_or(FunctionNotFound           )?.apply(task_state, Some(&call.args))?,
            Self::FunctionArg(name    ) => args.ok_or(NotInFunction)?      .actions.get(get!(&name)).ok_or(FunctionArgFunctionNotFound)?.apply(task_state, args            )?,
            Self::Extern     (function) => function(task_state, args)?
        })
    }
}
