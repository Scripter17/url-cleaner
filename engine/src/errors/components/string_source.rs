//! [`StringSourceError`].

use crate::prelude::*;

/// The enum of errors [`StringSource::get`] can return.
#[derive(Debug, Error)]
pub enum StringSourceError {
    /** [`ExplicitError`].               **/                           #[error(transparent)] ExplicitError              (#[from] ExplicitError                ),
    /** [`AssertError`].                 **/                           #[error(transparent)] AssertError                (#[from] AssertError                  ),
    /** [`TryElseError`].                **/                           #[error(transparent)] TryElseError               (#[from] Box<TryElseError<Self>>      ),
    /** [`FirstNotErrorErrors`].         **/                           #[error(transparent)] FirstNotErrorErrors        (#[from] FirstNotErrorErrors<Self>    ),

    /** [`StringNotFound`].              **/                           #[error(transparent)] StringNotFound             (#[from] StringNotFound               ),
    /** [`StringModificationError`].     **/                           #[error(transparent)] StringModificationError    (#[from] StringModificationError      ),
    /** [`Box<StringMatcherError>`].     **/                           #[error(transparent)] StringMatcherError         (#[from] Box<StringMatcherError>      ),
    /** [`PartitioningSourceError`].     **/                           #[error(transparent)] PartitioningSourceError    (#[from] Box<PartitioningSourceError> ),
    /** [`RegexExpansionError`].         **/                           #[error(transparent)] RegexExpansionError        (#[from] Box<RegexExpansionError>     ),

    /** [`url::ParseError`].             **/                           #[error(transparent)] UrlParseError              (#[from] url::ParseError              ),
    /** [`PartitioningNotFound`]         **/                           #[error(transparent)] PartitioningNotFound       (#[from] PartitioningNotFound         ),
    /** [`MapNotFound`]                  **/                           #[error(transparent)] MapNotFound                (#[from] MapNotFound                  ),
    /** [`FlagSourceError`].             **/                           #[error(transparent)] FlagSourceError            (#[from] FlagSourceError              ),
    /** [`VarSourceError`].              **/                           #[error(transparent)] VarSourceError             (#[from] VarSourceError               ),
    /** [`MapSourceError`].              **/                           #[error(transparent)] MapSourceError             (#[from] Box<MapSourceError>          ),
    /** [`regex::Error`].                **/                           #[error(transparent)] RegexError                 (#[from] regex::Error                 ),

    /** [`reqwest::Error`].              **/ #[cfg(feature = "http" )] #[error(transparent)] ReqwestError               (#[from] reqwest::Error               ),
    /** [`HttpRequestSourceError`].      **/ #[cfg(feature = "http" )] #[error(transparent)] HttpRequestSourceError     (#[from] Box<HttpRequestSourceError>  ),
    /** [`HttpResponseHandlerError`].    **/ #[cfg(feature = "http" )] #[error(transparent)] HttpResponseHandlerError   (#[from] Box<HttpResponseHandlerError>),

    /** [`ReadFromCacheError`].          **/ #[cfg(feature = "cache")] #[error(transparent)] ReadFromCacheError         (#[from] ReadFromCacheError           ),
    /** [`WriteToCacheError`].           **/ #[cfg(feature = "cache")] #[error(transparent)] WriteToCacheError          (#[from] WriteToCacheError            ),

    /** [`FunctionNotFound`].            **/                           #[error(transparent)] FunctionNotFound           (#[from] FunctionNotFound             ),
    /** [`NotInFunction`].               **/                           #[error(transparent)] NotInFunction              (#[from] NotInFunction                ),
    /** [`FunctionArgFunctionNotFound`]. **/                           #[error(transparent)] FunctionArgFunctionNotFound(#[from] FunctionArgFunctionNotFound  ),

    /** [`ExternError`]. **/ #[error(transparent)] Extern(#[from] ExternError),
}

impl From<TryElseError<Self>> for StringSourceError {fn from(value: TryElseError<Self>) -> Self {Box::new(value).into()}}

impl From<StringMatcherError     > for StringSourceError {fn from(value: StringMatcherError     ) -> Self {Box::new(value).into()}}
impl From<PartitioningSourceError> for StringSourceError {fn from(value: PartitioningSourceError) -> Self {Box::new(value).into()}}
impl From<RegexExpansionError    > for StringSourceError {fn from(value: RegexExpansionError    ) -> Self {Box::new(value).into()}}
impl From<MapSourceError         > for StringSourceError {fn from(value: MapSourceError         ) -> Self {Box::new(value).into()}}

#[cfg(feature = "http")] impl From<HttpRequestSourceError  > for StringSourceError {fn from(value: HttpRequestSourceError  ) -> Self {Box::new(value).into()}}
#[cfg(feature = "http")] impl From<HttpResponseHandlerError> for StringSourceError {fn from(value: HttpResponseHandlerError) -> Self {Box::new(value).into()}}
