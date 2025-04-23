//! Parsing javascript string literals.

use serde::{Serialize, Deserialize};
use thiserror::Error;

/// The last state of the state machine used to unescape javascript string literal prefixes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StringLiteralPrefixLastState {
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

/// The enum of errors [`string_literal_prefix`] can return.
#[derive(Debug, Error)]
pub enum StringLiteralPrefixError {
    /// Returned when a syntax error is encountered.
    #[error("A syntax error was encountered.")]
    SyntaxError {
        /// The last state of the state machine before the error was encountered.
        last_state: StringLiteralPrefixLastState,
        /// The index of the character that triggered the error.
        i: usize,
        /// The character that triggered the error.
        c: char,
        /// The scratchspace that was calculating the unescaped character.
        scratchspace: u32,
        /// The quote being used.
        quote: char,
        /// The calculated return value prior to the error.
        partial: String
    },
    /// Returned when an invalid codepoint is encountered.
    #[error("An invalid codepoint was encountered: {0}.")]
    InvalidCodepoint(u32)
}

/// Given a [`str`] that starts with a javascript string literal, return the value of that string.
///
/// TODO: Handle template strings.
/// # Errors
/// If a syntax error happens, returns the error [`StringLiteralPrefixError::SyntaxError`].
///
/// If an invalid codepoint is encountered, returns the error [`StringLiteralPrefixError::InvalidCodepoint`].
/// # Examples
/// ```
/// # use url_cleaner::glue::*;
/// assert_eq!(parse::js::string_literal_prefix("\"abc\\n\\u000Adef\"other stuff"                                ).unwrap(), "abc\n\ndef"          );
/// assert_eq!(parse::js::string_literal_prefix("\"1\\u{a}2\\u{0a}3\\u{00a}4\\u{000a}5\\u{0000a}6\\u000a7\\\n8\"").unwrap(), "1\n2\n3\n4\n5\n6\n78");
/// assert_eq!(parse::js::string_literal_prefix("\"'\\\"\"outside"                                               ).unwrap(), "'\""                 );
/// assert_eq!(parse::js::string_literal_prefix("'\"\\''outside"                                                 ).unwrap(), "\"'"                 );
/// assert_eq!(parse::js::string_literal_prefix("'a\\na'"                                                        ).unwrap(), "a\na"                );
/// assert_eq!(parse::js::string_literal_prefix("'a\\\na'"                                                       ).unwrap(), "aa"                  );
/// 
/// parse::js::string_literal_prefix("\"\\u{00000a}\"").unwrap_err();
/// ```
#[allow(clippy::missing_panics_doc, reason = "Shouldn't ever happen.")]
#[allow(clippy::unwrap_used, reason = "Who cares?")]
pub fn string_literal_prefix(s: &str) -> Result<String, StringLiteralPrefixError> {
    let mut ret = String::new();
    let mut last_state = StringLiteralPrefixLastState::Outside;

    let mut scratchspace: u32 = 0;
    let mut quote = '"';

    for (i, c) in s.chars().enumerate() {
        #[allow(clippy::arithmetic_side_effects, reason = "Shouldn't ever happen.")]
        match (last_state, c) {
            (StringLiteralPrefixLastState::Outside         , '"' | '\''                       ) => {last_state = StringLiteralPrefixLastState::Inside          ; quote = c;},
            (StringLiteralPrefixLastState::Inside          , '\\'                             ) => {last_state = StringLiteralPrefixLastState::Start           ;},
            (StringLiteralPrefixLastState::Start           , '0'                              ) => {last_state = StringLiteralPrefixLastState::Inside          ; ret.push('\0');},
            (StringLiteralPrefixLastState::Start           , 'b'                              ) => {last_state = StringLiteralPrefixLastState::Inside          ; ret.push('\u{0008}');},
            (StringLiteralPrefixLastState::Start           , 'g'                              ) => {last_state = StringLiteralPrefixLastState::Inside          ; ret.push('\u{000c}');},
            (StringLiteralPrefixLastState::Start           , 'n'                              ) => {last_state = StringLiteralPrefixLastState::Inside          ; ret.push('\n');},
            (StringLiteralPrefixLastState::Start           , '\n'                             ) => {last_state = StringLiteralPrefixLastState::Inside          ;},
            (StringLiteralPrefixLastState::Start           , 'r'                              ) => {last_state = StringLiteralPrefixLastState::Inside          ; ret.push('\r');},
            (StringLiteralPrefixLastState::Start           , 't'                              ) => {last_state = StringLiteralPrefixLastState::Inside          ; ret.push('\t');},
            (StringLiteralPrefixLastState::Start           , 'v'                              ) => {last_state = StringLiteralPrefixLastState::Inside          ; ret.push('\u{000b}');},
            (StringLiteralPrefixLastState::Start           , '\''                             ) => {last_state = StringLiteralPrefixLastState::Inside          ; ret.push('\'');},
            (StringLiteralPrefixLastState::Start           , '"'                              ) => {last_state = StringLiteralPrefixLastState::Inside          ; ret.push('"') ;},
            (StringLiteralPrefixLastState::Start           , '\\'                             ) => {last_state = StringLiteralPrefixLastState::Inside          ; ret.push('\\');},
            (StringLiteralPrefixLastState::Start           , '0'..='7'                        ) => {last_state = StringLiteralPrefixLastState::Octal1          ; scratchspace =                     c.to_digit( 8).unwrap();},
            (StringLiteralPrefixLastState::Octal1          , '0'..='7'                        ) => {last_state = StringLiteralPrefixLastState::Octal2          ; scratchspace = scratchspace *  8 + c.to_digit( 8).unwrap();},
            (StringLiteralPrefixLastState::Octal2          , '0'..='7'                        ) => {last_state = StringLiteralPrefixLastState::Inside          ; scratchspace = scratchspace *  8 + c.to_digit( 8).unwrap(); ret.push(char::from_u32(scratchspace).ok_or(StringLiteralPrefixError::InvalidCodepoint(scratchspace))?);},
            (StringLiteralPrefixLastState::Start           , 'x'                              ) => {last_state = StringLiteralPrefixLastState::AsciiHexx       ;},
            (StringLiteralPrefixLastState::AsciiHexx       , '0'..='7' | 'A'..='F' | 'a'..='f') => {last_state = StringLiteralPrefixLastState::AsciiHex1       ; scratchspace =                     c.to_digit(16).unwrap();},
            (StringLiteralPrefixLastState::AsciiHex1       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = StringLiteralPrefixLastState::Inside          ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap(); ret.push(char::from_u32(scratchspace).ok_or(StringLiteralPrefixError::InvalidCodepoint(scratchspace))?);},
            (StringLiteralPrefixLastState::Start           , 'u'                              ) => {last_state = StringLiteralPrefixLastState::UnicodeU        ;},
            (StringLiteralPrefixLastState::UnicodeU        , '{'                              ) => {last_state = StringLiteralPrefixLastState::UnicodeLeftBrace;},
            (StringLiteralPrefixLastState::UnicodeLeftBrace, '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = StringLiteralPrefixLastState::Unicode51       ; scratchspace =                     c.to_digit(16).unwrap();},
            (StringLiteralPrefixLastState::Unicode51       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = StringLiteralPrefixLastState::Unicode52       ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap();},
            (StringLiteralPrefixLastState::Unicode52       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = StringLiteralPrefixLastState::Unicode53       ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap();},
            (StringLiteralPrefixLastState::Unicode53       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = StringLiteralPrefixLastState::Unicode54       ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap();},
            (StringLiteralPrefixLastState::Unicode54       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = StringLiteralPrefixLastState::Unicode55       ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap();},
            (StringLiteralPrefixLastState::Unicode51
                | StringLiteralPrefixLastState::Unicode52
                | StringLiteralPrefixLastState::Unicode53
                | StringLiteralPrefixLastState::Unicode54
                | StringLiteralPrefixLastState::Unicode55  , '}'                              ) => {last_state = StringLiteralPrefixLastState::Inside          ; ret.push(char::from_u32(scratchspace).ok_or(StringLiteralPrefixError::InvalidCodepoint(scratchspace))?);},
            (StringLiteralPrefixLastState::UnicodeU        , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = StringLiteralPrefixLastState::Unicode41       ; scratchspace =                     c.to_digit(16).unwrap();},
            (StringLiteralPrefixLastState::Unicode41       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = StringLiteralPrefixLastState::Unicode42       ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap();},
            (StringLiteralPrefixLastState::Unicode42       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = StringLiteralPrefixLastState::Unicode43       ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap();},
            (StringLiteralPrefixLastState::Unicode43       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = StringLiteralPrefixLastState::Inside          ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap(); ret.push(char::from_u32(scratchspace).ok_or(StringLiteralPrefixError::InvalidCodepoint(scratchspace))?);},
            (StringLiteralPrefixLastState::Inside          , '"' | '\''                       ) if c == quote => break,
            (StringLiteralPrefixLastState::Start           , _                                ) => {last_state = StringLiteralPrefixLastState::Inside          ; ret.push(c);},
            (StringLiteralPrefixLastState::Inside          , _                                ) => {ret.push(c);}
            _ => Err(StringLiteralPrefixError::SyntaxError {last_state, i, c, scratchspace, quote, partial: ret.clone()})?
        };
    }

    Ok(ret)
}

