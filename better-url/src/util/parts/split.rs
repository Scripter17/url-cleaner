//! URL parsing stuff.

use crate::prelude::*;

/// Split an authority, a `(userinfo@)?host(:port)?`, into the component parts.
pub fn split_auth(value: &str) -> (Option<&str>, &str, Option<&str>) {
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
pub fn split_pqf(value: &str) -> (&str, Option<&str>, Option<&str>) {
    unsafe {
        let (rest, fragment) = match value.memchr(b'#') {
            Some(i) => (value.get_unchecked(..i), Some(value.get_unchecked(i+1..))),
            None    => (value                   , None                            ),
        };

        let (path, query) = match rest.memchr(b'?') {
            Some(i) => (rest.get_unchecked(..i), Some(rest.get_unchecked(i+1..))),
            None    => (rest                   , None                           ),
        };

        (path, query, fragment)
    }
}
