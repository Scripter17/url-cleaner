use crate::prelude::*;

impl MyUrl {
    pub(super) fn new_can_be_a_base_host(scheme: Scheme<'_>, rest: &str) -> Result<Self, InvalidUrl> {
        let (auth, rest) = match rest.bytes().position(|b| b == b'/' || b == b'?' || b == b'#') {
            Some(i) => (&rest[..i], &rest[i..]),
            None    => (rest, ""),
        };

        let (userinfo, hostport) = match auth.split_once('@') {
            Some((userinfo, hostport)) => (userinfo, hostport),
            None                       => (""      , auth    ),
        };

        let (host, port) = match hostport.split_once(':') {
            Some((host, port)) => (host    , Some(port)),
            None               => (hostport, None      ),
        };

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



        let userinfo = Userinfo ::new       (userinfo) ;
        let host     = Host     ::new_opaque(host    ).map_err(InvalidHost::from)?;
        let port     = MaybePort::new       (port    )?;

        let path = Path::new_non_special(path);

        let query = match scheme.r#type() {
            SchemeType::File | SchemeType::SpecialNotFile => MaybeQuery::new_special    (query),
            SchemeType::NonSpecial                        => MaybeQuery::new_non_special(query),
        };

        let fragment = MaybeFragment::new(fragment);


        let scheme_end   = scheme.len();
        let username_end = scheme_end + 3 + userinfo.as_str().find(":").unwrap_or(userinfo.len());
        let host_start   = scheme_end + 3 + userinfo.len() + (!userinfo.is_empty()) as usize;
        let host_end     = host_start + host.len();
        let path_start   = host_end   + port.as_str().map_or(0, |x| x.len() + 1);
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

        serialization.extend([scheme.as_str(), "://"]);

        if !userinfo.is_empty() {
            serialization.extend([userinfo.as_str(), "@"]);
        }

        serialization.push_str(host.as_str());

        if let Some(port) = port.as_str() {serialization.extend([":", port]);}

        serialization.push_str(path.as_str());

        if let Some(query   ) = query   .as_str() {serialization.extend(["?", query   ])}
        if let Some(fragment) = fragment.as_str() {serialization.extend(["#", fragment])}

        Ok(Self {
            serialization,
            scheme_end    : scheme_end as u32,
            username_end  : NonZero::new(username_end as u32),
            host_start    : NonZero::new(host_start   as u32),
            host_end      : NonZero::new(host_end     as u32),
            port          : port.as_u16().unwrap_or_default(),
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
