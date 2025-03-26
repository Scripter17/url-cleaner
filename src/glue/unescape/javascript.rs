//! Unescaping for javascript.

use serde::{Serialize, Deserialize};
use thiserror::Error;

/// The last state of the state machine used to unescape javascript string literal prefixes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JSSLPLastState {
    /// Before the start of the string literal.
    Outside,
    /// Inside the string literal but nothing else special.
    ///
    /// When an escape sequence finishes, returns to this state.
    Inside,
    /// THe `\` of an escape sequence.
    Start,
    /// The first digit of an octal escape sequence.
    Octal1,
    /// The second digit of an octal escape sequence.
    Octal2,
    /// The `u` in `\xHH`.
    AsciiHexx,
    /// The first digit in an ascii escape sequence.
    AsciiHex1,
    /// THe `u` in `\uHHHH`/`\u{HHHHH}`.
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

/// The enum of errors [`unescape_javascript_string_literal_prefix`] can return.
#[derive(Debug, Error)]
pub enum JSSLPError {
    /// Returned when a syntax error is encountered.
    #[error("A syntax error was encountered.")]
    SyntaxError {
        /// The last state of the state machine before the error was encountered.
        last_state: JSSLPLastState,
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
/// If a syntax error happens, returns the error [`JSSLPLastState::SyntaxError`].
///
/// If an invalid codepoint is encountered, returns the error [`JSSLPLastState::InvalidCodepoint`].
/// # Examples
/// ```
/// # use url_cleaner::glue::*;
/// assert_eq!(unescape_javascript_string_literal_prefix("\"abc\\n\\u000Adef\"other stuff"                                ).unwrap(), "abc\n\ndef"          );
/// assert_eq!(unescape_javascript_string_literal_prefix("\"1\\u{a}2\\u{0a}3\\u{00a}4\\u{000a}5\\u{0000a}6\\u000a7\\\n8\"").unwrap(), "1\n2\n3\n4\n5\n6\n78");
/// assert_eq!(unescape_javascript_string_literal_prefix("\"'\\\"\"outside"                                               ).unwrap(), "'\""                 );
/// assert_eq!(unescape_javascript_string_literal_prefix("'\"\\''outside"                                                 ).unwrap(), "\"'"                 );
/// assert_eq!(unescape_javascript_string_literal_prefix("'a\\na'"                                                        ).unwrap(), "a\na"                );
/// assert_eq!(unescape_javascript_string_literal_prefix("'a\\\na'"                                                       ).unwrap(), "aa"                  );
/// 
/// unescape_javascript_string_literal_prefix("\"\\u{00000a}\"").unwrap_err();
/// ```
#[allow(clippy::missing_panics_doc, reason = "Shouldn't ever happen.")]
#[allow(clippy::unwrap_used, reason = "Who cares?")]
pub fn unescape_javascript_string_literal_prefix(s: &str) -> Result<String, JSSLPError> {
    let mut ret = String::new();
    let mut last_state = JSSLPLastState::Outside;

    let mut scratchspace: u32 = 0;
    let mut quote = '"';

    for (i, c) in s.chars().enumerate() {
        #[allow(clippy::arithmetic_side_effects, reason = "Shouldn't ever happen.")]
        match (last_state, c) {
            (JSSLPLastState::Outside         , '"' | '\''                       ) => {last_state = JSSLPLastState::Inside          ; quote = c;},
            (JSSLPLastState::Inside          , '\\'                             ) => {last_state = JSSLPLastState::Start           ;},
            (JSSLPLastState::Start           , '0'                              ) => {last_state = JSSLPLastState::Inside          ; ret.push('\0');},
            (JSSLPLastState::Start           , 'b'                              ) => {last_state = JSSLPLastState::Inside          ; ret.push('\u{0008}');},
            (JSSLPLastState::Start           , 'g'                              ) => {last_state = JSSLPLastState::Inside          ; ret.push('\u{000c}');},
            (JSSLPLastState::Start           , 'n'                              ) => {last_state = JSSLPLastState::Inside          ; ret.push('\n');},
            (JSSLPLastState::Start           , '\n'                             ) => {last_state = JSSLPLastState::Inside          ;},
            (JSSLPLastState::Start           , 'r'                              ) => {last_state = JSSLPLastState::Inside          ; ret.push('\r');},
            (JSSLPLastState::Start           , 't'                              ) => {last_state = JSSLPLastState::Inside          ; ret.push('\t');},
            (JSSLPLastState::Start           , 'v'                              ) => {last_state = JSSLPLastState::Inside          ; ret.push('\u{000b}');},
            (JSSLPLastState::Start           , '\''                             ) => {last_state = JSSLPLastState::Inside          ; ret.push('\'');},
            (JSSLPLastState::Start           , '"'                              ) => {last_state = JSSLPLastState::Inside          ; ret.push('"') ;},
            (JSSLPLastState::Start           , '\\'                             ) => {last_state = JSSLPLastState::Inside          ; ret.push('\\');},
            (JSSLPLastState::Start           , '0'..='7'                        ) => {last_state = JSSLPLastState::Octal1          ; scratchspace =                     c.to_digit( 8).unwrap();},
            (JSSLPLastState::Octal1          , '0'..='7'                        ) => {last_state = JSSLPLastState::Octal2          ; scratchspace = scratchspace *  8 + c.to_digit( 8).unwrap();},
            (JSSLPLastState::Octal2          , '0'..='7'                        ) => {last_state = JSSLPLastState::Inside          ; scratchspace = scratchspace *  8 + c.to_digit( 8).unwrap(); ret.push(char::from_u32(scratchspace).ok_or(JSSLPError::InvalidCodepoint(scratchspace))?);},
            (JSSLPLastState::Start           , 'x'                              ) => {last_state = JSSLPLastState::AsciiHexx       ;},
            (JSSLPLastState::AsciiHexx       , '0'..='7' | 'A'..='F' | 'a'..='f') => {last_state = JSSLPLastState::AsciiHex1       ; scratchspace =                     c.to_digit(16).unwrap();},
            (JSSLPLastState::AsciiHex1       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = JSSLPLastState::Inside          ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap(); ret.push(char::from_u32(scratchspace).ok_or(JSSLPError::InvalidCodepoint(scratchspace))?);},
            (JSSLPLastState::Start           , 'u'                              ) => {last_state = JSSLPLastState::UnicodeU        ;},
            (JSSLPLastState::UnicodeU        , '{'                              ) => {last_state = JSSLPLastState::UnicodeLeftBrace;},
            (JSSLPLastState::UnicodeLeftBrace, '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = JSSLPLastState::Unicode51       ; scratchspace =                     c.to_digit(16).unwrap();},
            (JSSLPLastState::Unicode51       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = JSSLPLastState::Unicode52       ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap();},
            (JSSLPLastState::Unicode52       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = JSSLPLastState::Unicode53       ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap();},
            (JSSLPLastState::Unicode53       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = JSSLPLastState::Unicode54       ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap();},
            (JSSLPLastState::Unicode54       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = JSSLPLastState::Unicode55       ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap();},
            (JSSLPLastState::Unicode51
                | JSSLPLastState::Unicode52
                | JSSLPLastState::Unicode53
                | JSSLPLastState::Unicode54
                | JSSLPLastState::Unicode55  , '}'                              ) => {last_state = JSSLPLastState::Inside          ; ret.push(char::from_u32(scratchspace).ok_or(JSSLPError::InvalidCodepoint(scratchspace))?);},
            (JSSLPLastState::UnicodeU        , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = JSSLPLastState::Unicode41       ; scratchspace =                     c.to_digit(16).unwrap();},
            (JSSLPLastState::Unicode41       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = JSSLPLastState::Unicode42       ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap();},
            (JSSLPLastState::Unicode42       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = JSSLPLastState::Unicode43       ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap();},
            (JSSLPLastState::Unicode43       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = JSSLPLastState::Inside          ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap(); ret.push(char::from_u32(scratchspace).ok_or(JSSLPError::InvalidCodepoint(scratchspace))?);},
            (JSSLPLastState::Inside          , '"' | '\''                       ) if c == quote => break,
            (JSSLPLastState::Start           , _                                ) => {last_state = JSSLPLastState::Inside          ; ret.push(c);},
            (JSSLPLastState::Inside          , _                                ) => {ret.push(c);}
            _ => Err(JSSLPError::SyntaxError {last_state, i, c, scratchspace, quote, partial: ret.clone()})?
        };
    }

    Ok(ret)
}
