//! File URLs.

use crate::prelude::*;

impl BetterUrl {
    /// [`SchemeType::File`].
    pub(super) fn new_file(scheme: Scheme<'_>, rest: &str) -> Result<Self, InvalidUrl> {
        // TODO: What?
        let (host, path, query, fragment) = match rest.as_bytes() {
            [b'/' | b'\\', b'/' | b'\\', b'a'..=b'z' | b'A'..=b'Z', b':' | b'|'] |
            [b'/' | b'\\', b'/' | b'\\', b'a'..=b'z' | b'A'..=b'Z', b':' | b'|', b'/' | b'\\' | b'?' | b'#', ..] => {
                let (path, query, fragment) = split_pqf(&rest[1..]);

                ("", path, query, fragment)
            },
            [b'/' | b'\\', b'/' | b'\\', ..] => {
                let rest = &rest[2..];

                let (rest, fragment) = pop_fragment    (rest);
                let (rest, query   ) = pop_query       (rest);
                let (host, path    ) = pop_special_path(rest);

                (host, path, query, fragment)
            },
            _ => {
                let (path, query, fragment) = split_pqf(rest);

                ("", path, query, fragment)
            }
        };


        let host     = FileHost         ::new(host    )?;
        let path     = FilePath         ::new(path    ) ;
        let query    = MaybeSpecialQuery::new(query   ) ;
        let fragment = MaybeFragment    ::new(fragment) ;


        let scheme_mark   = scheme.len();
        let host_start    = scheme_mark + 3;
        let path_start    = host_start + host.len();
        let path_after    = path_start + path.len();
        let query_mark    = query   .is_some().then_some(path_after                     );
        let fragment_mark = fragment.is_some().then_some(path_after + query.search_len());

        let len = path_after + query.search_len() + fragment.hash_len();

        if len > u32::MAX as usize {
            Err(TooLong)?;
        }

        let mut serialization = String::with_capacity(len);

        serialization.extend([scheme.as_str(), "://", host.as_str(), path.as_str()]);

        if let Some(query   ) = query   .as_str() {serialization.extend(["?", query   ])}
        if let Some(fragment) = fragment.as_str() {serialization.extend(["#", fragment])}

        debug_assert_eq!(serialization.len(), len);

        Ok(Self {
            serialization: unsafe {UrlString::new_unchecked(serialization)},
            details: UrlDetails {
                scheme_mark   : scheme_mark as u32,
                username_after: None,
                host_start    : NonZero::new(host_start as u32),
                port_mark     : None,
                path_start    : path_start as u32,
                query_mark    : query_mark   .and_then(|x| NonZero::new(x as u32)),
                fragment_mark : fragment_mark.and_then(|x| NonZero::new(x as u32)),
                scheme: scheme.details(),
                host: Some(host.details().into()),
                port: 0,
            }.into()
        })
    }
}
