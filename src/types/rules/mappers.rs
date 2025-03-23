//! Logic for how a [`JobState`] should be modified.

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
    Custom(FnWrapper<fn(&mut JobState) -> Result<(), MapperError>>)
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
    #[error("A JobState string var was none.")]
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
    /// If an error is returned, `job_state` may be left in a partially modified state.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn apply(&self, job_state: &mut JobState) -> Result<(), MapperError> {
        debug!(Mapper::apply, self, job_state);
        match self {
            // Testing.

            Self::None => {},
            Self::Error => Err(MapperError::ExplicitError)?,
            Self::Debug(mapper) => {
                let old_url = job_state.url.clone();
                let old_scratchpad = job_state.scratchpad.clone();
                let mapper_result=mapper.apply(job_state);
                eprintln!("=== Mapper::Debug ===\nMapper: {mapper:?}\nOld URL: {old_url:?}\nOld scratchpad: {old_scratchpad:?}\nMapper return value: {mapper_result:?}\nNew job state: {job_state:?}");
                mapper_result?;
            },

            // Logic.

            Self::If {condition, mapper, else_mapper} => if condition.satisfied_by(&job_state.to_view())? {
                mapper.apply(job_state)?;
            } else if let Some(else_mapper) = else_mapper {
                else_mapper.apply(job_state)?;
            },
            Self::ConditionChain(chain) => for link in chain {
                if link.condition.satisfied_by(&job_state.to_view())? {
                    link.mapper.apply(job_state)?;
                    break;
                }
            },
            Self::All(mappers) => {
                for mapper in mappers {
                    mapper.apply(job_state)?;
                }
            },
            Self::AllIgnoreError(mappers) => {
                for mapper in mappers {
                    let _ = mapper.apply(job_state);
                }
            },
            Self::PartMap  {part , map} => if let Some(mapper) = map.get(part .get( job_state.url      ) ) {mapper.apply(job_state)?},
            Self::StringMap{value, map} => if let Some(mapper) = map.get(value.get(&job_state.to_view())?) {mapper.apply(job_state)?},

            // Error handling.

            Self::IgnoreError(mapper) => {let _=mapper.apply(job_state);},
            Self::TryElse{r#try, r#else} => r#try.apply(job_state).or_else(|try_error| r#else.apply(job_state).map_err(|else_error2| MapperError::TryElseError {try_error: Box::new(try_error), else_error: Box::new(else_error2)}))?,
            Self::FirstNotError(mappers) => {
                let mut result = Ok(());
                for mapper in mappers {
                    result = mapper.apply(job_state);
                    if result.is_ok() {break}
                }
                result?
            },
            Self::RevertOnError(mapper) => {
                let old_url = job_state.url.clone();
                let old_scratchpad = job_state.scratchpad.clone();
                if let e @ Err(_) = mapper.apply(job_state) {
                    *job_state.url = old_url;
                    *job_state.scratchpad = old_scratchpad;
                    e?;
                }
            },

            // Query.

            Self::RemoveQuery => job_state.url.set_query(None),
            Self::RemoveQueryParam(name) => if let Some(query_len) = job_state.url.query().map(|x| x.len()) {
                let job_state_view = job_state.to_view();
                let name = get_cow!(name, job_state_view, MapperError);
                let new_query = form_urlencoded::Serializer::new(String::with_capacity(query_len)).extend_pairs(job_state.url.query_pairs().filter(|(x, _)| *x != name)).finish();
                job_state.url.set_query((!new_query.is_empty()).then_some(&new_query));
            },
            Self::RemoveQueryParams(names) => if let Some(query_len) = job_state.url.query().map(|x| x.len()) {
                let new_query=form_urlencoded::Serializer::new(String::with_capacity(query_len)).extend_pairs(job_state.url.query_pairs().filter(|(name, _)| !names.contains(name.as_ref()))).finish();
                job_state.url.set_query((!new_query.is_empty()).then_some(&new_query));
            },
            Self::AllowQueryParams(names) => if let Some(query_len) = job_state.url.query().map(|x| x.len()) {
                let new_query=form_urlencoded::Serializer::new(String::with_capacity(query_len)).extend_pairs(job_state.url.query_pairs().filter(|(name, _)|  names.contains(name.as_ref()))).finish();
                job_state.url.set_query((!new_query.is_empty()).then_some(&new_query));
            },
            Self::RemoveQueryParamsMatching(matcher) => if let Some(query_len) = job_state.url.query().map(|x| x.len()) {
                let mut new_query=form_urlencoded::Serializer::new(String::with_capacity(query_len));
                for (name, value) in job_state.url.query_pairs() {
                    if !matcher.satisfied_by(&name, &job_state.to_view())? {
                        new_query.append_pair(&name, &value);
                    }
                }
                let x = new_query.finish();
                job_state.url.set_query((!x.is_empty()).then_some(&x));
            },
            Self::AllowQueryParamsMatching(matcher) => if let Some(query_len) = job_state.url.query().map(|x| x.len()) {
                let mut new_query=form_urlencoded::Serializer::new(String::with_capacity(query_len));
                for (name, value) in job_state.url.query_pairs() {
                    if matcher.satisfied_by(&name, &job_state.to_view())? {
                        new_query.append_pair(&name, &value);
                    }
                }
                let x = new_query.finish();
                job_state.url.set_query((!x.is_empty()).then_some(&x));
            },
            Self::GetUrlFromQueryParam(name) => {
                let job_state_view = job_state.to_view();
                let name = name.get(&job_state_view)?.ok_or(MapperError::StringSourceIsNone)?;

                match job_state.url.query_pairs().find(|(param_name, _)| *param_name==name) {
                    Some((_, new_url)) => {*job_state.url=Url::parse(&new_url)?.into()},
                    None => Err(MapperError::CannotFindQueryParam)?
                }
            },
            Self::GetPathFromQueryParam(name) => {
                let job_state_view = job_state.to_view();
                let name = name.get(&job_state_view)?.ok_or(MapperError::StringSourceIsNone)?;

                match job_state.url.query_pairs().find(|(param_name, _)| *param_name==name) {
                    Some((_, new_path)) => {#[expect(clippy::unnecessary_to_owned, reason = "False positive.")] job_state.url.set_path(&new_path.into_owned());},
                    None => Err(MapperError::CannotFindQueryParam)?
                }
            },

            // Other parts.

            Self::SetHost(new_host) => job_state.url.set_host(Some(new_host))?,
            Self::Join(with) => *job_state.url=job_state.url.join(get_str!(with, job_state, MapperError))?.into(),

            // Generic part handling.

            Self::SetPart{part, value} => part.set(job_state.url, value.get(&job_state.to_view())?.map(Cow::into_owned).as_deref())?, // The deref is needed for borrow checking reasons.
            Self::ModifyPart{part, modification} => if let Some(mut temp) = part.get(job_state.url).map(|x| x.into_owned()) {
                modification.apply(&mut temp, &job_state.to_view())?;
                part.set(job_state.url, Some(&temp))?;
            }
            Self::CopyPart{from, to} => to.set(job_state.url, from.get(job_state.url).map(|x| x.into_owned()).as_deref())?,
            Self::MovePart{from, to} => {
                let mut temp_url = job_state.url.clone();
                let temp_url_ref = &mut temp_url;
                to.set(temp_url_ref, from.get(temp_url_ref).map(|x| x.into_owned()).as_deref())?;
                from.set(&mut temp_url, None)?;
                *job_state.url = temp_url;
            },

            // Miscellaneous.

            #[cfg(feature = "http")]
            Self::ExpandRedirect {headers, http_client_config_diff} => {
                #[cfg(feature = "cache")]
                if job_state.params.read_cache {
                    if let Some(new_url) = job_state.cache.read("redirect", job_state.url.as_str())? {
                        *job_state.url = Url::parse(&new_url.ok_or(MapperError::CachedUrlIsNone)?)?.into();
                        return Ok(());
                    }
                }
                let response = job_state.to_view().http_client(http_client_config_diff.as_deref())?.get(job_state.url.as_str()).headers(headers.clone()).send()?;
                let new_url = if response.status().is_redirection() {
                    Url::parse(std::str::from_utf8(response.headers().get("location").ok_or(MapperError::HeaderNotFound)?.as_bytes())?)?
                } else {
                    response.url().clone()
                };
                #[cfg(feature = "cache")]
                if job_state.params.write_cache {
                    job_state.cache.write("redirect", job_state.url.as_str(), Some(new_url.as_str()))?;
                }
                *job_state.url=new_url.into();
            },

            Self::SetScratchpadFlag {name, value} => {
                let name = get_string!(name, job_state, MapperError);
                match value {
                    true  => job_state.scratchpad.flags.insert( name),
                    false => job_state.scratchpad.flags.remove(&name)
                };
            },
            Self::SetScratchpadVar {name, value} => {let _ = job_state.scratchpad.vars.insert(get_string!(name, job_state, MapperError).to_owned(), get_string!(value, job_state, MapperError).to_owned());},
            Self::DeleteScratchpadVar(name) => {
                let name = get_string!(name, job_state, MapperError).to_owned();
                let _ = job_state.scratchpad.vars.remove(&name);
            },
            Self::ModifyScratchpadVar {name, modification} => {
                let name = get_string!(name, job_state, MapperError).to_owned();
                let mut temp = job_state.scratchpad.vars.get_mut(&name).ok_or(MapperError::ScratchpadVarIsNone)?.to_owned();
                modification.apply(&mut temp, &job_state.to_view())?;
                let _ = job_state.scratchpad.vars.insert(name, temp);
            },
            Self::Rule(rule) => {rule.apply(job_state)?;},
            Self::Rules(rules) => {rules.apply(job_state)?;},
            #[cfg(feature = "cache")]
            Self::CacheUrl {category, mapper} => {
                let category = get_string!(category, job_state, MapperError);
                if job_state.params.read_cache {
                    if let Some(new_url) = job_state.cache.read(&category, job_state.url.as_str())? {
                        *job_state.url = Url::parse(&new_url.ok_or(MapperError::CachedUrlIsNone)?)?.into();
                        return Ok(());
                    }
                }
                let old_url = job_state.url.to_string();
                mapper.apply(job_state)?;
                if job_state.params.write_cache {
                    job_state.cache.write(&category, &old_url, Some(job_state.url.as_str()))?;
                }
            },
            Self::Retry {mapper, delay, limit} => {
                for i in 0..*limit {
                    match mapper.apply(job_state) {
                        Ok(()) => return Ok(()),
                        #[allow(clippy::arithmetic_side_effects, reason = "`i` is never 255 and therefore never overflows.")]
                        e @ Err(_) if i+1==*limit => e?,
                        Err(_) => {std::thread::sleep(*delay);}
                    }
                }
            },
            Self::Common(common_call) => {
                job_state.commons.mappers.get(get_str!(common_call.name, job_state, MapperError)).ok_or(MapperError::CommonMapperNotFound)?.apply(&mut JobState {
                    common_args: Some(&common_call.args.build(&job_state.to_view())?),
                    url: job_state.url,
                    context: job_state.context,
                    params: job_state.params,
                    scratchpad: job_state.scratchpad,
                    #[cfg(feature = "cache")]
                    cache: job_state.cache,
                    commons: job_state.commons,
                    jobs_context: job_state.jobs_context
                })?
            },
            #[cfg(feature = "custom")]
            Self::Custom(function) => function(job_state)?
        };
        Ok(())
    }
}
