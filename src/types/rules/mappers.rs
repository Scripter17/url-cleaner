//! The logic for how to modify a URL.

use std::str::Utf8Error;
use std::collections::hash_set::HashSet;
use std::path::PathBuf;

use serde::{Serialize, Deserialize};
use thiserror::Error;
use url::Url;
#[cfg(all(feature = "http", not(target_family = "wasm")))]
use reqwest::header::HeaderMap;

use crate::glue::*;
use crate::types::*;
use crate::util::*;

/// The part of a [`Rule`] that specifies how to modify a [`Url`] if the rule's condition passes.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Mapper {

    // Testing.

    /// Does nothing.
    None,
    /// Always returns the error [`MapperError::ExplicitError`].
    /// # Errors
    /// Always returns the error [`MapperError::ExplicitError`].
    Error,
    /// Prints debugging information about the contained [`Self`] and the details of its application to STDERR.
    /// Intended primarily for debugging logic errors.
    /// *Can* be used in production as in both bash and batch `x | y` only pipes `x`'s STDOUT, but you probably shouldn't.
    /// # Errors
    /// If the contained [`Self`] returns an error, that error is returned after the debug info is printed.
    Debug(Box<Self>),

    // Logic.

    /// If `r#if` passes, apply `then`, otherwise apply `r#else`.
    /// # Errors
    /// If `r#if` returns an error, that error is returned.
    /// If `r#if` passes and `then` returns an error, that error is returned.
    /// If `r#if` fails and `r#else` returns an error, that error is returned.
    IfCondition {
        /// The [`Condition`] that decides if `then` or `r#else` is used.
        r#if: Condition,
        /// The [`Self`] to use if `r#if` passes.
        then: Box<Self>,
        /// The [`Self`] to use if `r#if` fails.
        #[serde(default = "box_mapper_none")]
        r#else: Box<Self>
    },
    /// Applies the contained [`Self`]s in order.
    /// # Errors
    /// If one of the contained [`Self`]s returns an error, the URL is left unchanged and the error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut url=Url::parse("https://www.example.com").unwrap();
    /// Mapper::All(vec![Mapper::SetHost("2.com".to_string()), Mapper::Error]).apply(&mut url, &Params::default()).unwrap_err();
    /// assert_eq!(url.domain(), Some("www.example.com"));
    /// ```
    All(Vec<Self>),
    /// Applies the contained [`Self`]s in order. If an error occurs, the URL remains changed by the previous contained [`Self`]s and the error is returned.
    /// Technically the name is wrong as [`Self::All`] only actually applies the change after all the contained [`Self`] pass, but this is conceptually simpler.
    /// # Errors
    /// If one of the contained [`Self`]s returns an error, the URL is left as whatever the previous contained mapper set it to and the error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut url=Url::parse("https://www.example.com").unwrap();
    /// Mapper::AllNoRevert(vec![Mapper::SetHost("3.com".to_string()), Mapper::Error, Mapper::SetHost("4.com".to_string())]).apply(&mut url, &Params::default()).unwrap_err();
    /// assert_eq!(url.domain(), Some("3.com"));
    /// ```
    AllNoRevert(Vec<Self>),
    /// If any of the contained [`Self`]s returns an error, the error is ignored and subsequent [`Self`]s are still applied.
    /// This is equivalent to wrapping every contained [`Self`] in a [`Self::IgnoreError`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut url=Url::parse("https://www.example.com").unwrap();
    /// Mapper::AllIgnoreError(vec![Mapper::SetHost("5.com".to_string()), Mapper::Error, Mapper::SetHost("6.com".to_string())]).apply(&mut url, &Params::default()).unwrap();
    /// assert_eq!(url.domain(), Some("6.com"));
    /// ```
    AllIgnoreError(Vec<Self>),

    // Error handling.

    /// Ignores any error the contained [`Self`] may return.
    IgnoreError(Box<Self>),
    /// If `try` returns an error, `else` is applied.
    /// If `try` does not return an error, `else` is not applied.
    /// # Errors
    /// If `else` returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// Mapper::TryElse {r#try: Box::new(Mapper::None ), r#else: Box::new(Mapper::None )}.apply(&mut Url::parse("https://www.example.com").unwrap(), &Params::default()).unwrap ();
    /// Mapper::TryElse {r#try: Box::new(Mapper::None ), r#else: Box::new(Mapper::Error)}.apply(&mut Url::parse("https://www.example.com").unwrap(), &Params::default()).unwrap ();
    /// Mapper::TryElse {r#try: Box::new(Mapper::Error), r#else: Box::new(Mapper::None )}.apply(&mut Url::parse("https://www.example.com").unwrap(), &Params::default()).unwrap ();
    /// Mapper::TryElse {r#try: Box::new(Mapper::Error), r#else: Box::new(Mapper::Error)}.apply(&mut Url::parse("https://www.example.com").unwrap(), &Params::default()).unwrap_err();
    /// ```
    TryElse {
        /// The [`Self`] to try first.
        r#try: Box<Self>,
        /// If `try` fails, instead return the result of this one.
        r#else: Box<Self>
    },
    /// Effectively a [`Self::TryElse`] chain but less ugly.
    /// # Errors
    /// If every contained [`Self`] returns an error, returns the last error.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut url=Url::parse("https://www.example.com").unwrap();
    /// Mapper::FirstNotError(vec![Mapper::SetHost("1.com".to_string()), Mapper::SetHost("2.com".to_string())]).apply(&mut url, &Params::default()).unwrap();
    /// assert_eq!(url.domain(), Some("1.com"));
    /// Mapper::FirstNotError(vec![Mapper::SetHost("3.com".to_string()), Mapper::Error                       ]).apply(&mut url, &Params::default()).unwrap();
    /// assert_eq!(url.domain(), Some("3.com"));
    /// Mapper::FirstNotError(vec![Mapper::Error                       , Mapper::SetHost("4.com".to_string())]).apply(&mut url, &Params::default()).unwrap();
    /// assert_eq!(url.domain(), Some("4.com"));
    /// Mapper::FirstNotError(vec![Mapper::Error                       , Mapper::Error                       ]).apply(&mut url, &Params::default()).unwrap_err();
    /// assert_eq!(url.domain(), Some("4.com"));
    /// ```
    FirstNotError(Vec<Self>),

    // Query.

    /// Removes the URL's entire query.
    /// Useful for websites that only use the query for tracking.
    RemoveQuery,
    /// Removes all query parameters whose name exists in the specified [`std::collections::HashMap`].
    /// Useful for websites that append random stuff to shared URLs so the website knows your friend got that link from you.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// # use std::collections::hash_set::HashSet;
    /// let mut url=Url::parse("https://example.com?a=2&b=3&c=4&d=5").unwrap();
    /// Mapper::RemoveQueryParams(HashSet::from(["a".to_string()])).apply(&mut url, &Params::default()).unwrap();
    /// assert_eq!(url.query(), Some("b=3&c=4&d=5"));
    /// Mapper::RemoveQueryParams(HashSet::from(["b".to_string(), "c".to_string()])).apply(&mut url, &Params::default()).unwrap();
    /// assert_eq!(url.query(), Some("d=5"));
    /// Mapper::RemoveQueryParams(HashSet::from(["d".to_string()])).apply(&mut url, &Params::default()).unwrap();
    /// assert_eq!(url.query(), None);
    /// ```
    RemoveQueryParams(HashSet<String>),
    /// Keeps only the query parameters whose name exists in the specified [`HashSet`].
    /// Useful for websites that keep changing their tracking parameters and you're sick of updating your rule set.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// # use std::collections::hash_set::HashSet;
    /// let mut url=Url::parse("https://example.com?a=2&b=3&c=4&d=5").unwrap();
    /// Mapper::RemoveQueryParams(HashSet::from(["a".to_string()])).apply(&mut url, &Params::default()).unwrap();
    /// ```
    AllowQueryParams(HashSet<String>),
    /// Removes all query parameters whose name matches the specified [`StringMatcher`].
    /// # Errors
    /// If the call to [`StringMatcher::satisfied_by`] returns an error, that error is returned.
    #[cfg(feature = "string-matcher")]
    RemoveQueryParamsMatching(StringMatcher),
    /// Keeps only the query parameters whose name matches the specified [`StringMatcher`].
    /// # Errors
    /// If the call to [`StringMatcher::satisfied_by`] returns an error, that error is returned.
    #[cfg(feature = "string-matcher")]
    AllowQueryParamsMatching(StringMatcher),
    /// Replace the current URL with the value of the specified query parameter.
    /// Useful for websites for have a "are you sure you want to leave?" page with a URL like `https://example.com/outgoing?to=https://example.com`.
    /// # Errors
    /// If the specified query parameter cannot be found, returns the error [`MapperError::CannotFindQueryParam`].
    /// 
    /// If the query parameter is found but its value cannot be parsed as a URL, returns the error [`MapperError::UrlParseError`].
    GetUrlFromQueryParam(String),
    /// Replace the current URL's path with the value of the specified query parameter.
    /// Useful for websites that have a "you must log in to see this page" page.
    /// # Errors
    /// If the specified query parameter cannot be found, returns the error [`MapperError::CannotFindQueryParam`].
    GetPathFromQueryParam(String),

    // Other parts.

    /// Replaces the URL's host to the provided host.
    /// Useful for websites that are just a wrapper around another website. For example, `vxtwitter.com`.
    /// # Errors
    /// If the resulting string cannot be parsed as a URL, returns the error [`MapperError::UrlParseError`].
    /// See [`Url::set_host`] for details.
    SetHost(String),
    /// Removes the path segments with an index in the specified list.
    /// See [`Url::path_segments`] for details.
    /// # Errors
    /// If the URL cannot be a base, returns the error [`MapperError::UrlDoesNotHaveAPath`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut url=Url::parse("https://example.com/0/1/2/3/4/5/6").unwrap();
    /// Mapper::RemovePathSegments(vec![1,3,5,6,8]).apply(&mut url, &Params::default()).unwrap();
    /// assert_eq!(url.path(), "/0/2/4");
    /// ```
    RemovePathSegments(Vec<usize>),
    /// [`Url::join`].
    #[cfg(feature = "string-source")]
    Join(StringSource),
    /// [`Url::join`].
    #[cfg(not(feature = "string-source"))]
    Join(String),

    // Generic part handling.

    /// Sets the specified URL part to `to`.
    /// # Errors
    /// If the call to [`StringSource::get`] return's an error, that error is returned.
    /// 
    /// If the call to [`UrlPart::set`] returns an error, that error is returned.
    SetPart {
        /// The name of the part to replace.
        part: UrlPart,
        /// The value to set the part to.
        #[cfg(feature = "string-source")]
        value: Option<StringSource>,
        /// The value to set the part to.
        #[cfg(not(feature = "string-source"))]
        value: Option<String>
    },
    /// Modifies the specified part of the URL.
    /// # Errors
    /// If the call to [`StringModification::apply`] returns an error, that error is returned in a [`MapperError::StringModificationError`].
    /// 
    /// If the call to [`UrlPart::modify`] returns an error, that error is returned in a [`MapperError::UrlPartModifyError`].
    #[cfg(feature = "string-modification")]
    ModifyPart {
        /// The name of the part to modify.
        part: UrlPart,
        /// How exactly to modify the part.
        how: StringModification
    },
    /// Copies the part specified by `from` to the part specified by `to`.
    /// # Errors
    /// If the part specified by `from` is None and the part specified by `to` cannot be `None` (see [`Mapper::SetPart`]), returns the error [`UrlPartSetError::PartCannotBeNone`].
    CopyPart {
        /// The part to get the value from.
        from: UrlPart,
        /// The part to set to `from`'s value.
        to: UrlPart
    },   

    // Miscellaneous.

    /// Sends an HTTP request to the current URL and replaces it with the URL the website responds with.
    /// Useful for link shorteners like `bit.ly` and `t.co`.
    /// This mapper only works on non-WASM targets.
    /// This is both because CORS makes this mapper useless and because `reqwest::blocking` does not work on WASM targets.
    /// See [reqwest#891](https://github.com/seanmonstar/reqwest/issues/891) and [reqwest#1068](https://github.com/seanmonstar/reqwest/issues/1068) for details.
    /// # Errors
    /// If the call to [`Params::get_redirect_from_cache`] returns an error, that error is returned.
    /// If the [`reqwest::blocking::Client`] is not able to send the HTTP request, returns the error [`MapperError::ReqwestError`].
    /// All errors regarding caching the redirect to disk are ignored. This may change in the future.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// # use reqwest::header::HeaderMap;
    /// let mut url = Url::parse("https://t.co/H8IF8DHSFL").unwrap();
    /// Mapper::ExpandShortLink{headers: HeaderMap::default()}.apply(&mut url, &Params::default()).unwrap();
    /// assert_eq!(url.as_str(), "https://www.eff.org/deeplinks/2024/01/eff-and-access-now-submission-un-expert-anti-lgbtq-repression");
    /// ```
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
    ExpandShortLink {
        /// The headers to send alongside the param's default headers.
        #[serde(default, with = "headermap")]
        headers: HeaderMap,
        /// Rules for how to make the HTTP client.
        #[serde(default)]
        http_client_config_diff: Option<HttpClientConfigDiff>
    },
    /// If [`StringSource::get`] returns `Ok(Some(x))`, [`println`]'s `x`.
    /// If it returns `Ok(None)`, doesn't print anything.
    /// Does not change the URL at all.
    /// # Errors
    /// If [`StringSource::get`] returns an error, that error is returned.
    #[cfg(feature = "string-source")]
    Println(StringSource),
    /// If [`StringSource::get`] returns `Ok(Some(x))`, [`print`]'s `x`.
    /// If it returns `Ok(None)`, doesn't print anything.
    /// Does not change the URL at all.
    /// # Errors
    /// If [`StringSource::get`] returns an error, that error is returned.
    #[cfg(feature = "string-source")]
    Eprintln(StringSource),
    /// If [`StringSource::get`] returns `Ok(Some(x))`, [`eprintln`]'s `x`.
    /// If it returns `Ok(None)`, doesn't print anything.
    /// Does not change the URL at all.
    /// # Errors
    /// If [`StringSource::get`] returns an error, that error is returned.
    #[cfg(feature = "string-source")]
    Print(StringSource),
    /// If [`StringSource::get`] returns `Ok(Some(x))`, [`eprint`]'s `x`.
    /// If it returns `Ok(None)`, doesn't print anything.
    /// Does not change the URL at all.
    /// # Errors
    /// If [`StringSource::get`] returns an error, that error is returned.
    #[cfg(feature = "string-source")]
    Eprint(StringSource),
    /// Loads a config specified by `path` (or the default config if it's [`None`]) and applies it.
    /// # Errors
    /// If the call to [`Config::get_default_or_load`] returns an error, that error is returned.
    /// 
    /// If the call to [`Config::apply`] returns an error, that error is returned.
    ApplyConfig {
        /// The path of the config to load. Loads the default config if [`None`].
        path: Option<PathBuf>,
        /// How to modify the config's params.
        params_diff: ParamsDiff
    }
}

