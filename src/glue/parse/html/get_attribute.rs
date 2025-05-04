//! Gets attributes from HTML elements.

use thiserror::Error;

use super::*;

/// The enum of errors that can be encountered when failing to parse an HTML element.
#[derive(Debug, Error, Clone, Copy)]
pub enum GAVSyntaxErrorKind {
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

/// The enum of errors [`get_attribute_value`] can return.
#[derive(Debug, Error)]
pub enum GAVError {
    /// A syntax error.
    #[error("Syntax error: {index}, {last_bite:?}, {kind:?}")]
    Syntax {
        /// The index of the input string the error happened.
        index: usize,
        /// The state the previous character put the DFA in.
        last_bite: GAVLastBite,
        /// The error kind.
        kind: GAVSyntaxErrorKind
    },
    /// Returned when an [`UnescapeTextError`] is encountered.
    #[error(transparent)]
    UnescapeTextError(#[from] UnescapeTextError),
    /// Returned when the HTML tag isn't finished.
    #[error("The HTML tag wasn't finished.")]
    UnfinishedTag
}

/// The states the DFA in [`get_attribute_value`] can be in.
#[derive(Debug, Clone, Copy)]
pub enum GAVLastBite {
    /// The [Data](https://html.spec.whatwg.org/multipage/parsing.html#data-state) state.
    Data,
    /// The [Tag open](https://html.spec.whatwg.org/multipage/parsing.html#tag-open-state) state.
    TagOpen,
    /// The [Tag name](https://html.spec.whatwg.org/multipage/parsing.html#tag-name-state) state.
    TagName,
    /// The [Self-closing start tag](https://html.spec.whatwg.org/multipage/parsing.html#self-closing-start-tag-state) state.
    SelfClosingStartTag,
    /// The [Before attribute name](https://html.spec.whatwg.org/multipage/parsing.html#before-attribute-name-state) state.
    BeforeAttributeName,
    /// The [Attribute name](https://html.spec.whatwg.org/multipage/parsing.html#attribute-name-state) state.
    AttributeName,
    /// The [After attribute name](https://html.spec.whatwg.org/multipage/parsing.html#after-attribute-name-state) state.
    AfterAttributeName,
    /// The [Before attribute value](https://html.spec.whatwg.org/multipage/parsing.html#before-attribute-value-state) state.
    BeforeAttributeValue,
    /// The [Attribute value (double-quoted)](https://html.spec.whatwg.org/multipage/parsing.html#attribute-value-(double-quoted)-state) state.
    AttributeValueDoubleQuoted,
    /// The [Attribute value (single-quoted)](https://html.spec.whatwg.org/multipage/parsing.html#attribute-value-(single-quoted)-state) state.
    AttributeValueSingleQuoted,
    /// The [Attribute value (unquoted)](https://html.spec.whatwg.org/multipage/parsing.html#attribute-value-(unquoted)-state) state.
    AttributeValueUnquoted,
    /// The [After attribute value (quoted)](https://html.spec.whatwg.org/multipage/parsing.html#after-attribute-value-(quoted)-state) state.
    AfterAttributeValueQuoted,
    /// The done state.
    Done
}

/// The current state of the [`get_attribute_value`] DFA.
#[derive(Debug)]
struct GAVState<'a> {
    /// The input.
    input: &'a str,
    /// The name of the attribute to search for.
    name: &'a str,
    /// The state the last bite put the DFA in.
    last_bite: GAVLastBite,
    /// The return value.
    ret: Option<Option<&'a str>>,
    /// The location of the start of the most recent attribute's name.
    attr_name_start: usize,
    /// The location of the end of the most recent attribute's name.
    attr_name_end: usize,
    /// The location of the start of the most recent attribute's value.
    attr_value_start: usize
}

/// Shorthand.
type LB = GAVLastBite;
/// Shorthand.
type EK = GAVSyntaxErrorKind;

/// Take a string that starts with an HTML element and get the value of the last attribute with the specified name.
/// # Errors
/// It's complicated, but TL;DR if the spec says an error happens (even if it can recover), that error is returned.
///
/// If the call to [`unescape_text`] returns an error, that error is returned.
///
/// If the input doesn't start with a complete HTML start tag, returns the error [`GAVError::UnfinishedTag`].
/// # Examples
/// ```
/// use url_cleaner::glue::*;
///
/// assert_eq!(parse::html::get_attribute_value("<a href='aaa'>"       , "href").unwrap(), Some(Some("aaa" .to_string())));
/// assert_eq!(parse::html::get_attribute_value("<a href='a&quot;a'>"  , "href").unwrap(), Some(Some("a\"a".to_string())));
/// assert_eq!(parse::html::get_attribute_value("<a href=\"aaa\">"     , "href").unwrap(), Some(Some("aaa" .to_string())));
/// assert_eq!(parse::html::get_attribute_value("<a href=\"a&quot;a\">", "href").unwrap(), Some(Some("a\"a".to_string())));
/// assert_eq!(parse::html::get_attribute_value("<a href=aaa>"         , "href").unwrap(), Some(Some("aaa" .to_string())));
/// assert_eq!(parse::html::get_attribute_value("<a href=a&quot;a>"    , "href").unwrap(), Some(Some("a\"a".to_string())));
///
/// assert_eq!(parse::html::get_attribute_value("<a href='aaa'        >", "href").unwrap(), Some(Some("aaa" .to_string())));
/// assert_eq!(parse::html::get_attribute_value("<a href='a&quot;a'   >", "href").unwrap(), Some(Some("a\"a".to_string())));
/// assert_eq!(parse::html::get_attribute_value("<a href=\"aaa\"      >", "href").unwrap(), Some(Some("aaa" .to_string())));
/// assert_eq!(parse::html::get_attribute_value("<a href=\"a&quot;a\" >", "href").unwrap(), Some(Some("a\"a".to_string())));
/// assert_eq!(parse::html::get_attribute_value("<a href=aaa          >", "href").unwrap(), Some(Some("aaa" .to_string())));
/// assert_eq!(parse::html::get_attribute_value("<a href=a&quot;a     >", "href").unwrap(), Some(Some("a\"a".to_string())));
///
/// assert_eq!(parse::html::get_attribute_value("<a href=b href='aaa'        >", "href").unwrap(), Some(Some("aaa" .to_string())));
/// assert_eq!(parse::html::get_attribute_value("<a href=b href='a&quot;a'   >", "href").unwrap(), Some(Some("a\"a".to_string())));
/// assert_eq!(parse::html::get_attribute_value("<a href=b href=\"aaa\"      >", "href").unwrap(), Some(Some("aaa" .to_string())));
/// assert_eq!(parse::html::get_attribute_value("<a href=b href=\"a&quot;a\" >", "href").unwrap(), Some(Some("a\"a".to_string())));
/// assert_eq!(parse::html::get_attribute_value("<a href=b href=aaa          >", "href").unwrap(), Some(Some("aaa" .to_string())));
/// assert_eq!(parse::html::get_attribute_value("<a href=b href=a&quot;a     >", "href").unwrap(), Some(Some("a\"a".to_string())));
///
/// assert_eq!(parse::html::get_attribute_value("<a>", "href").unwrap(), None, "1");
///
/// assert_eq!(parse::html::get_attribute_value("<a href>"                           , "href").unwrap(), Some(None));
///
/// assert_eq!(parse::html::get_attribute_value("<a href href=\"1\">"                , "href").unwrap(), Some(Some("1".to_string())));
/// assert_eq!(parse::html::get_attribute_value("<a href href=\"1\" href>"           , "href").unwrap(), Some(None));
/// assert_eq!(parse::html::get_attribute_value("<a href href=\"1\" href href=\"2\">", "href").unwrap(), Some(Some("2".to_string())));
///
/// assert_eq!(parse::html::get_attribute_value("<a href=\"1\" href>"                , "href").unwrap(), Some(None));
/// assert_eq!(parse::html::get_attribute_value("<a href=\"1\" href href=\"2\">"     , "href").unwrap(), Some(Some("2".to_string())));
/// assert_eq!(parse::html::get_attribute_value("<a href=\"1\" href href=\"2\" href>", "href").unwrap(), Some(None));
///
/// assert_eq!(parse::html::get_attribute_value("<a href=\"1\">stuff"              , "href").unwrap(), Some(Some("1".to_string())));
/// assert_eq!(parse::html::get_attribute_value("<a href=\"1\"><a href=\"2\">"     , "href").unwrap(), Some(Some("1".to_string())));
/// assert_eq!(parse::html::get_attribute_value("<a href=\"1\">stuff<a href=\"2\">", "href").unwrap(), Some(Some("1".to_string())));
/// assert_eq!(parse::html::get_attribute_value("<a href=\"1\">href=\"2\""         , "href").unwrap(), Some(Some("1".to_string())));
/// ```
pub fn get_attribute_value<'a>(input: &'a str, name: &'a str) -> Result<Option<Option<String>>, GAVError> {
    let mut state = GAVState {
        input,
        name,
        last_bite: LB::Data,
        ret: None,
        attr_name_start: 0,
        attr_name_end: 0,
        attr_value_start: 0
    };

