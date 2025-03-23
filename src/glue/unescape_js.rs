use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsUnescapeCodeStateMachine {
    Outside,
    Inside,
    Start,
    Octal1,
    Octal2,
    AsciiHexx,
    AsciiHex1,
    UnicodeU,
    Unicode41,
    Unicode42,
    Unicode43,
    UnicodeLeftBrace,
    Unicode51,
    Unicode52,
    Unicode53,
    Unicode54,
    Unicode55,
}

#[derive(Debug, Error)]
pub enum JsUnescapeError {
    #[error("Syntax error.")]
    SyntaxError {
        last_state: JsUnescapeCodeStateMachine,
        c: char,
        scratchspace: u32,
        quote: char,
        partial: String
    },
    #[error("Invalid codepoint.")]
    InvalidCodepoint(u32)
}

/// Given a [`str`] that starts with a javascript string literal, return the value of that string.
///
/// TODO: Handle template strings.
/// # Errors
/// If a syntax error happens, returns the error [`JsUnescapeError::SyntaxError`].
///
/// If an invalid codepoint is encountered, returns the error [`JsUnescapeError::InvalidCodepoint`].
#[allow(clippy::missing_panics_doc, reason = "Shouldn't ever happen.")]
#[allow(clippy::unwrap_used, reason = "Who cares?")]
pub fn js_get_string_literal_prefix(s: &str) -> Result<String, JsUnescapeError> {
    let mut ret = String::new();
    let mut last_state = JsUnescapeCodeStateMachine::Outside;

    let mut scratchspace: u32 = 0;
    let mut quote = '"';

    for c in s.chars() {
        match (last_state, c) {
            (JsUnescapeCodeStateMachine::Outside         , '"' | '\''                       ) => {last_state = JsUnescapeCodeStateMachine::Inside          ; quote = c;},
            (JsUnescapeCodeStateMachine::Inside          , '\\'                             ) => {last_state = JsUnescapeCodeStateMachine::Start           ;},
            (JsUnescapeCodeStateMachine::Start           , '0'                              ) => {last_state = JsUnescapeCodeStateMachine::Inside          ; ret.push('\0');},
            (JsUnescapeCodeStateMachine::Start           , 'b'                              ) => {last_state = JsUnescapeCodeStateMachine::Inside          ; ret.push('\u{0008}');},
            (JsUnescapeCodeStateMachine::Start           , 'g'                              ) => {last_state = JsUnescapeCodeStateMachine::Inside          ; ret.push('\u{000c}');},
            (JsUnescapeCodeStateMachine::Start           , 'n'                              ) => {last_state = JsUnescapeCodeStateMachine::Inside          ; ret.push('\n');},
            (JsUnescapeCodeStateMachine::Start           , '\n'                             ) => {last_state = JsUnescapeCodeStateMachine::Inside          ; ret.push('\n');},
            (JsUnescapeCodeStateMachine::Start           , 'r'                              ) => {last_state = JsUnescapeCodeStateMachine::Inside          ; ret.push('\r');},
            (JsUnescapeCodeStateMachine::Start           , 't'                              ) => {last_state = JsUnescapeCodeStateMachine::Inside          ; ret.push('\t');},
            (JsUnescapeCodeStateMachine::Start           , 'v'                              ) => {last_state = JsUnescapeCodeStateMachine::Inside          ; ret.push('\u{000b}');},
            (JsUnescapeCodeStateMachine::Start           , '\''                             ) => {last_state = JsUnescapeCodeStateMachine::Inside          ; ret.push('\'');},
            (JsUnescapeCodeStateMachine::Start           , '"'                              ) => {last_state = JsUnescapeCodeStateMachine::Inside          ; ret.push('"') ;},
            (JsUnescapeCodeStateMachine::Start           , '\\'                             ) => {last_state = JsUnescapeCodeStateMachine::Inside          ; ret.push('\\');},
            (JsUnescapeCodeStateMachine::Start           , '0'..='7'                        ) => {last_state = JsUnescapeCodeStateMachine::Octal1          ; scratchspace =                     c.to_digit( 8).unwrap();},
            (JsUnescapeCodeStateMachine::Octal1          , '0'..='7'                        ) => {last_state = JsUnescapeCodeStateMachine::Octal2          ; scratchspace = scratchspace *  8 + c.to_digit( 8).unwrap();},
            (JsUnescapeCodeStateMachine::Octal2          , '0'..='7'                        ) => {last_state = JsUnescapeCodeStateMachine::Inside          ; scratchspace = scratchspace *  8 + c.to_digit( 8).unwrap(); ret.push(char::from_u32(scratchspace).ok_or(JsUnescapeError::InvalidCodepoint(scratchspace))?);},
            (JsUnescapeCodeStateMachine::Start           , 'x'                              ) => {last_state = JsUnescapeCodeStateMachine::AsciiHexx       ;},
            (JsUnescapeCodeStateMachine::AsciiHexx       , '0'..='7' | 'A'..='F' | 'a'..='f') => {last_state = JsUnescapeCodeStateMachine::AsciiHex1       ; scratchspace =                     c.to_digit(16).unwrap();},
            (JsUnescapeCodeStateMachine::AsciiHex1       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = JsUnescapeCodeStateMachine::Inside          ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap(); ret.push(char::from_u32(scratchspace).ok_or(JsUnescapeError::InvalidCodepoint(scratchspace))?);},
            (JsUnescapeCodeStateMachine::Start           , 'u'                              ) => {last_state = JsUnescapeCodeStateMachine::UnicodeU        ;},
            (JsUnescapeCodeStateMachine::UnicodeU        , '{'                              ) => {last_state = JsUnescapeCodeStateMachine::UnicodeLeftBrace;},
            (JsUnescapeCodeStateMachine::UnicodeLeftBrace, '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = JsUnescapeCodeStateMachine::Unicode51       ; scratchspace =                     c.to_digit(16).unwrap();},
            (JsUnescapeCodeStateMachine::Unicode51       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = JsUnescapeCodeStateMachine::Unicode52       ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap();},
            (JsUnescapeCodeStateMachine::Unicode52       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = JsUnescapeCodeStateMachine::Unicode53       ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap();},
            (JsUnescapeCodeStateMachine::Unicode53       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = JsUnescapeCodeStateMachine::Unicode54       ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap();},
            (JsUnescapeCodeStateMachine::Unicode54       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = JsUnescapeCodeStateMachine::Unicode55       ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap();},
            (JsUnescapeCodeStateMachine::Unicode51
                | JsUnescapeCodeStateMachine::Unicode52
                | JsUnescapeCodeStateMachine::Unicode53
                | JsUnescapeCodeStateMachine::Unicode54
                | JsUnescapeCodeStateMachine::Unicode55  , '}'                              ) => {last_state = JsUnescapeCodeStateMachine::Inside          ; ret.push(char::from_u32(scratchspace).ok_or(JsUnescapeError::InvalidCodepoint(scratchspace))?);},
            (JsUnescapeCodeStateMachine::UnicodeU        , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = JsUnescapeCodeStateMachine::Unicode41       ; scratchspace =                     c.to_digit(16).unwrap();},
            (JsUnescapeCodeStateMachine::Unicode41       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = JsUnescapeCodeStateMachine::Unicode42       ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap();},
            (JsUnescapeCodeStateMachine::Unicode42       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = JsUnescapeCodeStateMachine::Unicode43       ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap();},
            (JsUnescapeCodeStateMachine::Unicode43       , '0'..='9' | 'A'..='F' | 'a'..='f') => {last_state = JsUnescapeCodeStateMachine::Inside          ; scratchspace = scratchspace * 16 + c.to_digit(16).unwrap(); ret.push(char::from_u32(scratchspace).ok_or(JsUnescapeError::InvalidCodepoint(scratchspace))?);},
            (JsUnescapeCodeStateMachine::Inside          , '"' | '\''                       ) if c == quote => break,
            (JsUnescapeCodeStateMachine::Start           , _                                ) => {last_state = JsUnescapeCodeStateMachine::Inside          ; ret.push(c);},
            (JsUnescapeCodeStateMachine::Inside          , _                                ) => {ret.push(c);}
            _ => Err(JsUnescapeError::SyntaxError {last_state, quote, scratchspace, c, partial: ret.clone()})?
        };
    }

    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_js_unescape() {
        assert_eq!(js_get_string_literal_prefix("\"abc\\n\\u000Adef\"other stuff").unwrap(), "abc\n\ndef");
        assert_eq!(js_get_string_literal_prefix("\"1\\u{a}2\\u{0a}3\\u{00a}4\\u{000a}5\\u{0000a}6\\u000a7\\\n8\"").unwrap(), "1\n2\n3\n4\n5\n6\n7\n8");
        js_get_string_literal_prefix("\"\\u{00000a}\"").unwrap_err();
        assert_eq!(js_get_string_literal_prefix("\"'\\\"\"outside").unwrap(), "'\"");
        assert_eq!(js_get_string_literal_prefix("'\"\\''outside").unwrap(), "\"'");
    }
}
