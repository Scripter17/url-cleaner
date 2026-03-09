//! General utility functions.

use std::ops::Bound;

pub(crate) mod segments;
pub(crate) mod str_ext;
pub(crate) mod string_ext;
pub(crate) mod cow_str_ext;
pub(crate) mod iterator_ext;
pub(crate) mod double_ended_iterator_ext;

/// Prelude module for importing everything here better.
pub(crate) mod prelude {
    pub(crate) use super::segments::*;
    pub(crate) use super::str_ext::*;
    pub(crate) use super::string_ext::*;
    pub(crate) use super::cow_str_ext::*;
    pub(crate) use super::iterator_ext::*;
    pub(crate) use super::double_ended_iterator_ext::*;

    pub(crate) use super::{exorub, PercentEncodeSet, percent_encode, after_percent_sign};
}

/// Converts an `end` bound to a [`Bound`].
///
/// Specifically, if `i` is [`Some`], return [`Bound::Excluded`] or [`Bound::Unbounded`] if it's [`None`].
pub(crate) fn exorub(i: Option<usize>) -> Bound<usize> {
    match i {
        Some(i) => Bound::Excluded(i),
        None => Bound::Unbounded
    }
}

/// Percent encoding sets.
#[allow(dead_code, reason = "Unused variants are for completion.")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PercentEncodeSet {
    /// C0 control.
    C0,
    /// Fragment.
    Fragment,
    /// Query.
    Query,
    /// Special-query.
    SpecialQuery,
    /// Path.
    Path,
    /// Userinfo.
    Userinfo,
    /// Component.
    Component,
    /// Application/x-www-form-urlencoded.
    FormUrlencoded,
    /// [`url::PathSegmentsMut::extend`].
    PathSegment,
}

impl PercentEncodeSet {
    /// Returns [`true`] if `b` is in `self`'s set.
    pub(crate) fn matches(self, b: u8) -> bool {
        match self {
            Self::C0 => matches!(b, b'\x00'..=b'\x1f' | b'\x7f'..),
            Self::Fragment       => Self::C0       .matches(b) || matches!(b, b' ' | b'"' | b'<' | b'>' | b'`'),
            Self::Query          => Self::C0       .matches(b) || matches!(b, b' ' | b'"' | b'#' | b'<' | b'>'),
            Self::SpecialQuery   => Self::Query    .matches(b) || matches!(b, b'\''),
            Self::Path           => Self::Query    .matches(b) || matches!(b, b'?' | b'^' | b'`' | b'{' | b'}'),
            Self::Userinfo       => Self::Path     .matches(b) || matches!(b, b'/' | b':' | b';' | b'=' | b'@' | b'['..=b']' | b'|'),
            Self::Component      => Self::Userinfo .matches(b) || matches!(b, b'$' | b'&' | b'+' | b','),
            Self::FormUrlencoded => Self::Component.matches(b) || matches!(b, b'!' | b'\''..=b')' | b'~'),
            Self::PathSegment    => Self::Path     .matches(b) || matches!(b, b'/' | b'%'),
        }
    }
}

/// Helper function stolen from `percent_encoding`.
pub(crate) fn after_percent_sign<I: Iterator<Item = u8> + Clone>(iter: &mut I) -> Option<u8> {
    let mut cloned_iter = iter.clone();
    let h = char::from(cloned_iter.next()?).to_digit(16)?;
    let l = char::from(cloned_iter.next()?).to_digit(16)?;
    *iter = cloned_iter;
    Some(h as u8 * 0x10 + l as u8)
}

/// Percent encode `b`.
pub(crate) fn percent_encode(b: u8) -> &'static str {
    &ENC_TABLE[(b as usize) * 3 .. (b as usize) * 3 + 3]
}

/// Percent encoding table also stolen from `percent_encoding`.
const ENC_TABLE: &str = "\
    %00%01%02%03%04%05%06%07%08%09%0A%0B%0C%0D%0E%0F\
    %10%11%12%13%14%15%16%17%18%19%1A%1B%1C%1D%1E%1F\
    %20%21%22%23%24%25%26%27%28%29%2A%2B%2C%2D%2E%2F\
    %30%31%32%33%34%35%36%37%38%39%3A%3B%3C%3D%3E%3F\
    %40%41%42%43%44%45%46%47%48%49%4A%4B%4C%4D%4E%4F\
    %50%51%52%53%54%55%56%57%58%59%5A%5B%5C%5D%5E%5F\
    %60%61%62%63%64%65%66%67%68%69%6A%6B%6C%6D%6E%6F\
    %70%71%72%73%74%75%76%77%78%79%7A%7B%7C%7D%7E%7F\
    %80%81%82%83%84%85%86%87%88%89%8A%8B%8C%8D%8E%8F\
    %90%91%92%93%94%95%96%97%98%99%9A%9B%9C%9D%9E%9F\
    %A0%A1%A2%A3%A4%A5%A6%A7%A8%A9%AA%AB%AC%AD%AE%AF\
    %B0%B1%B2%B3%B4%B5%B6%B7%B8%B9%BA%BB%BC%BD%BE%BF\
    %C0%C1%C2%C3%C4%C5%C6%C7%C8%C9%CA%CB%CC%CD%CE%CF\
    %D0%D1%D2%D3%D4%D5%D6%D7%D8%D9%DA%DB%DC%DD%DE%DF\
    %E0%E1%E2%E3%E4%E5%E6%E7%E8%E9%EA%EB%EC%ED%EE%EF\
    %F0%F1%F2%F3%F4%F5%F6%F7%F8%F9%FA%FB%FC%FD%FE%FF\
    ";
