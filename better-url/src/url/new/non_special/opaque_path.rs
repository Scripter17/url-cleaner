//! Cannot-be-a-base.

use crate::prelude::*;

impl BetterUrl {
    /// `non-special:...`.
    pub(super) fn new_ns_opaque_path(scheme: Scheme<'_>, rest: &str) -> Result<Self, InvalidUrl> {
        let (path, query, fragment) = split_pqf(rest);

        let path     = OpaquePath          ::new(path    );
        let query    = MaybeNonSpecialQuery::new(query   );
        let fragment = MaybeFragment       ::new(fragment);


        let scheme_mark   = scheme.len();
        let path_start    = scheme_mark + 1;
        let path_after    = path_start + path.len();
        let query_mark    = query   .is_some().then_some(path_after                     );
        let fragment_mark = fragment.is_some().then_some(path_after + query.search_len());

        let len = path_after + query.search_len() + fragment.hash_len();

        if len > u32::MAX as usize {
            Err(TooLong)?;
        }

        let mut serialization = String::with_capacity(len);

        serialization.extend([scheme.as_str(), ":", path.as_str()]);

        if let Some(query   ) = query   .as_str() {serialization.extend(["?", query   ])}
        if let Some(fragment) = fragment.as_str() {serialization.extend(["#", fragment])}

        debug_assert_eq!(serialization.len(), len);

        Ok(Self {
            serialization,
            details: UrlDetails {
                scheme_mark   : scheme_mark as u32,
                username_after: None,
                host_start    : None,
                port_mark     : None,
                path_start    : path_start as u32,
                query_mark    : query_mark   .and_then(|x| NonZero::new(x as u32)),
                fragment_mark : fragment_mark.and_then(|x| NonZero::new(x as u32)),
                scheme: scheme.details(),
                host  : None,
                port: 0,
            }
        })
    }
}
