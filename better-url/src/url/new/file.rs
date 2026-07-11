//! File URLs.

use crate::prelude::*;

impl BetterUrl {
    /// Make a new file [`Self`].
    pub(super) fn new_file(scheme: Scheme<'_>, rest: &str) -> Result<Self, InvalidUrl> {
        let (host, pqf) = match rest.as_bytes() {
            [b'/' | b'\\', b'/' | b'\\', x @ ..] => {
                let rest = &rest[2..];
                match x {
                    [b'a'..=b'z' | b'A'..=b'Z', b':' | b'|'                                ] => ("", rest),
                    [b'a'..=b'z' | b'A'..=b'Z', b':' | b'|', b'/' | b'\\' | b'?' | b'#', ..] => ("", rest),
                    x => rest.split_at(x.iter().position(|&b| b == b'/' || b == b'\\' || b == b'?' || b == b'#').unwrap_or(x.len()))
                }
            },
            _ => ("", rest)
        };

        let (path, query, fragment) = split_pqf(pqf);


        let host     = FileHost         ::new(host    )?;
        let path     = FilePath         ::new(path    ) ;
        let query    = MaybeSpecialQuery::new(query   ) ;
        let fragment = MaybeFragment    ::new(fragment) ;


        let scheme_mark = scheme.len();
        let host_start   = scheme_mark + 3;
        let path_start   = host_start + host.len();
        let path_after   = path_start + path.len();
        let query_mark   = query.as_str().map(|_| path_after);

        let fragment_mark = match (query.as_str(), fragment.as_str()) {
            (_      , None   ) => None,
            (None   , Some(_)) => Some(path_after),
            (Some(x), Some(_)) => Some(path_after + 1 + x.len())
        };

        let len = path_after + query.len().map_or(0, |x| x + 1) + fragment.len().map_or(0, |x| x + 1);

        if len > u32::MAX as usize {
            Err(TooLong)?;
        }

        let mut serialization = String::with_capacity(len);

        serialization.extend([scheme.as_str(), "://", host.as_str(), path.as_str()]);

        if let Some(query   ) = query   .as_str() {serialization.extend(["?", query   ])}
        if let Some(fragment) = fragment.as_str() {serialization.extend(["#", fragment])}

        Ok(Self {
            serialization,
            scheme_mark  : scheme_mark as u32,
            username_after: None,
            host_start    : NonZero::new(host_start as u32),
            port_mark     : None,
            port          : 0,
            path_start    : path_start as u32,
            query_mark    : query_mark   .and_then(|x| NonZero::new(x as u32)),
            fragment_mark : fragment_mark.and_then(|x| NonZero::new(x as u32)),
            details: UrlDetails {
                host  : Some(host.details()),
                scheme: scheme.details()
            }
        })
    }
}
