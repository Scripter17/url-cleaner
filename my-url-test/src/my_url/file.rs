//! File URLs.

use super::*;

impl MyUrl {
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


        let scheme_end   = scheme.len();
        let host_start   = scheme_end + 3;
        let host_end     = host_start + host.len();
        let path_start   = host_end;
        let path_end     = path_start + path.len();
        let query_start  = query.as_str().map(|_| path_end);

        let fragment_start = match (query.as_str(), fragment.as_str()) {
            (_      , None   ) => None,
            (None   , Some(_)) => Some(path_end),
            (Some(x), Some(_)) => Some(path_end + 1 + x.len())
        };

        let len = path_end + query.len().map_or(0, |x| x + 1) + fragment.len().map_or(0, |x| x + 1);

        if len > u32::MAX as usize {
            Err(TooLong)?;
        }

        let mut serialization = String::with_capacity(len);

        serialization.extend([scheme.as_str(), "://", host.as_str(), path.as_str()]);

        if let Some(query   ) = query   .as_str() {serialization.extend(["?", query   ])}
        if let Some(fragment) = fragment.as_str() {serialization.extend(["#", fragment])}

        Ok(Self {
            serialization,
            scheme_end    : scheme_end as u32,
            username_end  : None,
            host_start    : NonZero::new(host_start as u32),
            host_end      : NonZero::new(host_end   as u32),
            port          : 0,
            path_start    : path_start as u32,
            query_start   : query_start   .and_then(|x| NonZero::new(x as u32)),
            fragment_start: fragment_start.and_then(|x| NonZero::new(x as u32)),
            details: UrlDetails {
                host  : Some(host.details()),
                scheme: scheme.details()
            }
        })
    }
}
