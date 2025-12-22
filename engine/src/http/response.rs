//! [`HttpResponseHandler`].

#![allow(unused_assignments, reason = "False positive.")]

use std::borrow::Cow;
use std::io::Read;

use serde::{Deserialize, Serialize};
use thiserror::Error;
#[expect(unused_imports, reason = "Used in doc comments.")]
use reqwest::header::HeaderValue;
use reqwest::StatusCode;

use crate::prelude::*;

/// What part of a response a [`HttpRequestConfig`] should return.
///
/// Defaults to [`Self::Body`].
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Suitability)]
#[serde(deny_unknown_fields)]
pub enum HttpResponseHandler {
    /// Always returns [`None`].
    ///
    /// Deserializes from and serializes to `null`.
    None,
    /// Return the value of a [`Self::String::0`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    String(StringSource),
    /// Always returns [`HttpResponseHandlerError::ExplicitError`] with the included error.
    /// # Errors
    /// Always returns the error [`HttpResponseHandlerError::ExplicitError`].
    Error(String),
    /// If [`Self::TryElse::try`]'s call to [`Self::handle`] returns an error, instead return the value of [`Self::TryElse::else`].
    /// # Errors
    #[doc = edoc!(callerrte(Self::handle, HttpResponseHandler))]
    TryElse {
        /// The hanlder to try to use.
        ///
        /// If it's an error, use [`Self::TryElse::else`].
        r#try: Box<Self>,
        /// The handler to use if [`Self::TryElse::try`] is an error.
        r#else: Box<Self>
    },
    /// [`Self::TryElse`] with a [`Self::Modified`] only if the first [`Self`] worked.
    /// # Errors
    #[doc = edoc!(applyerr(StringModification), callerrte(Self::handle, HttpResponseHandler))]
    TryThenModifiedOrElse {
        /// The [`Self`] to try.
        r#try: Box<Self>,
        /// The [`StringModification`] to apply if successful.
        modification: StringModification,
        /// The [`Self`] to use if [`Self::TryThenModifiedOrElse`] was unsuccessful.
        r#else: Box<Self>
    },
    /// Calls [`Self::handle`] on each contained [`Self`] in order, returning the first to return [`Ok`].
    /// # Errors
    #[doc = edoc!(callerrfne(Self::handle, HttpResponseHandler))]
    FirstNotError(Vec<Self>),
    /// Print debug info about the contained [`Self`] and its call to [`Self::handle`].
    ///
    /// The exact info printed is unspecified and subject to change at any time for any reason.
    /// # Suitability
    /// Always unsuitable to be in the bundled cleaner.
    #[suitable(never)]
    Debug(Box<Self>),
    /// If [`Self::NoneTo::handler`] is [`Some`], return it. Otherwise return [`Self::NoneTo::if_none`].
    /// # Errors
    #[doc = edoc!(callerr(Self::handle, 2))]
    NoneTo {
        /// The handler to return if it's [`Some`].
        handler: Box<Self>,
        /// The handler to return if [`Self::NoneTo::handler`] is [`None`].
        if_none: Box<Self>
    },
    /// If the handler of the contained [`Self`] is [`None`], return the empty string.
    /// # Errors
    #[doc = edoc!(callerr(Self::handle))]
    NoneToEmpty(Box<Self>),
    /// If the handler of the contained [`Self`] is the empty string, return [`None`].
    /// # Errors
    #[doc = edoc!(callerr(Self::handle))]
    EmptyToNone(Box<Self>),
    /// If [`Self::AssertMatches::handler`] satisfies [`Self::AssertMatches::matcher`], return it. Otherwise return the error [`StringSourceError::AssertMatchesFailed`].
    /// # Errors
    /// If [`Self::AssertMatches::handler`] doesn't satisfy [`Self::AssertMatches::matcher`], returns the error [`StringSourceError::AssertMatchesFailed`].
    AssertMatches {
        /// The [`Self`] to assert matches [`Self::AssertMatches::matcher`].
        handler: Box<Self>,
        /// The [`StringMatcher`] to match [`Self::AssertMatches::handler`].
        matcher: StringMatcher,
        /// The error message. Defaults to [`Self::None`].
        #[serde(default, skip_serializing_if = "is_default")]
        message: StringSource
    },

    /// Get the response body.
    /// # Errors
    /// If the call to [`reqwest::blocking::Response::text`] returns an error, that error is returned.
    #[default]
    Body,
    /// Get the specified header.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, HttpResponseHandlerError), callerr(str::from_utf8))]
    Header(StringSource),
    /// Get the final URL.
    Url,

