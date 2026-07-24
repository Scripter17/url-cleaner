//! [`HttpResponseHandler`].

use crate::prelude::*;

/// How to turn an HTTP response into a string to use.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub enum HttpResponseHandler {
    /// If the status is 100-199, the inner [`Self`].
    /// # Errors
    /// If the status is not 100-199, returns the error [`HttpResponseHandlerError::Required1xx`].
    Require1xx(Box<Self>),
    /// If the status is 200-299, the inner [`Self`].
    /// # Errors
    /// If the status is not 100-199, returns the error [`HttpResponseHandlerError::Required2xx`].
    Require2xx(Box<Self>),
    /// If the status is 300-399, the inner [`Self`].
    /// # Errors
    /// If the status is not 100-199, returns the error [`HttpResponseHandlerError::Required3xx`].
    Require3xx(Box<Self>),
    /// If the status is 400-499, the inner [`Self`].
    /// # Errors
    /// If the status is not 100-199, returns the error [`HttpResponseHandlerError::Required4xx`].
    Require4xx(Box<Self>),
    /// If the status is 500-599, the inner [`Self`].
    /// # Errors
    /// If the status is not 100-199, returns the error [`HttpResponseHandlerError::Required5xx`].
    Require5xx(Box<Self>),

    /// [`Self::TryElse::try`], or [`Self::TryElse::else`] if it returns [`Err`].
    TryElse {
        /// The try.
        r#try: Box<Self>,
        /// The else.
        r#else: Box<Self>
    },
    /// [`Self::NoneTo::handler`], or, if it's [`None`], [`Self::NoneTo::if_none`].
    NoneTo {
        /// The try.
        handler: Box<Self>,
        /// The else.
        if_none: Box<Self>,
    },
    /// [`Self::Modified::handler`] + [`Self::Modified::modification`].
    Modified {
        /// The [`Self`].
        handler: Box<Self>,
        /// The [`StringModification`].
        modification: StringModification
    },

    /// The body.
    ///
    /// The default.
    #[default]
    Body,
    /// The specified header.
    Header(StringSource),
    /// The URL.
    Url,

    /// Everything between the first found [`BodyExtractor::prefix`] and its [`BodyExtractor::suffix`], optionally including the prefix and/or suffix.
    /// # Errors
    /// If no prefix is found within [`Self::ExtractFromBody::limit`], returns the error [`HttpResponseHandlerError::PrefixNotFoundWithinLimit`].
    ///
    /// If the suffix is found within [`Self::ExtractFromBody::limit`], returns the error [`HttpResponseHandlerError::SuffixNotFoundWithinLimit`].
    ExtractFromBody {
        /// The [`BodyExtractor`]s.
        extractors: Vec<BodyExtractor>,
        /// The max amount of bytes to read.
        ///
        /// Defaults to 8MiB.
        #[serde(default = "default_limit", skip_serializing_if = "is_default_limit")]
        limit: usize
    },
}

/// Serde helper function.
const fn default_limit() -> usize {8 * 1024 * 1024}
/// Serde helper function.
const fn is_default_limit(x: &usize) -> bool {*x == default_limit()}

/// An HTTP body extractor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub struct BodyExtractor {
    /// The prefix to look for.
    pub prefix      : StringSource,
    /// The suffix to look for.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub suffix      : StringSource,
    /// If the prefix should be removed.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub strip_prefix: FlagSource,
    /// If the suffix should be removed.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub strip_suffix: FlagSource,
    /// The [`StringModification`] to apply to the extracted value.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub parser      : StringModification,
}

