//! No host.

use crate::prelude::*;

impl BetterUrl {
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


        let scheme_mark = scheme.len();
        let path_start   = scheme_mark + 1 + fake_host.len();
        let path_after   = path_start + path.len();
        let query_mark   = query.as_str().map(|_| path_after);

        let fragment_mark = match fragment.as_str() {
            Some(_) => match query.as_str() {
                Some(query) => Some(path_after + 1 + query.len()),
                None        => Some(path_after),
            },
            None => None
        };

        let len = path_after + query.as_str().map_or(0, |x| x.len() + 1) + fragment.as_str().map_or(0, |x| x.len() + 1);

        if len > u32::MAX as usize {
            Err(TooLong)?;
        }

        let mut serialization = String::with_capacity(len);

        serialization.extend([scheme.as_str(), ":", fake_host, path.as_str()]);

        if let Some(query   ) = query   .as_str() {serialization.extend(["?", query   ])}
        if let Some(fragment) = fragment.as_str() {serialization.extend(["#", fragment])}

        Ok(Self {
            serialization,
            scheme_mark  : scheme_mark as u32,
            username_after: None,
            host_start    : None,
            port_mark     : None,
            port          : 0,
            path_start    : path_start as u32,
            query_mark    : query_mark   .and_then(|x| NonZero::new(x as u32)),
            fragment_mark : fragment_mark.and_then(|x| NonZero::new(x as u32)),
            details: UrlDetails {
                host  : None,
                scheme: scheme.details()
            }
        })
    }
}
