//! URL parsing stuff.

use crate::prelude::*;

/// Split an authority, a `(userinfo@)?host(:port)?`, into the component parts.
pub fn split_auth(value: &str) -> (Option<&str>, &str, Option<&str>) {
    unsafe {
        let (u, h, p) = split_auth_bytes(value.as_bytes());

        (
            u.map(|u| str::from_utf8_unchecked(u)),
                      str::from_utf8_unchecked(h) ,
            p.map(|p| str::from_utf8_unchecked(p)),
        )
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
    unsafe {
        let (p, q, f) = split_pqf_bytes(value.as_bytes());

        (
                      str::from_utf8_unchecked(p) ,
            q.map(|q| str::from_utf8_unchecked(q)),
            f.map(|f| str::from_utf8_unchecked(f)),
        )
    }
}

/// Pop the fragment.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(pop_fragment("example.com/1/2?3?4#5#6"), ("example.com/1/2?3?4", Some("5#6")));
/// ```
pub fn pop_fragment(value: &str) -> (&str, Option<&str>) {
    unsafe {
        let (r, f) = pop_fragment_bytes(value.as_bytes());

        (
                      str::from_utf8_unchecked(r) ,
            f.map(|f| str::from_utf8_unchecked(f)),
        )
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
    unsafe {
        let (r, q) = pop_query_bytes(value.as_bytes());

        (
                      str::from_utf8_unchecked(r) ,
            q.map(|q| str::from_utf8_unchecked(q)),
        )
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
    unsafe {
        let (auth, path) = pop_special_path_bytes(value.as_bytes());

        (
            str::from_utf8_unchecked(auth),
            str::from_utf8_unchecked(path),
        )
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
    unsafe {
        let (auth, path) = pop_non_special_path_bytes(value.as_bytes());

        (
            str::from_utf8_unchecked(auth),
            str::from_utf8_unchecked(path),
        )
    }
}
