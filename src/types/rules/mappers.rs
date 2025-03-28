//! Logic for how a [`TaskState`] should be modified.

use std::str::Utf8Error;
use std::collections::HashSet;
use std::time::Duration;
use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use serde_with::{serde_as, SetPreventDuplicates};
use thiserror::Error;
use url::Url;
#[cfg(feature = "http")]
use reqwest::header::HeaderMap;

use crate::glue::*;
use crate::types::*;
use crate::util::*;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Suitability)]
pub enum Mapper {

    None,
    Error,
    #[suitable(never)]
    Debug(Box<Self>),

    If {
        condition: Condition,
        mapper: Box<Self>,
        #[serde(default)]
        else_mapper: Option<Box<Self>>
    },
    ConditionChain(Vec<ConditionChainLink>),
    All(Vec<Self>),
    AllIgnoreError(Vec<Self>),
    PartMap {
        part: UrlPart,
        #[serde(flatten)]
        map: Map<Self>
    },
    StringMap {
        value: StringSource,
        #[serde(flatten)]
        map: Map<Self>
    },

    IgnoreError(Box<Self>),
    TryElse {
        r#try: Box<Self>,
        r#else: Box<Self>
    },
    FirstNotError(Vec<Self>),
    RevertOnError(Box<Self>),