    for (i, c) in input.chars().enumerate() {
        if let Err(e) = munch(&mut state, i, c) {
            return Err(GAVError::Syntax {
                index: i,
                last_bite: state.last_bite,
                kind: e
            });
        }
        if matches!(state.last_bite, LB::Done) {
            return Ok(match state.ret {
                Some(Some(value)) => Some(Some(unescape_text(value)?)),
                Some(None) => Some(None),
                None => None
            });
        }
    }

    Err(GAVError::UnfinishedTag)
}

/// Advance the state of the [`get_attribute_name`] DFA.
fn munch(state: &mut GAVState, i: usize, c: char) -> Result<(), GAVSyntaxErrorKind> {
    match (state.last_bite, c) {
        (LB::Data, '<') => {state.last_bite = LB::TagOpen;},
        (LB::Data, _  ) => Err(EK::InputDoesntStartWithHtmlElement)?,


        (LB::TagOpen, 'a'..='z' | 'A'..='Z') => {state.last_bite = LB::TagName;},
        (LB::TagOpen, '?'                  ) => Err(EK::UnexpectedQuestionMarkInsteadOfTagName)?,
        (LB::TagOpen, _                    ) => Err(EK::InvalidStartOfTagName)?,


        (LB::TagName, '\t' | '\r' | '\n' | ' ') => {state.last_bite = LB::BeforeAttributeName;},
        (LB::TagName, '/'                     ) => {state.last_bite = LB::SelfClosingStartTag;},
        (LB::TagName, '\0'                    ) => Err(EK::UnexpectedNullCharacter)?,
        (LB::TagName, '>'                     ) => {state.last_bite = LB::Done;},
        (LB::TagName, _                       ) => {},


        (LB::SelfClosingStartTag, '>') => {state.last_bite = LB::Done;},
        (LB::SelfClosingStartTag, _  ) => Err(EK::UnexpectedSolidusInTag)?,


        (LB::BeforeAttributeName, '\t' | '\r' | '\n' | ' ') => {},
        (LB::BeforeAttributeName, '/' | '>'               ) => {state.last_bite = LB::AfterAttributeName; munch(state, i, c)?;},
        (LB::BeforeAttributeName, '='                     ) => Err(EK::UnexpectedEqualsSignBeforeAttributeName)?,
        (LB::BeforeAttributeName, _                       ) => {state.last_bite = LB::AttributeName; state.attr_name_start = i; munch(state, i, c)?;},


        (LB::AttributeName, '\t' | '\r' | '\n' | ' ' | '/' | '>' ) => {state.last_bite = LB::AfterAttributeName  ; state.attr_name_end = i; if &state.input[state.attr_name_start..state.attr_name_end] == state.name {state.ret = Some(None);} else {state.ret = None;} munch(state, i, c)?;},
        (LB::AttributeName, '='                                  ) => {state.last_bite = LB::BeforeAttributeValue; state.attr_name_end = i;},
        (LB::AttributeName, '\0' | '"' | '\'' | '<'              ) => Err(EK::UnexpectedCharacterInAttributeName)?,
        (LB::AttributeName, _                                    ) => {},


        (LB::AfterAttributeName, '\t' | '\r' | '\n' | ' ') => {},
        (LB::AfterAttributeName, '/'                     ) => {state.last_bite = LB::SelfClosingStartTag ;},
        (LB::AfterAttributeName, '='                     ) => {state.last_bite = LB::BeforeAttributeValue;},
        (LB::AfterAttributeName, '>'                     ) => {state.last_bite = LB::Done; if &state.input[state.attr_name_start..i] == state.name {state.ret = Some(None);}},
        (LB::AfterAttributeName, _                       ) => {state.last_bite = LB::AttributeName; state.attr_name_start = i; munch(state, i, c)?;},


        (LB::BeforeAttributeValue, '\t' | '\r' | '\n' | ' ') => {},
        #[allow(clippy::arithmetic_side_effects, reason = "Can't happen.")]
        (LB::BeforeAttributeValue, '"'                     ) => {state.last_bite = LB::AttributeValueDoubleQuoted; state.attr_value_start = i+1;},
        #[allow(clippy::arithmetic_side_effects, reason = "Can't happen.")]
        (LB::BeforeAttributeValue, '\''                    ) => {state.last_bite = LB::AttributeValueSingleQuoted; state.attr_value_start = i+1;},
        (LB::BeforeAttributeValue, '>'                     ) => Err(EK::MissingAttributeValue)?,
        (LB::BeforeAttributeValue, _                       ) => {state.last_bite = LB::AttributeValueUnquoted; state.attr_value_start = i; munch(state, i, c)?;},


        (LB::AttributeValueDoubleQuoted, '"' ) => {state.last_bite = LB::AfterAttributeValueQuoted; if &state.input[state.attr_name_start..state.attr_name_end] == state.name {state.ret = Some(Some(&state.input[state.attr_value_start..i]));}},
        (LB::AttributeValueDoubleQuoted, '&' ) => {}, // Processed later.
        (LB::AttributeValueDoubleQuoted, '\0') => Err(EK::UnexpectedNullCharacter)?,
        (LB::AttributeValueDoubleQuoted, _   ) => {},


        (LB::AttributeValueSingleQuoted, '\'') => {state.last_bite = LB::AfterAttributeValueQuoted; if &state.input[state.attr_name_start..state.attr_name_end] == state.name {state.ret = Some(Some(&state.input[state.attr_value_start..i]));}},
        (LB::AttributeValueSingleQuoted, '&' ) => {}, // Processed later.
        (LB::AttributeValueSingleQuoted, '\0') => Err(EK::UnexpectedNullCharacter)?,
        (LB::AttributeValueSingleQuoted, _   ) => {},


        (LB::AttributeValueUnquoted, '\t' | '\r' | '\n' | ' ') => {state.last_bite = LB::BeforeAttributeName; if &state.input[state.attr_name_start..state.attr_name_end] == state.name {state.ret = Some(Some(&state.input[state.attr_value_start..i]));}},
        (LB::AttributeValueUnquoted, '&'                     ) => {}, // Processed later.
        (LB::AttributeValueUnquoted, '>'                     ) => {state.last_bite = LB::Done; if &state.input[state.attr_name_start..state.attr_name_end] == state.name {state.ret = Some(Some(&state.input[state.attr_value_start..i]));}},
        (LB::AttributeValueUnquoted, '\0'                    ) => Err(EK::UnexpectedNullCharacter)?,
        (LB::AttributeValueUnquoted, _                       ) => {},


        (LB::AfterAttributeValueQuoted, '\t' | '\r' | '\n' | ' ') => {state.last_bite = LB::BeforeAttributeName;},
        (LB::AfterAttributeValueQuoted, '/'                     ) => {state.last_bite = LB::SelfClosingStartTag;},
        (LB::AfterAttributeValueQuoted, '>'                     ) => {state.last_bite = LB::Done;},
        (LB::AfterAttributeValueQuoted, _                       ) => Err(EK::MissingWhitespaceBetweenAttributes)?,


        (LB::Done, _) => panic!("Logic error.")
    }

    Ok(())
}
