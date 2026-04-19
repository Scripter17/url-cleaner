//! Encoding.

use crate::prelude::*;

/// Encode a domain label, in-place if possible.
///
/// Please note that there are no checks to ensure the input is a valid label.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(encode_label("abc".into()), "abc");
/// assert_eq!(encode_label("Αθήνα".into()), "xn--jxafb0a0a");
/// ```
pub fn encode_label(s: Cow<'_, str>) -> Cow<'_, str> {
    if s.is_ascii() {
        s
    } else {
        let mut ret = String::new();
        encode_label_into(s.chars(), &mut ret);
        ret.into()
    }
}

/// Encode a domain label into a [`String`].
///
/// Please note that there are no checks to ensure the input is a valid label.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// let mut out = String::new();
/// encode_label_into("abc".chars(), &mut out);
/// assert_eq!(out, "abc");
///
/// let mut out = String::new();
/// encode_label_into("Αθήνα".chars(), &mut out);
/// assert_eq!(out, "xn--jxafb0a0a");
/// ```
pub fn encode_label_into<I: IntoIterator<Item = char>>(iter: I, out: &mut String) {
    encode_label_into_bytes(iter, unsafe {out.as_mut_vec()})
}

/// Encode a domain label into a [`Vec`] of bytes.
///
/// Please note that there are no checks to ensure the input is a valid label.
pub fn encode_label_into_bytes<I: IntoIterator<Item = char>>(iter: I, out: &mut Vec<u8>) {
    let start = out.len();

    let (ascii, unicode) = punycode_into_bytes(NORMALIZER.map_normalize(iter.into_iter()), out);

    match (ascii, unicode) {
        (0, 0) => {},
        (_, 0) => {out.pop();},
        (_, _) => {
            out.insert(start    , b'x');
            out.insert(start + 1, b'n');
            out.insert(start + 2, b'-');
            out.insert(start + 3, b'-');
        }
    }
}
