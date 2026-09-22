//! URL parsing stuff.

use crate::prelude::*;

/// Split an authority, a `(userinfo@)?host(:port)?`, into the component parts.
pub fn split_auth_bytes(value: &[u8]) -> (Option<&[u8]>, &[u8], Option<&[u8]>) {
    unsafe {
        let (userinfo, rest) = match value.memrchr(b'@') {
            Some(i) => (Some(value.get_unchecked(..i)), value.get_unchecked(i+1..)),
            None    => (None                          , value                     ),
        };

        let (host, port) = match rest.memrchr(b':') {
            Some(i) if rest.get_unchecked(i+1..).memchr(b']').is_none() => (rest.get_unchecked(..i), Some(rest.get_unchecked(i+1..))),
            _                                                           => (rest,                    None                           ),
        };

        (userinfo, host, port)
    }
}

/// Split a path, query, and fragment.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(split_pqf_bytes(b"/1/2?3?4#5#6"), (b"/1/2".as_slice(), Some(b"3?4".as_slice()), Some(b"5#6".as_slice())));
/// assert_eq!(split_pqf_bytes(b"/1/2?3?4"    ), (b"/1/2".as_slice(), Some(b"3?4".as_slice()), None                   ));
/// assert_eq!(split_pqf_bytes(b"/1/2"        ), (b"/1/2".as_slice(), None                   , None                   ));
/// ```
pub fn split_pqf_bytes(value: &[u8]) -> (&[u8], Option<&[u8]>, Option<&[u8]>) {
    let (rest, fragment) = pop_fragment_bytes(value);
    let (path, query   ) = pop_query_bytes   (rest );

    (path, query, fragment)
}

/// Pop the fragment.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(pop_fragment_bytes(b"example.com/1/2?3?4#5#6"), (b"example.com/1/2?3?4".as_slice(), Some(b"5#6".as_slice())));
/// ```
pub fn pop_fragment_bytes(value: &[u8]) -> (&[u8], Option<&[u8]>) {
    match value.memchr(b'#') {
        Some(i) => unsafe {(value.get_unchecked(..i), Some(value.get_unchecked(i+1..)))},
        None    =>         (value                   , None                            ) ,
    }
}

/// Pop the query.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(pop_query_bytes(b"example.com/1/2?3?4"), (b"example.com/1/2".as_slice(), Some(b"3?4".as_slice())));
/// ```
pub fn pop_query_bytes(value: &[u8]) -> (&[u8], Option<&[u8]>) {
    match value.memchr(b'?') {
        Some(i) => unsafe {(value.get_unchecked(..i), Some(value.get_unchecked(i+1..)))},
        None    =>         (value                   , None                            ) ,
    }
}

/// Pop the special path.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(pop_special_path_bytes(b"example.com/1/2"  ), (b"example.com".as_slice(), b"/1/2"  .as_slice()));
/// assert_eq!(pop_special_path_bytes(b"example.com\\1/2" ), (b"example.com".as_slice(), b"\\1/2" .as_slice()));
/// assert_eq!(pop_special_path_bytes(b"example.com\\1\\2"), (b"example.com".as_slice(), b"\\1\\2".as_slice()));
/// assert_eq!(pop_special_path_bytes(b"example.com"      ), (b"example.com".as_slice(), b"/"     .as_slice()));
/// ```
pub fn pop_special_path_bytes(value: &[u8]) -> (&[u8], &[u8]) {
    match value.memchrn(*b"/\\") {
        Some(i) => unsafe {(value.get_unchecked(..i), value.get_unchecked(i..))},
        None    =>         (value                   , b"/"                    ) ,
    }
}

/// Pop the non-special path
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(pop_non_special_path_bytes(b"example.com/1/2"  ), (b"example.com"      .as_slice(), b"/1/2".as_slice()));
/// assert_eq!(pop_non_special_path_bytes(b"example.com\\1/2" ), (b"example.com\\1"   .as_slice(), b"/2"  .as_slice()));
/// assert_eq!(pop_non_special_path_bytes(b"example.com\\1\\2"), (b"example.com\\1\\2".as_slice(), b""    .as_slice()));
/// assert_eq!(pop_non_special_path_bytes(b"example.com"      ), (b"example.com"      .as_slice(), b""    .as_slice()));
/// ```
pub fn pop_non_special_path_bytes(value: &[u8]) -> (&[u8], &[u8]) {
    match value.memchr(b'/') {
        Some(i) => unsafe {(value.get_unchecked(..i), value.get_unchecked(i..))},
        None    =>         (value                   , b""                     ) ,
    }
}
