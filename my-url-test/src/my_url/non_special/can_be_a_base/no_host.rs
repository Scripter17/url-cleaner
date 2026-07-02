//! No host.

use crate::prelude::*;

impl MyUrl {
    /// Make a new non-special [`Self`] that can be a base but has no host.
    pub(super) fn new_can_be_a_base_no_host(scheme: Scheme<'_>, rest: &str) -> Result<Self, InvalidUrl> {
        let (path, rest) = match rest.bytes().position(|b| b == b'?' || b == b'#') {
            Some(i) => (&rest[..i], &rest[i..]),
            None    => (rest, ""),
        };

        let (query, fragment) = match rest.strip_prefix('?') {
            Some(rest) => match rest.split_once('#') {
                Some((query, fragment)) => (Some(query), Some(fragment)),
                None                    => (Some(rest ), None          ),
            },
            None => (None, rest.strip_prefix('#'))
        };

        let path     = NonSpecialPath      ::new(path    );
        let query    = MaybeNonSpecialQuery::new(query   );
        let fragment = MaybeFragment       ::new(fragment);

        let fake_host = match path.as_str().starts_with("//") {
            true  => "/.",
            false => "",
        };


        let scheme_end   = scheme.len();
        let host_start   = scheme_end + 1;
        let host_end     = host_start + fake_host.len();
        let path_start   = host_end;
        let path_end     = path_start + path.len();
        let query_start  = query.as_str().map(|_| path_end);

        let fragment_start = match fragment.as_str() {
            Some(_) => match query.as_str() {
                Some(query) => Some(path_end + 1 + query.len()),
                None        => Some(path_end),
            },
            None => None
        };

        let len = path_end + query.as_str().map_or(0, |x| x.len() + 1) + fragment.as_str().map_or(0, |x| x.len() + 1);

        if len > u32::MAX as usize {
            Err(TooLong)?;
        }

        let mut serialization = String::with_capacity(len);

        serialization.extend([scheme.as_str(), ":", fake_host, path.as_str()]);

        if let Some(query   ) = query   .as_str() {serialization.extend(["?", query   ])}
        if let Some(fragment) = fragment.as_str() {serialization.extend(["#", fragment])}

        Ok(Self {
            serialization,
            scheme_end    : scheme_end as u32,
            username_end  : None,
            host_start    : None,
            host_end      : None,
            port          : 0,
            path_start    : path_start as u32,
            query_start   : query_start   .and_then(|x| NonZero::new(x as u32)),
            fragment_start: fragment_start.and_then(|x| NonZero::new(x as u32)),
            details: UrlDetails {
                host  : None,
                scheme: scheme.details()
            }
        })
    }
}