    RemoveQuery,
    RemoveQueryParam         (StringSource),
    RemoveQueryParams        (#[serde_as(as = "SetPreventDuplicates<_>")] HashSet<String>),
    AllowQueryParams         (#[serde_as(as = "SetPreventDuplicates<_>")] HashSet<String>),
    RemoveQueryParamsMatching(StringMatcher),
    AllowQueryParamsMatching (StringMatcher),
    GetUrlFromQueryParam     (StringSource),
    GetPathFromQueryParam    (StringSource),


    SetHost(String),
    Join(StringSource),

    SetPart {
        part: UrlPart,
        value: StringSource
    },
    ModifyPart {
        part: UrlPart,
        modification: StringModification
    },
    CopyPart {
        from: UrlPart,
        to: UrlPart
    },
    MovePart {
        from: UrlPart,
        to: UrlPart
    },

    // Miscellaneous.
    #[cfg_attr(feature = "cache", doc = "If the call to [`Cache::read`] returns an error, that error is returned.")]
    #[cfg_attr(feature = "cache", doc = "If the call to [`Cache::write`] returns an error, that error is returned.")]
    #[cfg(feature = "http")]
    ExpandRedirect {
        #[serde(default, with = "serde_headermap")]
        headers: HeaderMap,
        #[serde(default)]
        http_client_config_diff: Option<Box<HttpClientConfigDiff>>
    },
    SetScratchpadFlag {
        name: StringSource,
        value: bool
    },
    SetScratchpadVar {
        name: StringSource,
        value: StringSource
    },
    DeleteScratchpadVar(StringSource),
    ModifyScratchpadVar {
        name: StringSource,
        modification: StringModification
    },
    Rule(Box<Rule>),
    Rules(Rules),
    #[cfg(feature = "cache")]
    CacheUrl {
        category: StringSource,
        mapper: Box<Self>
    },
    Retry {
        mapper: Box<Self>,
        delay: Duration,
        #[serde(default = "get_10_u8")]
        limit: u8
    },
    Common(CommonCall),
    #[expect(clippy::type_complexity, reason = "Who cares")]
    #[cfg(feature = "custom")]
    #[suitable(never)]
    Custom(FnWrapper<fn(&mut TaskState) -> Result<(), MapperError>>)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub struct ConditionChainLink {
    pub condition: Condition,
    pub mapper: Mapper
}

/// Serde helper function.
const fn get_10_u8() -> u8 {10}

#[derive(Debug, Error)]
pub enum MapperError {
    #[error("Mapper::Error was used.")]
    ExplicitError,
    #[error("The provided URL does not contain the requested query parameter.")]
    CannotFindQueryParam,
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    #[cfg(feature = "http")]
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    Utf8Error(#[from] Utf8Error),
    #[error(transparent)]
    UrlPartSetError(#[from] UrlPartSetError),
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,
    #[error(transparent)]
    StringMatcherError(#[from] StringMatcherError),
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
    #[error(transparent)]
    StringModificationError(#[from] StringModificationError),
    #[error(transparent)]
    ConditionError(#[from] ConditionError),
    #[error(transparent)]
    GetConfigError(#[from] GetConfigError),
    #[error(transparent)]
    RuleError(Box<RuleError>),
    #[cfg(feature = "http")]
    #[error("The requested header was not found.")]
    HeaderNotFound,
    #[cfg(feature = "http")]
    #[error(transparent)]
    ToStrError(#[from] reqwest::header::ToStrError),
    #[error("A `Mapper::TryElse` had both `try` and `else` return an error.")]
    TryElseError {
        try_error: Box<Self>,
        else_error: Box<Self>
    },
    #[error("A TaskState string var was none.")]
    ScratchpadVarIsNone,
    #[cfg(feature = "cache")]
    #[error(transparent)]
    ReadFromCacheError(#[from] ReadFromCacheError),
    #[cfg(feature = "cache")]
    #[error(transparent)]
    WriteToCacheError(#[from] WriteToCacheError),
    #[cfg(feature = "cache")]
    #[error("The cached URL was None.")]
    CachedUrlIsNone,
    #[error("The common Mapper was not found.")]
    CommonMapperNotFound,
    #[error("The mapper was not found.")]
    MapperNotFound,
    #[error(transparent)]
    CommonCallArgsError(#[from] CommonCallArgsError),
    #[error(transparent)]
    #[cfg(feature = "custom")]
    Custom(Box<dyn std::error::Error + Send>),
    #[error("The requested part of the URL was None.")]
    UrlPartIsNone
}

impl From<RuleError> for MapperError {
    fn from(value: RuleError) -> Self {
        Self::RuleError(Box::new(value))
    }
}

impl Mapper {
    /// Applies the specified variant of [`Self`].
    ///
    /// If an error is returned, `task_state` may be left in a partially modified state.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn apply(&self, task_state: &mut TaskState) -> Result<(), MapperError> {
        debug!(Mapper::apply, self, task_state);
        match self {
            // Testing.

            Self::None => {},
            Self::Error => Err(MapperError::ExplicitError)?,
            Self::Debug(mapper) => {
                let old_url = task_state.url.clone();
                let old_scratchpad = task_state.scratchpad.clone();
                let mapper_result=mapper.apply(task_state);
                eprintln!("=== Mapper::Debug ===\nMapper: {mapper:?}\nOld URL: {old_url:?}\nOld scratchpad: {old_scratchpad:?}\nMapper return value: {mapper_result:?}\nNew task_state: {task_state:?}");
                mapper_result?;
            },

            // Logic.

            Self::If {condition, mapper, else_mapper} => if condition.satisfied_by(&task_state.to_view())? {
                mapper.apply(task_state)?;
            } else if let Some(else_mapper) = else_mapper {
                else_mapper.apply(task_state)?;
            },
            Self::ConditionChain(chain) => for link in chain {
                if link.condition.satisfied_by(&task_state.to_view())? {
                    link.mapper.apply(task_state)?;
                    break;
                }
            },
            Self::All(mappers) => {
                for mapper in mappers {
                    mapper.apply(task_state)?;
                }
            },
            Self::AllIgnoreError(mappers) => {
                for mapper in mappers {
                    let _ = mapper.apply(task_state);
                }
            },
            Self::PartMap  {part , map} => if let Some(mapper) = map.get(part .get( task_state.url      ) ) {mapper.apply(task_state)?},
            Self::StringMap{value, map} => if let Some(mapper) = map.get(value.get(&task_state.to_view())?) {mapper.apply(task_state)?},

            // Error handling.

            Self::IgnoreError(mapper) => {let _=mapper.apply(task_state);},
            Self::TryElse{r#try, r#else} => r#try.apply(task_state).or_else(|try_error| r#else.apply(task_state).map_err(|else_error2| MapperError::TryElseError {try_error: Box::new(try_error), else_error: Box::new(else_error2)}))?,
            Self::FirstNotError(mappers) => {
                let mut result = Ok(());
                for mapper in mappers {
                    result = mapper.apply(task_state);
                    if result.is_ok() {break}
                }
                result?
            },
            Self::RevertOnError(mapper) => {
                let old_url = task_state.url.clone();
                let old_scratchpad = task_state.scratchpad.clone();
                if let e @ Err(_) = mapper.apply(task_state) {
                    *task_state.url = old_url;
                    *task_state.scratchpad = old_scratchpad;
                    e?;
                }
            },

            // Query.

            Self::RemoveQuery => task_state.url.set_query(None),
            Self::RemoveQueryParam(name) => if let Some(query_len) = task_state.url.query().map(|x| x.len()) {
                let task_state_view = task_state.to_view();
                let name = get_cow!(name, task_state_view, MapperError);
                let new_query = form_urlencoded::Serializer::new(String::with_capacity(query_len)).extend_pairs(task_state.url.query_pairs().filter(|(x, _)| *x != name)).finish();
                task_state.url.set_query((!new_query.is_empty()).then_some(&new_query));
            },
            Self::RemoveQueryParams(names) => if let Some(query_len) = task_state.url.query().map(|x| x.len()) {
                let new_query=form_urlencoded::Serializer::new(String::with_capacity(query_len)).extend_pairs(task_state.url.query_pairs().filter(|(name, _)| !names.contains(name.as_ref()))).finish();
                task_state.url.set_query((!new_query.is_empty()).then_some(&new_query));
            },
            Self::AllowQueryParams(names) => if let Some(query_len) = task_state.url.query().map(|x| x.len()) {
                let new_query=form_urlencoded::Serializer::new(String::with_capacity(query_len)).extend_pairs(task_state.url.query_pairs().filter(|(name, _)|  names.contains(name.as_ref()))).finish();
                task_state.url.set_query((!new_query.is_empty()).then_some(&new_query));
            },
            Self::RemoveQueryParamsMatching(matcher) => if let Some(query_len) = task_state.url.query().map(|x| x.len()) {
                let mut new_query=form_urlencoded::Serializer::new(String::with_capacity(query_len));
                for (name, value) in task_state.url.query_pairs() {
                    if !matcher.satisfied_by(&name, &task_state.to_view())? {
                        new_query.append_pair(&name, &value);
                    }
                }
                let x = new_query.finish();
                task_state.url.set_query((!x.is_empty()).then_some(&x));
            },
            Self::AllowQueryParamsMatching(matcher) => if let Some(query_len) = task_state.url.query().map(|x| x.len()) {
                let mut new_query=form_urlencoded::Serializer::new(String::with_capacity(query_len));
                for (name, value) in task_state.url.query_pairs() {
                    if matcher.satisfied_by(&name, &task_state.to_view())? {
                        new_query.append_pair(&name, &value);
                    }
                }
                let x = new_query.finish();
                task_state.url.set_query((!x.is_empty()).then_some(&x));
            },
            Self::GetUrlFromQueryParam(name) => {
                let task_state_view = task_state.to_view();
                let name = name.get(&task_state_view)?.ok_or(MapperError::StringSourceIsNone)?;

                match task_state.url.query_pairs().find(|(param_name, _)| *param_name==name) {
                    Some((_, new_url)) => {*task_state.url=Url::parse(&new_url)?.into()},
                    None => Err(MapperError::CannotFindQueryParam)?
                }
            },
            Self::GetPathFromQueryParam(name) => {
                let task_state_view = task_state.to_view();
                let name = name.get(&task_state_view)?.ok_or(MapperError::StringSourceIsNone)?;

                match task_state.url.query_pairs().find(|(param_name, _)| *param_name==name) {
                    Some((_, new_path)) => {#[expect(clippy::unnecessary_to_owned, reason = "False positive.")] task_state.url.set_path(&new_path.into_owned());},
                    None => Err(MapperError::CannotFindQueryParam)?
                }
            },

            // Other parts.

            Self::SetHost(new_host) => task_state.url.set_host(Some(new_host))?,
            Self::Join(with) => *task_state.url=task_state.url.join(get_str!(with, task_state, MapperError))?.into(),

            // Generic part handling.

            Self::SetPart{part, value} => part.set(task_state.url, value.get(&task_state.to_view())?.map(Cow::into_owned).as_deref())?, // The deref is needed for borrow checking reasons.
            Self::ModifyPart{part, modification} => if let Some(mut temp) = part.get(task_state.url).map(|x| x.into_owned()) {
                modification.apply(&mut temp, &task_state.to_view())?;
                part.set(task_state.url, Some(&temp))?;
            }
            Self::CopyPart{from, to} => to.set(task_state.url, from.get(task_state.url).map(|x| x.into_owned()).as_deref())?,
            Self::MovePart{from, to} => {
                let mut temp_url = task_state.url.clone();
                let temp_url_ref = &mut temp_url;
                to.set(temp_url_ref, from.get(temp_url_ref).map(|x| x.into_owned()).as_deref())?;
                from.set(&mut temp_url, None)?;
                *task_state.url = temp_url;
            },

            // Miscellaneous.

            #[cfg(feature = "http")]
            Self::ExpandRedirect {headers, http_client_config_diff} => {
                #[cfg(feature = "cache")]
                if task_state.params.read_cache {
                    if let Some(new_url) = task_state.cache.read("redirect", task_state.url.as_str())? {
                        *task_state.url = Url::parse(&new_url.ok_or(MapperError::CachedUrlIsNone)?)?.into();
                        return Ok(());
                    }
                }
                let response = task_state.to_view().http_client(http_client_config_diff.as_deref())?.get(task_state.url.as_str()).headers(headers.clone()).send()?;
                let new_url = if response.status().is_redirection() {
                    Url::parse(std::str::from_utf8(response.headers().get("location").ok_or(MapperError::HeaderNotFound)?.as_bytes())?)?
                } else {
                    response.url().clone()
                };
                #[cfg(feature = "cache")]
                if task_state.params.write_cache {
                    task_state.cache.write("redirect", task_state.url.as_str(), Some(new_url.as_str()))?;
                }
                *task_state.url=new_url.into();
            },

            Self::SetScratchpadFlag {name, value} => {
                let name = get_string!(name, task_state, MapperError);
                match value {
                    true  => task_state.scratchpad.flags.insert( name),
                    false => task_state.scratchpad.flags.remove(&name)
                };
            },
            Self::SetScratchpadVar {name, value} => {let _ = task_state.scratchpad.vars.insert(get_string!(name, task_state, MapperError).to_owned(), get_string!(value, task_state, MapperError).to_owned());},
            Self::DeleteScratchpadVar(name) => {
                let name = get_string!(name, task_state, MapperError).to_owned();
                let _ = task_state.scratchpad.vars.remove(&name);
            },
            Self::ModifyScratchpadVar {name, modification} => {
                let name = get_string!(name, task_state, MapperError).to_owned();
                let mut temp = task_state.scratchpad.vars.get_mut(&name).ok_or(MapperError::ScratchpadVarIsNone)?.to_owned();
                modification.apply(&mut temp, &task_state.to_view())?;
                let _ = task_state.scratchpad.vars.insert(name, temp);
            },
            Self::Rule(rule) => {rule.apply(task_state)?;},
            Self::Rules(rules) => {rules.apply(task_state)?;},
            #[cfg(feature = "cache")]
            Self::CacheUrl {category, mapper} => {
                let category = get_string!(category, task_state, MapperError);
                if task_state.params.read_cache {
                    if let Some(new_url) = task_state.cache.read(&category, task_state.url.as_str())? {
                        *task_state.url = Url::parse(&new_url.ok_or(MapperError::CachedUrlIsNone)?)?.into();
                        return Ok(());
                    }
                }
                let old_url = task_state.url.to_string();
                mapper.apply(task_state)?;
                if task_state.params.write_cache {
                    task_state.cache.write(&category, &old_url, Some(task_state.url.as_str()))?;
                }
            },
            Self::Retry {mapper, delay, limit} => {
                for i in 0..*limit {
                    match mapper.apply(task_state) {
                        Ok(()) => return Ok(()),
                        #[allow(clippy::arithmetic_side_effects, reason = "`i` is never 255 and therefore never overflows.")]
                        e @ Err(_) if i+1==*limit => e?,
                        Err(_) => {std::thread::sleep(*delay);}
                    }
                }
            },
            Self::Common(common_call) => {
                task_state.commons.mappers.get(get_str!(common_call.name, task_state, MapperError)).ok_or(MapperError::CommonMapperNotFound)?.apply(&mut TaskState {
                    common_args: Some(&common_call.args.build(&task_state.to_view())?),
                    url: task_state.url,
                    context: task_state.context,
                    params: task_state.params,
                    scratchpad: task_state.scratchpad,
                    #[cfg(feature = "cache")]
                    cache: task_state.cache,
                    commons: task_state.commons,
                    job_context: task_state.job_context
                })?
            },
            #[cfg(feature = "custom")]
            Self::Custom(function) => function(task_state)?
        };
        Ok(())
    }
}
