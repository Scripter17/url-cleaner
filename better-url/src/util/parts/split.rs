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
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(split_pqf("/1/2?3?4#5#6"), ("/1/2", Some("3?4"), Some("5#6")));
/// assert_eq!(split_pqf("/1/2?3?4"    ), ("/1/2", Some("3?4"), None       ));
/// assert_eq!(split_pqf("/1/2"        ), ("/1/2", None       , None       ));
/// ```
pub fn split_pqf(value: &str) -> (&str, Option<&str>, Option<&str>) {
    let (rest, fragment) = pop_fragment(value);
    let (path, query   ) = pop_query   (rest );

    (path, query, fragment)
}

/// Pop the fragment.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(pop_fragment("example.com/1/2?3?4#5#6"), ("example.com/1/2?3?4", Some("5#6")));
/// ```
pub fn pop_fragment(value: &str) -> (&str, Option<&str>) {
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
/// assert_eq!(pop_query("example.com/1/2?3?4"), ("example.com/1/2", Some("3?4")));
/// ```
pub fn pop_query(value: &str) -> (&str, Option<&str>) {
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
/// assert_eq!(pop_special_path("example.com/1/2"  ), ("example.com", "/1/2"  ));
/// assert_eq!(pop_special_path("example.com\\1/2" ), ("example.com", "\\1/2" ));
/// assert_eq!(pop_special_path("example.com\\1\\2"), ("example.com", "\\1\\2"));
/// assert_eq!(pop_special_path("example.com"      ), ("example.com", "/"     ));
/// ```
pub fn pop_special_path(value: &str) -> (&str, &str) {
    match value.memchrn(*b"/\\") {
        Some(i) => unsafe {(value.get_unchecked(..i), value.get_unchecked(i..))},
        None    =>         (value                   , "/"                     ) ,
    }
}

/// Pop the non-special path
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(pop_non_special_path("example.com/1/2"  ), ("example.com"      , "/1/2"));
/// assert_eq!(pop_non_special_path("example.com\\1/2" ), ("example.com\\1"   , "/2"  ));
/// assert_eq!(pop_non_special_path("example.com\\1\\2"), ("example.com\\1\\2", ""    ));
/// assert_eq!(pop_non_special_path("example.com"      ), ("example.com"      , ""    ));
/// ```
pub fn pop_non_special_path(value: &str) -> (&str, &str) {
    match value.memchr(b'/') {
        Some(i) => unsafe {(value.get_unchecked(..i), value.get_unchecked(i..))},
        None    =>         (value                   , ""                      ) ,
    }
}