impl HttpResponseHandler {
    /// Handle the response.
    /// # Errors
    /// See each variant of [`Self`] for details.
    pub async fn handle<'j: 't, 't>(&'j self, task_state: &'t TaskState<'j>, args: Option<&'j FunctionArgs>, response: &mut reqwest::Response) -> Result<Option<Cow<'t, str>>, HttpResponseHandlerError> {
        Ok(match self {
            Self::NoneTo {handler, if_none} => match Box::pin(handler.handle(task_state, args, response)).await? {
                Some(x) => Some(x),
                None    => Box::pin(if_none.handle(task_state, args, response)).await?,
            },
            Self::Modified {handler, modification} => {
                let mut temp = Box::pin(handler.handle(task_state, args, response)).await?;
                modification.apply(task_state, args, &mut temp)?;
                temp
            },
            Self::TryElse {r#try, r#else} => match Box::pin(r#try.handle(task_state, args, response)).await {
                Ok(x) => x,
                Err(try_error) => match Box::pin(r#else.handle(task_state, args, response)).await {
                    Ok(x) => x,
                    Err(else_error) => Err(TryElseError {try_error, else_error})?
                }
            },

            Self::Require1xx(handler) => if response.status().is_informational() {Box::pin(handler.handle(task_state, args, response)).await?} else {Err(HttpResponseHandlerError::Required1xx(response.status()))?},
            Self::Require2xx(handler) => if response.status().is_success      () {Box::pin(handler.handle(task_state, args, response)).await?} else {Err(HttpResponseHandlerError::Required2xx(response.status()))?},
            Self::Require3xx(handler) => if response.status().is_redirection  () {Box::pin(handler.handle(task_state, args, response)).await?} else {Err(HttpResponseHandlerError::Required3xx(response.status()))?},
            Self::Require4xx(handler) => if response.status().is_client_error () {Box::pin(handler.handle(task_state, args, response)).await?} else {Err(HttpResponseHandlerError::Required4xx(response.status()))?},
            Self::Require5xx(handler) => if response.status().is_server_error () {Box::pin(handler.handle(task_state, args, response)).await?} else {Err(HttpResponseHandlerError::Required5xx(response.status()))?},

            Self::Body => {
                let mut ret = Vec::new();
                while let Some(chunk) = response.chunk().await? {
                    ret.extend_from_slice(&chunk);
                }
                Some(String::try_from(ret)?.into())
            },
            Self::Header(name) => match response.headers().get(get!(&name)) {
                Some(value) => Some(str::from_utf8(value.as_bytes())?.to_string().into()),
                None => None
            },
            Self::Url => Some(response.url().to_string().into()),

            Self::ExtractFromBody {extractors, limit} => {
                let prefixes = extractors.iter().map(|x| x.prefix.get_some(task_state, args)).collect::<Result<Result<Vec<_>, _>, _>>()??;

                let mut buf = Vec::with_capacity(prefixes.iter().map(|x| x.len()).max().ok_or(HttpResponseHandlerError::NoExtractors)?);
                let mut bytes = bytes::Bytes::new().into_iter();
                let mut read = 0;

                loop {
                    for (prefix, extractor) in prefixes.iter().zip(extractors) {
                        if buf.ends_with(prefix.as_bytes()) {
                            match get!(?extractor.strip_prefix) {
                                false => {buf.drain(..buf.len() - prefix.len());},
                                true  =>  buf.clear()
                            }

                            let middle_start = buf.len();

                            let suffix = get!(extractor.suffix);

                            loop {
                                for byte in bytes {
                                    if buf[middle_start..].ends_with(suffix.as_bytes()) {
                                        if get!(?extractor.strip_suffix) {
                                            buf.truncate(buf.len() - suffix.len());
                                        }

                                        let mut ret = Some(String::try_from(buf)?.into());

                                        extractor.parser.apply(task_state, args, &mut ret)?;

                                        return Ok(ret);
                                    }
                                    buf.push(byte);
                                }

                                if read >= *limit {
                                    Err(HttpResponseHandlerError::SuffixNotFoundWithinLimit)?;
                                }

                                bytes = response.chunk().await?.ok_or(HttpResponseHandlerError::SuffixNotFound)?.into_iter();
                                read += bytes.len();
                            }
                        }
                    }

                    if buf.len() == buf.capacity() {
                        buf.remove(0);
                    }

                    match bytes.next() {
                        Some(byte) => buf.push(byte),
                        None => {
                            if read >= *limit {
                                Err(HttpResponseHandlerError::PrefixNotFoundWithinLimit)?;
                            }

                            bytes = response.chunk().await?.ok_or(HttpResponseHandlerError::PrefixNotFound)?.into_iter();
                            read += bytes.len();
                        }
                    }
                }
            },
        })
    }
}
