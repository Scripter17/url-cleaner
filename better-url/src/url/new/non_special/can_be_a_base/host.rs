//! Host.

use crate::prelude::*;

impl BetterUrl {
    /// Make a new non-special [`Self`] that can be a base and has a host. 
    pub(super) fn new_can_be_a_base_host(scheme: Scheme<'_>, rest: &str) -> Result<Self, InvalidUrl> {
        let (auth, pqf) = match rest.bytes().position(|b| b == b'/' || b == b'?' || b == b'#') {
            Some(i) => (&rest[..i], &rest[i..]),
            None    => (rest, ""),
        };

        let (userinfo, host, port) = split_auth(auth);
        let (path, query, fragment) = split_pqf(pqf);

        if host.is_empty() && (userinfo.is_some() || port.is_some()) {
            Err(InvalidUrl::EmptyHostCantHaveUserinfoOrPort)?;
        }

        let userinfo = match userinfo {
            None | Some("") | Some(":") => None,
            Some(x) => Some(Userinfo::new(x))
        };

        let host     = NonSpecialHost      ::new(host    )?;
        let port     = MaybePort           ::new(port    )?;
        let path     = NonSpecialPath      ::new(path    ) ;
        let query    = MaybeNonSpecialQuery::new(query   ) ;
        let fragment = MaybeFragment       ::new(fragment) ;


        let scheme_mark   = scheme.len();
        let username_after = userinfo.as_ref().map(|x| scheme_mark + 3 + x.as_str().bytes().position(|b| b == b':').unwrap_or(x.len()));
        let host_start     = userinfo.as_ref().map_or(scheme_mark + 3, |x| scheme_mark + 4 + x.len());
        let port_mark      = port.as_str().map   (                         |_| host_start + host.len());
        let path_start     = port.as_str().map_or(host_start + host.len(), |x| host_start + host.len() + x.len() + 1);
        let path_after     = path_start   + path.len();
        let query_mark     = query.as_str().map(|_| path_after);

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

        serialization.extend([scheme.as_str(), "://"]);

        if let Some(userinfo) = userinfo {
            serialization.extend([userinfo.as_str(), "@"]);
        }

        serialization.push_str(host.as_str());

        if let Some(port) = port.as_str() {serialization.extend([":", port]);}

        serialization.push_str(path.as_str());

        if let Some(query   ) = query   .as_str() {serialization.extend(["?", query   ])}
        if let Some(fragment) = fragment.as_str() {serialization.extend(["#", fragment])}

        Ok(Self {
            serialization,
            scheme_mark  : scheme_mark as u32,
            username_after: username_after.and_then(|x| NonZero::new(x as u32)),
            host_start    : NonZero::new(host_start as u32),
            port_mark     : port_mark.and_then(|x| NonZero::new(x as u32)),
            port          : port.as_num().unwrap_or_default(),
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
