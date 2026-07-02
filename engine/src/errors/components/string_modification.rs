//! [`StringModificationError`].

use crate::prelude::*;

/// The enum of errors [`StringModification::apply`] can return.
#[derive(Debug, Error)]
pub enum StringModificationError {
    /** [`ExplicitError`].                 **/ #[error(transparent)] ExplicitError                (#[from] ExplicitError                ),
    /** [`TryElseError`].                  **/ #[error(transparent)] TryElseError                 (#[from] Box<TryElseError<Self>>      ),
    /** [`FirstNotErrorErrors`].           **/ #[error(transparent)] FirstNotErrorErrors          (#[from] FirstNotErrorErrors<Self>    ),

    /** [`StringSourceError`].             **/ #[error(transparent)] StringSourceError            (#[from] Box<StringSourceError>       ),
    /** [`StringNotFound`].                **/ #[error(transparent)] StringNotFound               (#[from] StringNotFound               ),
    /** [`StringMatcherError`].            **/ #[error(transparent)] StringMatcherError           (#[from] Box<StringMatcherError>      ),
    /** [`StringLocationError`].           **/ #[error(transparent)] StringLocationError          (#[from] StringLocationError          ),
    /** [`RegexExpansionError`].           **/ #[error(transparent)] RegexExpansionError          (#[from] Box<RegexExpansionError>     ),

    /** [`std::str::Utf8Error`].           **/ #[error(transparent)] Utf8Error                    (#[from] std::str::Utf8Error          ),
    /** [`std::string::FromUtf8Error`].    **/ #[error(transparent)] FromUtf8Error                (#[from] std::string::FromUtf8Error   ),

    /** [`serde_json::Error`].             **/ #[error(transparent)] SerdeJsonError               (#[from] serde_json::Error            ),
    /** [`regex::Error`].                  **/ #[error(transparent)] RegexError                   (#[from] regex::Error                 ),
    /** [`base64::DecodeError`].           **/ #[error(transparent)] Base64DecodeError            (#[from] base64::DecodeError          ),
    /** [`GetJsStringLiteralPrefixError`]. **/ #[error(transparent)] GetJsStringLiteralPrefixError(#[from] GetJsStringLiteralPrefixError),
    /** [`UnescapeHtmlError`].             **/ #[error(transparent)] HtmlUnescapeHtmlError        (#[from] UnescapeHtmlError            ),
    /** [`GetHtmlAttributeError`].         **/ #[error(transparent)] HtmlGetAttributeValueError   (#[from] GetHtmlAttributeError        ),

    /** [`SubstringNotFound`].             **/ #[error(transparent)] SubstringNotFound            (#[from] SubstringNotFound            ),
    /** [`SubjectIsNone`].                 **/ #[error(transparent)] SubjectIsNone                (#[from] SubjectIsNone                ),

    /// Returned when a JSON value isn't found.
    #[error("The requested JSON value was not found.")]
    JsonValueNotFound,
    /// Returned when a JSON pointee isn't a string.
    #[error("The requested JSON pointee was not a string.")]
    JsonPointeeIsNotAString,

    /// Returned when the string being modified doesn't start with the specified prefix.
    #[error("The string being modified didn't start with the provided prefix. Maybe try `StringModification::TrimPrefix`?")]
    PrefixNotFound,
    /// Returned when the string being modified doesn't end with the specified suffix.
    #[error("The string being modified didn't end with the provided suffix. Maybe try `StringModification::TrimSuffix`?")]
    SuffixNotFound,

    /// Returned when the requested HTML attribute isn't found.
    #[error("The requested HTML attribute wasn't found.")]
    HtmlAttributeNotFound,
    /// Returned when the requested HTML attribute doesn't have a value.
    #[error("The requested HTML attribute doesn't have a value.")]
    HtmlAttributeHasNoValue,

    /// Returned when a [`regex::Regex`] doesn't find any matches in the string.
    #[error("The regex didn't find any matches in the string.")]
    RegexMatchNotFound,

    /** [`FunctionNotFound`].            **/ #[error(transparent)] FunctionNotFound(#[from] FunctionNotFound),
    /** [`NotInFunction`].               **/ #[error(transparent)] NotInFunction(#[from] NotInFunction),
    /** [`FunctionArgFunctionNotFound`]. **/ #[error(transparent)] FunctionArgFunctionNotFound(#[from] FunctionArgFunctionNotFound),

    /** [`ExternError`]. **/ #[error(transparent)] Extern(#[from] ExternError),
}

impl From<TryElseError<Self>> for StringModificationError {fn from(value: TryElseError<Self>) -> Self {Box::new(value).into()}}

impl From<StringSourceError  > for StringModificationError {fn from(value: StringSourceError  ) -> Self {Box::new(value).into()}}
impl From<StringMatcherError > for StringModificationError {fn from(value: StringMatcherError ) -> Self {Box::new(value).into()}}
impl From<RegexExpansionError> for StringModificationError {fn from(value: RegexExpansionError) -> Self {Box::new(value).into()}}