    /// Applies [`Self::Modified::modification`] to [`Self::Modified::handler`].
    /// # Erorrs
    #[doc = edoc!(callerr(Self::handle), applyerr(StringModification))]
    Modified {
        /// The [`Self`] to use.
        handler: Box<Self>,
        /// The [`StringModification`] to apply.
        modification: StringModification
    },
    /// Searches the body until one of the [`HttpBodyExtractor::prefix`]es is found, then processes it and the remainder of the body per that [`HttpBodyExtractor`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource, 3))]
    ///
    /// If no extractors are given, returns the error [`HttpResponseHandlerError::NoExtractors`].
    ///
    #[doc = edoc!(callerr(std::io::Bytes::next, 3))]
    ///
    /// If no prefix is found, returns the error [`HttpResponseHandlerError::PrefixNotFound`].
    ///
    /// If the selected extractor's suffix isn't found, returns the error [`HttpResponseHandlerError::SuffixNotFound`].
    ///
    /// If [`Self::ExtractFromBody::limit`] bytes are read without finding a match, returns the error [`HttpResponseHandlerError::LimitReached`].
    ///
    #[doc = edoc!(callerr(String::try_from), applyerr(StringModification))]
    ExtractFromBody {
        /// The extractors to use.
        extractors: Vec<HttpBodyExtractor>,
        /// The maximum number of bytes to search.
        ///
        /// Defaults to 8MiB.
        #[serde(default = "default_limit", skip_serializing_if = "is_default_limit")]
        limit: usize
    },

    /// If the response has a 1xx status code, use [`Self::Require1xx::0`].
    /// # Errors
    /// If the response has a non-1xx status code, returns the error [`HttpResponseHandlerError::Required1xx`].
    Require1xx(Box<Self>),
    /// If the response has a 2xx status code, use [`Self::Require2xx::0`].
    /// # Errors
    /// If the response has a non-2xx status code, returns the error [`HttpResponseHandlerError::Required2xx`].
    Require2xx(Box<Self>),
    /// If the response has a 3xx status code, use [`Self::Require3xx::0`].
    /// # Errors
    /// If the response has a non-3xx status code, returns the error [`HttpResponseHandlerError::Required3xx`].
    Require3xx(Box<Self>),
    /// If the response has a 4xx status code, use [`Self::Require4xx::0`].
    /// # Errors
    /// If the response has a non-4xx status code, returns the error [`HttpResponseHandlerError::Required4xx`].
    Require4xx(Box<Self>),
    /// If the response has a 5xx status code, use [`Self::Require5xx::0`].
    /// # Errors
    /// If the response has a non-5xx status code, returns the error [`HttpResponseHandlerError::Required5xx`].
    Require5xx(Box<Self>),
}

/// An HTTP body extractor.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Suitability)]
pub struct HttpBodyExtractor {
    /// The prefix to search for.
    ///
    /// If [`None`], none of the body is skipped.
    pub prefix: StringSource,
    /// The suffix to search for.
    ///
    /// If [`None`], the entire body is read.
    ///
    /// Defaults to [`StringSource::None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub suffix: StringSource,
    /// If [`true`], remove the prefix from the result.
    ///
    /// Defaults to [`false`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub strip_prefix: FlagSource,
    /// If [`true`], remove the suffix from the result.
    ///
    /// Defaults to [`false`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub strip_suffix: FlagSource,
    /// The [`StringModification`] to use to parse the extracted segment.
    ///
    /// Defaults to [`StringModification::None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub parser: StringModification
}

/// Serder helper function.
const fn default_limit() -> usize {8 * 1024 * 1024}
/// Serder helper function.
const fn is_default_limit(x: &usize) -> bool {*x == default_limit()}

/// The enum of errors [`HttpResponseHandler::handle`] can return.
#[derive(Debug, Error)]
pub enum HttpResponseHandlerError {
    /// Returned when a [`HttpResponseHandler::Error`] is used.
    #[error("Explicit error: {0}")]
    ExplicitError(String),
    /// Returned when a [`HttpResponseHandler::AssertMatches`]'s assertion fails.
    #[error("AssertMatches failed: {0}")]
    AssertMatchesFailed(String),
    /// Returned when both [`HttpResponseHandler`]s in a [`HttpResponseHandler::TryElse`] return errors.
    #[error("Both HttpResponseHandlers in a HttpResponseHandler::TryElse returned errors.")]
    TryElseError {
        /// The error returned by [`HttpResponseHandler::TryElse::try`].
        try_error: Box<Self>,
        /// The error returned by [`HttpResponseHandler::TryElse::else`].
        else_error: Box<Self>
    },
    /// Returned when all [`HttpResponseHandler`]s in a [`HttpResponseHandler::FirstNotError`] error.
    #[error("All HttpResponseHandlers in a HttpResponseHandler::FirstNotError errored.")]
    FirstNotErrorErrors(Vec<Self>),

