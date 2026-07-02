//! Parsing.

use crate::prelude::*;

/// The enum of errors [`get_html_attribute`] can return.
#[derive(Debug, Error)]
pub enum GetHtmlAttributeError {
    /// A syntax error.
    #[error("Syntax error: {index}, {last_bite:?}, {kind:?}")]
    Syntax {
        /// The index of the input string the error happened.
        index: usize,
        /// The state the previous character put the DFA in.
        last_bite: GetHtmlAttributeLastBite,
        /// The error kind.
        kind: GetHtmlAttributeSyntaxErrorKind
    },
    /// Returned when an [`UnescapeHtmlError`] is encountered.
    #[error(transparent)]
    UnescapeHtmlError(#[from] UnescapeHtmlError),
    /// Returned when the HTML tag isn't finished.
    #[error("The HTML tag wasn't finished.")]
    UnfinishedTag
}

/// [`get_html_attribute`].
#[derive(Debug, Error, Clone, Copy)]
pub enum GetHtmlAttributeSyntaxErrorKind {
    /// The [input-doesnt-start-with-html-element](https://html.spec.whatwg.org/multipage/parsing.html#parse-errors) error.
    #[error("Input doesn't start with an HTML element.")]
    InputDoesntStartWithHtmlElement,
    /// The [unexpected-question-mark-instead-of-tagname](https://html.spec.whatwg.org/multipage/parsing.html#parse-errors) error.
    #[error("Unexpected question mark instead of tag name.")]
    UnexpectedQuestionMarkInsteadOfTagName,
    /// The [invalid-start-of-tag-name](https://html.spec.whatwg.org/multipage/parsing.html#parse-errors) error.
    #[error("Invalid start of tag name.")]
    InvalidStartOfTagName,
    /// The [unexpected-null-character](https://html.spec.whatwg.org/multipage/parsing.html#parse-errors) error.
    #[error("Unexpected null character.")]
    UnexpectedNullCharacter,
    /// The [unexpected-solidus-in-tag](https://html.spec.whatwg.org/multipage/parsing.html#parse-errors) error.
    #[error("Unexpected solidus in tag.")]
    UnexpectedSolidusInTag,
    /// The [unexpected-equals-sign-before-attribute-name](https://html.spec.whatwg.org/multipage/parsing.html#parse-errors) error.
    #[error("Unexpected equals sign before attribute name.")]
    UnexpectedEqualsSignBeforeAttributeName,
    /// The [unexpected-character-in-attribute-name](https://html.spec.whatwg.org/multipage/parsing.html#parse-errors) error.
    #[error("Unexpected character in attribute name.")]
    UnexpectedCharacterInAttributeName,
    /// The [missing-attribute-value](https://html.spec.whatwg.org/multipage/parsing.html#parse-errors) error.
    #[error("Missing attribute value.")]
    MissingAttributeValue,
    /// The [missing-whitespace-between-attributes](https://html.spec.whatwg.org/multipage/parsing.html#parse-errors) error.
    #[error("Missing whitespace between attributes.")]
    MissingWhitespaceBetweenAttributes
}

/// [`get_html_char_ref`].
#[derive(Debug, Error)]
#[error("Unknown character reference.")]
pub enum GetHtmlCharRefError {
    /// Unknown char name.
    #[error("Unknown char name.")]
    UnknownCharName,
    /// Invalid dec.
    #[error("Invalid dec.")]
    InvalidDec,
    /// Invalid hex.
    #[error("Invalid hex.")]
    InvalidHex,
    /// Dec overflow.
    #[error("Dec overflow.")]
    DecOverflow,
    /// Hex overflow.
    #[error("Hex overflow.")]
    HexOverflow,
    /// Invalid char code.
    #[error("Invalid char code.")]
    InvalidCharCode(u32),
    /// Disallowed char code.
    #[error("Disallowed char code.")]
    DisallowedCharCode(char)
}

/// [`unescape_html`].
#[derive(Debug, Error)]
pub enum UnescapeHtmlError {
    /// Returned when a syntax error is encountered.
    #[error("Syntax error.")]
    SyntaxError,
    /// Returned when an [`GetHtmlCharRefError`] is encountered.
    #[error(transparent)]
    GetHtmlCharRefError(#[from] GetHtmlCharRefError)
}

/// [`get_js_string_literal_prefix`].
#[derive(Debug, Error)]
pub enum GetJsStringLiteralPrefixError {
    /// Returned when a syntax error is encountered.
    #[error("A syntax error was encountered.")]
    SyntaxError,
    /// Returned when an invalid codepoint is encountered.
    #[error("An invalid codepoint was encountered.")]
    InvalidCodepoint
}
