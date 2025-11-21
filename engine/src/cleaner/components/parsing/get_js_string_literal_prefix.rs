//! [`get_js_string_literal_prefix`] and co.

use serde::{Serialize, Deserialize};
use thiserror::Error;

/// The last state of the state machine used to unescape javascript string literal prefixes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum GetJsStringLiteralPrefixLastState {
    /// Before the start of the string literal.
    Outside,
    /// Inside the string literal but nothing else special.
    ///
    /// When an escape sequence finishes, returns to this state.
    Inside,
    /// The `\` of an escape sequence.
    Start,
    /// The first digit of an octal escape sequence.
    Octal1,
    /// The second digit of an octal escape sequence.
    Octal2,
    /// The `u` in `\xHH`.
    AsciiHexx,
    /// The first digit in an ascii escape sequence.
    AsciiHex1,
    /// The `u` in `\uHHHH`/`\u{HHHHH}`.
    UnicodeU,
    /// The first digit in `\uHHHH`.
    Unicode41,
    /// The second digit in `\uHHHH`.
    Unicode42,
    /// The third digit in `\uHHHH`.
    Unicode43,
    /// The `{` in `\u{HHHHH}`.
    UnicodeLeftBrace,
    /// The first digit in `\u{HHHHH}`.
    Unicode51,
    /// The second digit in `\u{HHHHH}`.
    Unicode52,
    /// The third digit in `\u{HHHHH}`.
    Unicode53,
    /// The fourth digit in `\u{HHHHH}`.
    Unicode54,
    /// The fifth digit in `\u{HHHHH}`.
    Unicode55
}

/// The enum of errors [`get_js_string_literal_prefix`] can return.
#[derive(Debug, Error)]
pub enum GetJsStringLiteralPrefixError {
    /// Returned when a syntax error is encountered.
    #[error("A syntax error was encountered.")]
    SyntaxError,
    /// Returned when an invalid codepoint is encountered.
    #[error("An invalid codepoint was encountered.")]
    InvalidCodepoint
}

