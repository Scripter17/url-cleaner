//! Host.

use crate::prelude::*;

impl BetterUrl {
    /// `non_special://...`
    pub(super) fn new_ns_host(scheme: Scheme<'_>, rest: &str) -> Result<Self, InvalidUrl> {
        let (auth, pqf) = match rest.memchr3(b'/', b'?', b'#') {
            Some(i) => unsafe {(rest.get_unchecked(..i), rest.get_unchecked(i..))},
            None    =>         (rest                   , ""                      ),
        };

        let (userinfo, host, port ) = split_auth(auth);
        let (path, query, fragment) = split_pqf (pqf );

        if host.is_empty() && (userinfo.is_some() || port.is_some()) {
            Err(InvalidUrl::EmptyHostCantHaveUserinfoOrPort)?;
        }

        let userinfo = match userinfo {
            None | Some("") | Some(":") => None,
            Some(x)                     => Some(Userinfo::new(x)),
        };

        let host     = NonSpecialHost      ::new(host    )?;
        let port     = MaybePort           ::new(port    )?;
        let path     = NonSpecialPath      ::new(path    ) ;
        let query    = MaybeNonSpecialQuery::new(query   ) ;
        let fragment = MaybeFragment       ::new(fragment) ;


        let scheme_mark    = scheme.len();
        let username_after = userinfo.as_ref().map   (                 |x| scheme_mark + 3 + x.username_after());
        let host_start     = userinfo.as_ref().map_or(scheme_mark + 3, |x| scheme_mark + 4 + x.len()           );
        let port_mark      = port.as_str().map   (                         |_| host_start + host.len());
        let path_start     = port.as_str().map_or(host_start + host.len(), |x| host_start + host.len() + x.len() + 1);
        let path_after     = path_start + path.len();
        let query_mark     = query   .is_some().then_some(path_after                     );
        let fragment_mark  = fragment.is_some().then_some(path_after + query.search_len());

        let len = path_after + query.search_len() + fragment.hash_len();

        if len > u32::MAX as usize {
            Err(TooLong)?;
        }

        let mut serialization = String::with_capacity(len);

        serialization.extend([scheme.as_str(), "://"]);

        if let Some(userinfo) = userinfo {serialization.extend([userinfo.as_str(), "@"]);}

        serialization.push_str(host.as_str());

        if let Some(port) = port.as_str() {serialization.extend([":", port]);}

        serialization.push_str(path.as_str());

        if let Some(query   ) = query   .as_str() {serialization.extend(["?", query   ])}
        if let Some(fragment) = fragment.as_str() {serialization.extend(["#", fragment])}

        debug_assert_eq!(serialization.len(), len);

        Ok(Self {
            serialization,
            details: UrlDetails {
                scheme_mark  : scheme_mark as u32,
                username_after: username_after.and_then(|x| NonZero::new(x as u32)),
                host_start    : NonZero::new(host_start as u32),
                port_mark     : port_mark.and_then(|x| NonZero::new(x as u32)),
                path_start    : path_start as u32,
                query_mark    : query_mark   .and_then(|x| NonZero::new(x as u32)),
                fragment_mark : fragment_mark.and_then(|x| NonZero::new(x as u32)),
                scheme: scheme.details(),
                host  : Some(host.details().into()),
                port  : port.as_num().unwrap_or_default(),
            }
        })
    }
}