    /// Returned when a [`std::string::FromUtf8Error`] is encountered.
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    /// Returned when a [`std::str::Utf8Error`] is encountered.
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
    /// Returned when a [`std::io::Error`] is encountered.
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    /// Returned when a [`reqwest::Error`] is encountered.
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] Box<StringSourceError>),
    /// Returned when a call to [`StringSource::get`] returns [`None`] where it has to return [`Some`].
    #[error("A StringSource was None where it has to be Some.")]
    StringSourceIsNone,
    /// Returned when a [`StringModificationError`] is encountered.
    #[error(transparent)]
    StringModificationError(#[from] StringModificationError),
    /// Returned when a [`StringMatcherError`] is encountered.
    #[error(transparent)]
    StringMatcherError(#[from] StringMatcherError),
    /// Returnws when a [`FlagSourceError`] is encountered.
    #[error(transparent)]
    FlagSourceError(#[from] FlagSourceError),

    /// Returned whan the extracton prefix isn't found.
    #[error("The extraction prefix wasn't found.")]
    PrefixNotFound,
    /// Returned whan the extracton suffix isn't found.
    #[error("The extraction suffix wasn't found.")]
    SuffixNotFound,
    /// Returned when a limit of [`Self::LimitReached::0`] is reached.
    #[error("The limit of {0} bytes was reached.")]
    LimitReached(usize),
    /// Returned when [`HttpResponseHandler::ExtractFromBody`] is used with zero extractors.
    #[error("ExtractFromBody was used with zero extractors.")]
    NoExtractors,

    /// Returned when a 1xx status code is required but got [`Self::Required1xx::0`].
    #[error("A 1xx status code was required but got {0}.")]
    Required1xx(StatusCode),
    /// Returned when a 2xx status code is required but got [`Self::Required2xx::0`].
    #[error("A 2xx status code was required but got {0}.")]
    Required2xx(StatusCode),
    /// Returned when a 3xx status code is required but got [`Self::Required3xx::0`].
    #[error("A 3xx status code was required but got {0}.")]
    Required3xx(StatusCode),
    /// Returned when a 4xx status code is required but got [`Self::Required4xx::0`].
    #[error("A 4xx status code was required but got {0}.")]
    Required4xx(StatusCode),
    /// Returned when a 5xx status code is required but got [`Self::Required5xx::0`].
    #[error("A 5xx status code was required but got {0}.")]
    Required5xx(StatusCode)
}

impl From<StringSourceError> for HttpResponseHandlerError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

impl HttpResponseHandler {
    /// Gets the specified part of a [`reqwest::blocking::Response`].
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn handle<'j>(&'j self, response: &mut reqwest::blocking::Response, task_state: &TaskState<'j>) -> Result<Option<String>, HttpResponseHandlerError> {
        Ok(match self {
            Self::String(string) => get_option_string!(string, task_state),
            Self::None => None,
            Self::Error(msg) => Err(HttpResponseHandlerError::ExplicitError(msg.clone()))?,
            Self::TryElse{r#try, r#else} => match r#try.handle(response, task_state) {
                Ok(x) => x,
                Err(e1) => match r#else.handle(response, task_state) {
                    Ok(x) => x,
                    Err(e2) => Err(HttpResponseHandlerError::TryElseError {try_error: Box::new(e1), else_error: Box::new(e2)})?
                }
            },
            Self::TryThenModifiedOrElse {r#try, modification, r#else} => match r#try.handle(response, task_state) {
                Ok(x) => {
                    let mut ret = x.map(Cow::Owned);
                    modification.apply(&mut ret, task_state)?;
                    ret.map(Cow::into_owned)
                },
                Err(e1) => match r#else.handle(response, task_state) {
                    Ok(x) => x,
                    Err(e2) => Err(HttpResponseHandlerError::TryElseError {try_error: Box::new(e1), else_error: Box::new(e2)})?
                }
            },
            Self::FirstNotError(sources) => {
                let mut errors = Vec::new();
                for source in sources {
                    match source.handle(response, task_state) {
                        Ok(x) => return Ok(x),
                        Err(e) => errors.push(e)
                    }
                }
                Err(HttpResponseHandlerError::FirstNotErrorErrors(errors))?
            },
            Self::Debug(handler) => {
                let ret = handler.handle(response, task_state);
                eprintln!("=== HttpResponseHandler::Debug ===\nHandler: {handler:?}\nret: {ret:?}");
                ret?
            },
            Self::NoneTo {handler, if_none} => match handler.handle(response, task_state)? {
                Some(x) => Some(x),
                None    => if_none.handle(response, task_state)?
            },
            Self::EmptyToNone(handler) => {
                let x = handler.handle(response, task_state)?;
                if x == Some("".into()) {
                    None
                } else {
                    x
                }
            },
            Self::NoneToEmpty(handler) => Some(handler.handle(response, task_state)?.unwrap_or("".into())),
            Self::AssertMatches {handler, matcher, message} => {
                let ret = handler.handle(response, task_state)?;
                if matcher.check(ret.as_deref(), task_state)? {
                    ret
                } else {
                    Err(HttpResponseHandlerError::AssertMatchesFailed(message.get(task_state)?.unwrap_or_default().into()))?
                }
            },

            Self::Body => {
                let mut ret = String::new();
                response.read_to_string(&mut ret)?;
                Some(ret)
            },
            Self::Header(name) => match response.headers().get(get_str!(name, task_state, HttpResponseHandlerError)) {
                Some(value) => Some(str::from_utf8(value.as_bytes())?.into()),
                None => None
            },
            Self::Url => Some(response.url().as_str().to_string()),

            Self::Modified {handler, modification} => {
                let mut temp = handler.handle(response, task_state)?.map(Cow::Owned);
                modification.apply(&mut temp, task_state)?;
                temp.map(Cow::into_owned)
            },
            Self::ExtractFromBody {extractors, limit} => {
                let mut ret = Vec::new();
                let mut bytes = Read::bytes(response);
                let mut total_read = 0;

                let prefixes = extractors.iter().map(|x| x.prefix.get(task_state)).collect::<Result<Option<Vec<_>>, _>>()?.ok_or(HttpResponseHandlerError::StringSourceIsNone)?;

                let mut window = Vec::with_capacity(prefixes.iter().map(|x| x.len()).max().ok_or(HttpResponseHandlerError::NoExtractors)?);

                loop {
                    for (prefix, extractor) in prefixes.iter().zip(extractors) {
                        if window.ends_with(prefix.as_bytes()) {
                            if !extractor.strip_prefix.get(task_state)? {
                                ret = prefix.clone().into_owned().into();
                            }

                            let suffix = extractor.suffix.get(task_state)?;

                            match suffix {
                                None => for byte in bytes {
                                    if total_read == *limit {
                                        Err(HttpResponseHandlerError::LimitReached(total_read))?
                                    }
                                    ret.push(byte?);
                                    total_read += 1;
                                },
                                Some(suffix) => {
                                    let mut extend = Vec::new();
                                    while !extend.ends_with(suffix.as_bytes()) {
                                        if total_read == *limit {
                                            Err(HttpResponseHandlerError::LimitReached(total_read))?
                                        }
                                        extend.push(bytes.next().ok_or(HttpResponseHandlerError::SuffixNotFound)??);
                                        total_read += 1;
                                    }
                                    if extractor.strip_suffix.get(task_state)? {
                                        extend.truncate(extend.len() - suffix.len());
                                    }
                                    ret.extend(extend);
                                }
                            }

                            let mut temp = Some(Cow::Owned(String::try_from(ret)?));

                            extractor.parser.apply(&mut temp, task_state)?;

                            return Ok(temp.map(Cow::into_owned));
                        }
                    }
                    if total_read == *limit {
                        Err(HttpResponseHandlerError::LimitReached(total_read))?
                    }
                    if window.len() == window.capacity() {
                        window.remove(0);
                    }
                    window.push(bytes.next().ok_or(HttpResponseHandlerError::PrefixNotFound)??);
                    total_read += 1;
                }
            },

            Self::Require1xx(handler) => if response.status().is_informational() {handler.handle(response, task_state)?} else {Err(HttpResponseHandlerError::Required1xx(response.status()))?},
            Self::Require2xx(handler) => if response.status().is_success()       {handler.handle(response, task_state)?} else {Err(HttpResponseHandlerError::Required2xx(response.status()))?},
            Self::Require3xx(handler) => if response.status().is_redirection()   {handler.handle(response, task_state)?} else {Err(HttpResponseHandlerError::Required3xx(response.status()))?},
            Self::Require4xx(handler) => if response.status().is_client_error()  {handler.handle(response, task_state)?} else {Err(HttpResponseHandlerError::Required4xx(response.status()))?},
            Self::Require5xx(handler) => if response.status().is_server_error()  {handler.handle(response, task_state)?} else {Err(HttpResponseHandlerError::Required5xx(response.status()))?},
        })
    }
}