/// Given a [`str`] that starts with a javascript string literal, return the value of that string.
///
/// TODO: Handle template strings.
/// # Errors
/// If a syntax error happens, returns the error [`GetJsStringLiteralPrefixError::SyntaxError`].
///
/// If an invalid codepoint is encountered, returns the error [`GetJsStringLiteralPrefixError::InvalidCodepoint`].
/// # Examples
/// ```
/// use url_cleaner_engine::prelude::*;
///
/// assert_eq!(get_js_string_literal_prefix("\"abc\\n\\u000Adef\"other stuff"                                ).unwrap(), "abc\n\ndef"          );
/// assert_eq!(get_js_string_literal_prefix("\"1\\u{a}2\\u{0a}3\\u{00a}4\\u{000a}5\\u{0000a}6\\u000a7\\\n8\"").unwrap(), "1\n2\n3\n4\n5\n6\n78");
/// assert_eq!(get_js_string_literal_prefix("\"'\\\"\"outside"                                               ).unwrap(), "'\""                 );
/// assert_eq!(get_js_string_literal_prefix("'\"\\''outside"                                                 ).unwrap(), "\"'"                 );
/// assert_eq!(get_js_string_literal_prefix("'a\\na'"                                                        ).unwrap(), "a\na"                );
/// assert_eq!(get_js_string_literal_prefix("'a\\\na'"                                                       ).unwrap(), "aa"                  );
///
/// get_js_string_literal_prefix("\"\\u{00000a}\"").unwrap_err();
/// ```
#[allow(clippy::missing_panics_doc, reason = "Shouldn't ever happen.")]
#[allow(clippy::unwrap_used, reason = "Who cares?")]
pub fn get_js_string_literal_prefix(s: &str) -> Result<String, GetJsStringLiteralPrefixError> {
    let mut ret = String::new();
    let mut last_state = GetJsStringLiteralPrefixLastState::Outside;

    let mut scratchspace: u32 = 0;
    let mut quote = '"';

    for c in s.chars() {
        match (last_state, c) {
            (GetJsStringLiteralPrefixLastState::Outside         , '"' | '\''                       ) => {last_state = GetJsStringLiteralPrefixLastState::Inside          ; quote = c;},
            (GetJsStringLiteralPrefixLastState::Inside          , '\\'                             ) => {last_state = GetJsStringLiteralPrefixLastState::Start           ;},
            (GetJsStringLiteralPrefixLastState::Start           , '0'                              ) => {last_state = GetJsStringLiteralPrefixLastState::Inside          ; ret.push('\0');},
            (GetJsStringLiteralPrefixLastState::Start           , 'b'                              ) => {last_state = GetJsStringLiteralPrefixLastState::Inside          ; ret.push('\u{0008}');},
            (GetJsStringLiteralPrefixLastState::Start           , 'g'                              ) => {last_state = GetJsStringLiteralPrefixLastState::Inside          ; ret.push('\u{000c}');},
            (GetJsStringLiteralPrefixLastState::Start           , 'n'                              ) => {last_state = GetJsStringLiteralPrefixLastState::Inside          ; ret.push('\n');},
            (GetJsStringLiteralPrefixLastState::Start           , '\n'                             ) => {last_state = GetJsStringLiteralPrefixLastState::Inside          ;},
            (GetJsStringLiteralPrefixLastState::Start           , 'r'                              ) => {last_state = GetJsStringLiteralPrefixLastState::Inside          ; ret.push('\r');},
            (GetJsStringLiteralPrefixLastState::Start           , 't'                              ) => {last_state = GetJsStringLiteralPrefixLastState::Inside          ; ret.push('\t');},
            (GetJsStringLiteralPrefixLastState::Start           , 'v'                              ) => {last_state = GetJsStringLiteralPrefixLastState::Inside          ; ret.push('\u{000b}');},
            (GetJsStringLiteralPrefixLastState::Start           , '\''                             ) => {last_state = GetJsStringLiteralPrefixLastState::Inside          ; ret.push('\'');},
            (GetJsStringLiteralPrefixLastState::Start           , '"'                              ) => {last_state = GetJsStringLiteralPrefixLastState::Inside          ; ret.push('"') ;},
            (GetJsStringLiteralPrefixLastState::Start           , '\\'                             ) => {last_state = GetJsStringLiteralPrefixLastState::Inside          ; ret.push('\\');},
            (GetJsStringLiteralPrefixLastState::Start           , '0'..='7'                        ) => {last_state = GetJsStringLiteralPrefixLastState::Octal1          ; scratchspace =                     c.to_digit( 8).unwrap();},
            (GetJsStringLiteralPrefixLastState::Octal1          , '0'..='7'                        ) => {last_state = GetJsStringLiteralPrefixLastState::Octal2          ; scratchspace = scratchspace *  8 + c.to_digit( 8).unwrap();},
            (GetJsStringLiteralPrefixLastState::Octal2          , '0'..='7'                        ) => {last_state = GetJsStringLiteralPrefixLastState::Inside          ; scratchspace = scratchspace *  8 + c.to_digit( 8).unwrap(); ret.push(char::from_u32(scratchspace).ok_or(GetJsStringLiteralPrefixError::InvalidCodepoint)?);},
            (GetJsStringLiteralPrefixLastState::Start           , 'x'                              ) => {last_state = GetJsStringLiteralPrefixLastState::AsciiHexx       ;},
            (GetJsStringLiteralPrefixLastState::AsciiHexx       , '0'..='7' | 'A'..='F' | 'a'..='f') => {last_state = GetJsStringLiteralPrefixLastState::AsciiHex1       ; scratchspace =                     c.to_digit(16).unwrap();},
            (GetJsStringLiteralPrefixLastState::AsciiHex1       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = GetJsStringLiteralPrefixLastState::Inside          ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap(); ret.push(char::from_u32(scratchspace).ok_or(GetJsStringLiteralPrefixError::InvalidCodepoint)?);},
            (GetJsStringLiteralPrefixLastState::Start           , 'u'                              ) => {last_state = GetJsStringLiteralPrefixLastState::UnicodeU        ;},
            (GetJsStringLiteralPrefixLastState::UnicodeU        , '{'                              ) => {last_state = GetJsStringLiteralPrefixLastState::UnicodeLeftBrace;},
            (GetJsStringLiteralPrefixLastState::UnicodeLeftBrace, '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = GetJsStringLiteralPrefixLastState::Unicode51       ; scratchspace =                     c.to_digit(16).unwrap();},
            (GetJsStringLiteralPrefixLastState::Unicode51       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = GetJsStringLiteralPrefixLastState::Unicode52       ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap();},
            (GetJsStringLiteralPrefixLastState::Unicode52       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = GetJsStringLiteralPrefixLastState::Unicode53       ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap();},
            (GetJsStringLiteralPrefixLastState::Unicode53       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = GetJsStringLiteralPrefixLastState::Unicode54       ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap();},
            (GetJsStringLiteralPrefixLastState::Unicode54       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = GetJsStringLiteralPrefixLastState::Unicode55       ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap();},
            (GetJsStringLiteralPrefixLastState::Unicode51
                | GetJsStringLiteralPrefixLastState::Unicode52
                | GetJsStringLiteralPrefixLastState::Unicode53
                | GetJsStringLiteralPrefixLastState::Unicode54
                | GetJsStringLiteralPrefixLastState::Unicode55  , '}'                              ) => {last_state = GetJsStringLiteralPrefixLastState::Inside          ; ret.push(char::from_u32(scratchspace).ok_or(GetJsStringLiteralPrefixError::InvalidCodepoint)?);},
            (GetJsStringLiteralPrefixLastState::UnicodeU        , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = GetJsStringLiteralPrefixLastState::Unicode41       ; scratchspace =                     c.to_digit(16).unwrap();},
            (GetJsStringLiteralPrefixLastState::Unicode41       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = GetJsStringLiteralPrefixLastState::Unicode42       ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap();},
            (GetJsStringLiteralPrefixLastState::Unicode42       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = GetJsStringLiteralPrefixLastState::Unicode43       ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap();},
            (GetJsStringLiteralPrefixLastState::Unicode43       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = GetJsStringLiteralPrefixLastState::Inside          ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap(); ret.push(char::from_u32(scratchspace).ok_or(GetJsStringLiteralPrefixError::InvalidCodepoint)?);},
            (GetJsStringLiteralPrefixLastState::Inside          , '"' | '\''                       ) if c == quote => break,
            (GetJsStringLiteralPrefixLastState::Start           , _                                ) => {last_state = GetJsStringLiteralPrefixLastState::Inside          ; ret.push(c);},
            (GetJsStringLiteralPrefixLastState::Inside          , _                                ) => {ret.push(c);}
            _ => Err(GetJsStringLiteralPrefixError::SyntaxError)?
        };
    }

    Ok(ret)
}
