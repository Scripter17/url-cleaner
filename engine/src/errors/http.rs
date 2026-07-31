//! HTTP.

use http::StatusCode;

use crate::prelude::*;

/// Returned when attempting to use HTTP in a [`Job`] without an [`HttpClient`].
#[derive(Debug, Error)]
#[error("Attempted to use HTTP in a Job without an HttpClient")]
pub struct NoHttpClient;

/// [`HttpClient::do`].
#[derive(Debug, Error)]
pub enum DoHttpRequestError {
    /** [`reqwest::Error`].           **/ #[error(transparent)] ReqwestError            (#[from] reqwest::Error          ),
    /** [`StringSourceError`].        **/ #[error(transparent)] StringSourceError       (#[from] StringSourceError       ),
    /** [`HttpRequestSourceError`].   **/ #[error(transparent)] HttpRequestSourceError  (#[from] HttpRequestSourceError  ),
    /** [`HttpResponseHandlerError`]. **/ #[error(transparent)] HttpResponseHandlerError(#[from] HttpResponseHandlerError),
}

/// [`HttpRequestSource::get`].
#[derive(Debug, Error)]
pub enum HttpRequestSourceError {
    /** [`reqwest::Error`].                      **/ #[error(transparent)] RequestError        (#[from] reqwest::Error                     ),
    /** [`StringSourceError`].                   **/ #[error(transparent)] StringSourceError   (#[from] StringSourceError                  ),
    /** [`StringNotFound`].                      **/ #[error(transparent)] StringNotFound      (#[from] StringNotFound                     ),
    /** [`MapSourceError`].                      **/ #[error(transparent)] MapSourceError      (#[from] MapSourceError                     ),
    /** [`http::method::InvalidMethod`].         **/ #[error(transparent)] HttpInvalidMethod   (#[from] http::method::InvalidMethod        ),
    /** [`reqwest::header::InvalidHeaderName`].  **/ #[error(transparent)] InvalidHeaderName   (#[from] reqwest::header::InvalidHeaderName ),
    /** [`reqwest::header::InvalidHeaderValue`]. **/ #[error(transparent)] InvalidHeaderValue  (#[from] reqwest::header::InvalidHeaderValue),
    /** [`url::ParseError`]                      **/ #[error(transparent)] UrlParseError       (#[from] url::ParseError                    ),
}

/// [`HttpResponseHandler::handle`].
#[derive(Debug, Error)]
pub enum HttpResponseHandlerError {
    /** [`TryElseError`].               **/ #[error(transparent)] TryElseError           (#[from] Box<TryElseError<Self>>   ),

    /** [`std::string::FromUtf8Error`]. **/ #[error(transparent)] FromUtf8Error          (#[from] std::string::FromUtf8Error),
    /** [`std::str::Utf8Error`].        **/ #[error(transparent)] Utf8Error              (#[from] std::str::Utf8Error       ),
    /** [`reqwest::Error`].             **/ #[error(transparent)] ReqwestError           (#[from] reqwest::Error            ),
    /** [`StringSourceError`].          **/ #[error(transparent)] StringSourceError      (#[from] StringSourceError         ),
    /** [`StringNotFound`].             **/ #[error(transparent)] StringNotFound         (#[from] StringNotFound            ),
    /** [`FlagSourceError`].            **/ #[error(transparent)] FlagSourceError        (#[from] FlagSourceError           ),
    /** [`StringModificationError`].    **/ #[error(transparent)] StringModificationError(#[from] StringModificationError   ),

    /** Returned when no [`BodyExtractor::prefix`] isn't found in the entire body.     **/ #[error("No BodyExtractor::prefix was found in the entire body."       )] PrefixNotFound,
    /** Returned when the [`BodyExtractor::suffix`] isn't found in the entire body.    **/ #[error("The BodyExtractor::suffix wasn't found in the entire body."   )] SuffixNotFound,

    /** Returned when no [`BodyExtractor::prefix`] is found within the read limit.     **/ #[error("No BodyExtractor::prefix was found within the read limit."    )] PrefixNotFoundWithinLimit,
    /** Returned when the [`BodyExtractor::suffix`] isn't found within the read limit. **/ #[error("The BodyExtractor::suffix wasn't found within the read limit.")] SuffixNotFoundWithinLimit,

    /** Returned when a 1xx status code is required but got [`Self::Required1xx::0`]. **/ #[error("A 1xx status code was required but got {0}.")] Required1xx(StatusCode),
    /** Returned when a 2xx status code is required but got [`Self::Required2xx::0`]. **/ #[error("A 2xx status code was required but got {0}.")] Required2xx(StatusCode),
    /** Returned when a 3xx status code is required but got [`Self::Required3xx::0`]. **/ #[error("A 3xx status code was required but got {0}.")] Required3xx(StatusCode),
    /** Returned when a 4xx status code is required but got [`Self::Required4xx::0`]. **/ #[error("A 4xx status code was required but got {0}.")] Required4xx(StatusCode),
    /** Returned when a 5xx status code is required but got [`Self::Required5xx::0`]. **/ #[error("A 5xx status code was required but got {0}.")] Required5xx(StatusCode),
}

impl From<TryElseError<Self>> for HttpResponseHandlerError {fn from(value: TryElseError<Self>) -> Self {Box::new(value).into()}}
