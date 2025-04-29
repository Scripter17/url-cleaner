//! Named common [`percent_encoding::AsciiSet`]s.

use serde::{Serialize, Deserialize};
use percent_encoding::AsciiSet;

#[expect(unused_imports, reason = "Used in docs.")]
use crate::types::*;
use crate::util::*;

/// As defined in [the URL spec](https://url.spec.whatwg.org/#percent-encoded-bytes).
pub const FRAGMENT_PERCENT_ENCODE_SET             : AsciiSet = percent_encoding::CONTROLS  .add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
/// As defined in [the URL spec](https://url.spec.whatwg.org/#percent-encoded-bytes).
pub const QUERY_PERCENT_ENCODE_SET                : AsciiSet = percent_encoding::CONTROLS  .add(b' ').add(b'"').add(b'#').add(b'<').add(b'>');
/// As defined in [the URL spec](https://url.spec.whatwg.org/#percent-encoded-bytes).
pub const SPECIAL_QUERY_PERCENT_ENCODE_SET        : AsciiSet = QUERY_PERCENT_ENCODE_SET    .add(b'\'');
/// As defined in [the URL spec](https://url.spec.whatwg.org/#percent-encoded-bytes).
pub const PATH_PERCENT_ENCODE_SET                 : AsciiSet = QUERY_PERCENT_ENCODE_SET    .add(b'?').add(b'^').add(b'`').add(b'{').add(b'}');
/// As defined in [the URL spec](https://url.spec.whatwg.org/#percent-encoded-bytes).
pub const USERINFO_PERCENT_ENCODE_SET             : AsciiSet = PATH_PERCENT_ENCODE_SET     .add(b'/').add(b':').add(b';').add(b'=').add(b'@').add(b'[').add(b'\\').add(b']').add(b'|');
/// As defined in [the URL spec](https://url.spec.whatwg.org/#percent-encoded-bytes).
pub const COMPONENT_PERCENT_ENCODE_SET            : AsciiSet = USERINFO_PERCENT_ENCODE_SET .add(b'$').add(b'%').add(b'&').add(b'+').add(b',');
/// As defined in [the URL spec](https://url.spec.whatwg.org/#percent-encoded-bytes).
pub const X_WWW_FORM_URLENCODED_PERCENT_ENCODE_SET: AsciiSet = COMPONENT_PERCENT_ENCODE_SET.add(b'!').add(b'\'').add(b'(').add(b')');

/// The [`AsciiSet`] that emulates [`encodeURIComponent`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent).
pub const JS_ENCODE_URI_COMPONENT_ASCII_SET: AsciiSet = percent_encoding::NON_ALPHANUMERIC
    .remove(b'-' ).remove(b'_').remove(b'.').remove(b'!' ).remove(b'~').remove(b'*').remove(b'\'').remove(b'(').remove(b')');

/// The [`AsciiSet`] that emulates [`encodeURI`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURI).
pub const JS_ENCODE_URI_ASCII_SET: AsciiSet = JS_ENCODE_URI_COMPONENT_ASCII_SET
    .remove(b';').remove(b'/').remove(b'?').remove(b':').remove(b'@').remove(b'&')
    .remove(b'=').remove(b'+').remove(b'$').remove(b',').remove(b'#');

/// An enum of named common [`AsciiSet`]s for use in [`StringModification::PercentEncode`].
///
/// Defaults to [`Self::XWWWFormUrlEncoded`].
///
/// Unfortunately, custom sets aren't possible because [`percent_encoding::utf8_percent_encode`] requires a `'static` [`AsciiSet`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub enum PercentEncodeAlphabet {
    /// Use [`JS_ENCODE_URI_COMPONENT_ASCII_SET`].
    JsEncodeUriComponent,
    /// Use [`JS_ENCODE_URI_ASCII_SET`].
    JsEncodeUri,
    /// Use [`percent_encoding::NON_ALPHANUMERIC`].
    NonAlphanumeric,
    /// Use [`FRAGMENT_PERCENT_ENCODE_SET`].
    Fragment,
    /// Use [`QUERY_PERCENT_ENCODE_SET`].
    Query,
    /// Use [`SPECIAL_QUERY_PERCENT_ENCODE_SET`].
    Special,
    /// Use [`PATH_PERCENT_ENCODE_SET`].
    Path,
    /// Use [`USERINFO_PERCENT_ENCODE_SET`].
    Userinfo,
    /// Use [`COMPONENT_PERCENT_ENCODE_SET`].
    Component,
    /// Use [`X_WWW_FORM_URLENCODED_PERCENT_ENCODE_SET`].
    #[default]
    XWWWFormUrlEncoded,
    /// Use [`percent_encoding::CONTROLS`].
    Controls
}

impl PercentEncodeAlphabet {
    /// Get the corresponding [`AsciiSet`].
    pub fn get(&self) -> &'static AsciiSet {
        match self {
            Self::JsEncodeUriComponent => &JS_ENCODE_URI_COMPONENT_ASCII_SET,
            Self::JsEncodeUri          => &JS_ENCODE_URI_ASCII_SET,
            Self::NonAlphanumeric      => percent_encoding::NON_ALPHANUMERIC,
            Self::Fragment             => &FRAGMENT_PERCENT_ENCODE_SET,
            Self::Query                => &QUERY_PERCENT_ENCODE_SET,
            Self::Special              => &SPECIAL_QUERY_PERCENT_ENCODE_SET,
            Self::Path                 => &PATH_PERCENT_ENCODE_SET,
            Self::Userinfo             => &USERINFO_PERCENT_ENCODE_SET,
            Self::Component            => &COMPONENT_PERCENT_ENCODE_SET,
            Self::XWWWFormUrlEncoded   => &X_WWW_FORM_URLENCODED_PERCENT_ENCODE_SET,
            Self::Controls             => percent_encoding::CONTROLS
        }
    }
}