/// The default value of [`Mapper::IfCondition::r#else`].
fn box_mapper_none() -> Box<Mapper> {Box::new(Mapper::None)}

/// An enum of all possible errors a [`Mapper`] can return.
#[derive(Error, Debug)]
pub enum MapperError {
    /// Returned when [`Mapper::Error`] is used.
    #[error("Mapper::Error was used.")]
    ExplicitError,
    /// Returned when a call to [`UrlPart::get`] returns `None` where it has to return `Some`.
    #[error("The provided URL does not have the requested part.")]
    UrlPartNotFound,
    /// Returned when the provided URL does not contain the requested query parameter.
    #[error("The provided URL does not contain the requested query parameter.")]
    CannotFindQueryParam,
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when a [`reqwest::Error`] is encountered.
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    /// Returned when a [`Utf8Error`] is encountered.
    #[error(transparent)]
    Utf8Error(#[from] Utf8Error),
    /// Returned when a [`UrlPartModifyError`] is encountered.
    #[cfg(feature = "string-modification")]
    #[error(transparent)]
    UrlPartModifyError(#[from] UrlPartModifyError),
    /// Returned when a [`UrlPartSetError`] is encountered.
    #[error(transparent)]
    UrlPartSetError(#[from] UrlPartSetError),
    /// Returned when the provided URL does not have a path.
    #[error("The URL does not have a path.")]
    UrlDoesNotHaveAPath,
    /// Returned when a regex does not find any matches.
    #[error("A regex pattern did not find any matches.")]
    NoRegexMatchesFound,
    /// Returned when the requested variable is not found in [`Params::vars`].
    #[error("A variable was requested but not found at runtime.")]
    VarNotFound,
    /// Returned when a call to [`StringSource::get`] returns `None` where it has to be `Some`.
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,
    /// Returned when a [`StringMatcherError`] is encountered.
    #[cfg(feature = "string-matcher")]
    #[error(transparent)]
    StringMatcherError(#[from] StringMatcherError),
    /// Returned when a [`StringSourceError`] is encountered.
    #[cfg(feature = "string-source")]
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
    /// Returned when a [`StringModificationError`] is encountered.
    #[cfg(feature = "string-modification")]
    #[error(transparent)]
    StringModificationError(#[from] StringModificationError),
    /// Returned when a [`ReadCacheError`] is encountered.
    #[cfg(feature = "cache")]
    #[error(transparent)]
    ReadCacheError(#[from] ReadCacheError),
    /// Returned when a [`WriteCacheError`] is encountered.
    #[cfg(feature = "cache")]
    #[error(transparent)]
    WriteCacheError(#[from] WriteCacheError),
    /// Returned when a [`ConditionError`] is encountered.
    #[error(transparent)]
    ConditionError(#[from] ConditionError),
    /// Returned when a [`GetConfigError`] is encountered.
    #[error(transparent)]
    GetConfigError(#[from] GetConfigError),
    /// Returned when a [`RuleError`] is encountered.
    #[error(transparent)]
    RuleError(Box<RuleError>),
    /// Returned when the requested header is not found.
    #[error("The requested header was not found.")]
    HeaderNotFound,
    /// Returned when a [`reqwest::header::ToStrError`] is encountered.
    #[error(transparent)]
    ToStrError(#[from] reqwest::header::ToStrError)
}

impl From<RuleError> for MapperError {
    fn from(value: RuleError) -> Self {
        Self::RuleError(Box::new(value))
    }
}

impl Mapper {
    /// Applies the mapper to the provided URL.
    /// Does not check with a [`crate::types::Condition`]. You should do that yourself or use [`crate::types::Rule`].
    /// # Errors
    /// If the mapper has an error, that error is returned.
    /// See [`Mapper`]'s documentation for details.
    pub fn apply(&self, url: &mut Url, params: &Params) -> Result<(), MapperError> {
        #[cfg(feature = "debug")]
        println!("Mapper: {self:?}");
        match self {
            // Testing.

            Self::None => {},
            Self::Error => Err(MapperError::ExplicitError)?,
            Self::Debug(mapper) => {
                let url_before_mapper=url.clone();
                let mapper_result=mapper.apply(url, params);
                eprintln!("=== Mapper::Debug ===\nMapper: {mapper:?}\nParams: {params:?}\nURL before mapper: {url_before_mapper:?}\nMapper return value: {mapper_result:?}\nURL after mapper: {url:?}");
                mapper_result?;
            }

            // Logic.

            Self::IfCondition {r#if, then, r#else} => if r#if.satisfied_by(url, params)? {then} else {r#else}.apply(url, params)?,
            Self::All(mappers) => {
                let mut temp_url=url.clone();
                for mapper in mappers {
                    mapper.apply(&mut temp_url, params)?;
                }
                *url=temp_url;
            },
            Self::AllNoRevert(mappers) => {
                for mapper in mappers {
                    mapper.apply(url, params)?;
                }
            },
            Self::AllIgnoreError(mappers) => {
                for mapper in mappers {
                    let _=mapper.apply(url, params);
                }
            },

            // Error handling.

            Self::IgnoreError(mapper) => {let _=mapper.apply(url, params);},
            Self::TryElse{r#try, r#else} => r#try.apply(url, params).or_else(|_| r#else.apply(url, params))?,
            Self::FirstNotError(mappers) => {
                let mut result = Ok(());
                for mapper in mappers {
                    result = mapper.apply(url, params);
                    if result.is_ok() {break}
                }
                result?
            },

            // Query.

            Self::RemoveQuery => url.set_query(None),
            Self::RemoveQueryParams(names) => {
                let new_query=form_urlencoded::Serializer::new(String::new()).extend_pairs(url.query_pairs().filter(|(name, _)| !names.contains(name.as_ref()))).finish();
                url.set_query((!new_query.is_empty()).then_some(&new_query));
            },
            Self::AllowQueryParams(names) => {
                let new_query=form_urlencoded::Serializer::new(String::new()).extend_pairs(url.query_pairs().filter(|(name, _)|  names.contains(name.as_ref()))).finish();
                url.set_query((!new_query.is_empty()).then_some(&new_query));
            },
            #[cfg(feature = "string-matcher")]
            Self::RemoveQueryParamsMatching(matcher) => {
                let mut new_query=form_urlencoded::Serializer::new(String::new());
                for (name, value) in url.query_pairs() {
                    if !matcher.satisfied_by(&name, url, params)? {
                        new_query.append_pair(&name, &value);
                    }
                }
                let x = new_query.finish();
                url.set_query((!x.is_empty()).then_some(&x));
            },
            #[cfg(feature = "string-matcher")]
            Self::AllowQueryParamsMatching(matcher) => {
                let mut new_query=form_urlencoded::Serializer::new(String::new());
                for (name, value) in url.query_pairs() {
                    if matcher.satisfied_by(&name, url, params)? {
                        new_query.append_pair(&name, &value);
                    }
                }
                let x = new_query.finish();
                url.set_query((!x.is_empty()).then_some(&x));
            },
            Self::GetUrlFromQueryParam(name) => {
                match url.query_pairs().find(|(param_name, _)| param_name==name) {
                    Some((_, new_url)) => {*url=Url::parse(&new_url)?},
                    None => Err(MapperError::CannotFindQueryParam)?
                }
            },
            Self::GetPathFromQueryParam(name) => {
                match url.query_pairs().find(|(param_name, _)| param_name==name) {
                    Some((_, new_path)) => {#[allow(clippy::unnecessary_to_owned)] url.set_path(&new_path.into_owned());},
                    None => Err(MapperError::CannotFindQueryParam)?
                }
            },

            // Other parts.

            Self::SetHost(new_host) => url.set_host(Some(new_host))?,
            Self::RemovePathSegments(indices) => url.set_path(&url.path_segments().ok_or(MapperError::UrlDoesNotHaveAPath)?.enumerate().filter_map(|(i, x)| (!indices.contains(&i)).then_some(x)).collect::<Vec<_>>().join("/")),
            Self::Join(with) => *url=url.join(get_string!(with, url, params, MapperError))?,

            // Generic part handling.

            Self::SetPart{part, value} => part.set(url, get_option_string!(value, url, params).map(|x| x.to_owned()).as_deref())?,
            #[cfg(feature = "string-modification")]
            Self::ModifyPart{part, how} => part.modify(how, url, params)?,
            Self::CopyPart{from, to} => to.set(url, from.get(url).map(|x| x.into_owned()).as_deref())?,

            // Miscellaneous.

            #[cfg(all(feature = "http", not(target_family = "wasm")))]
            Self::ExpandShortLink {headers, http_client_config_diff} => {
                #[cfg(feature = "cache-redirects")]
                if let Some(cached_result) = params.get_redirect_from_cache(url)? {
                    *url = cached_result;
                    return Ok(())
                }
                let response = params.http_client(http_client_config_diff.as_ref())?.get(url.as_str()).headers(headers.clone()).send()?;
                let new_url = if response.status() == reqwest::StatusCode::MOVED_PERMANENTLY {
                    println!("{:?}", response.url());
                    Url::parse(response.headers().get("location").ok_or(MapperError::HeaderNotFound)?.to_str()?)?
                } else {
                    response.url().clone()
                };
                #[cfg(feature = "cache-redirects")]
                params.write_redirect_to_cache(url, &new_url)?;
                *url=new_url;
            },

            #[cfg(feature = "string-source")] Self::Println (source) => if let Some(x) = source.get(url, params)? {println! ("{x}");},
            #[cfg(feature = "string-source")] Self::Print   (source) => if let Some(x) = source.get(url, params)? {print!   ("{x}");},
            #[cfg(feature = "string-source")] Self::Eprintln(source) => if let Some(x) = source.get(url, params)? {eprintln!("{x}");},
            #[cfg(feature = "string-source")] Self::Eprint  (source) => if let Some(x) = source.get(url, params)? {eprint!  ("{x}");},
            Self::ApplyConfig {path, params_diff} => {
                let mut config = Config::get_default_or_load(path.as_deref())?.into_owned();
                params_diff.apply(&mut config.params);
                config.apply(url)?;
            }
        };
        Ok(())
    }
}
