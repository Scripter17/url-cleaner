//! Special-not-file URLs.

use super::*;

impl MyUrl {
    /// Make a new-special-not-file [`Self`].
    pub(super) fn new_special_not_file(scheme: Scheme<'_>, mut rest: &str) -> Result<Self, InvalidUrl> {
        if let Some(trim) = rest.bytes().position(|b| b != b'/' && b != b'\\') {
            rest = unsafe {rest.get_unchecked(trim..)};
        }

        let ((userinfo, host, port), (path, query, fragment)) = match rest.bytes().position(|b| b == b'/' || b == b'\\' || b == b'?' || b == b'#') {
            Some(i) => unsafe {(split_auth(rest.get_unchecked(..i)), split_pqf(rest.get_unchecked(i..)))},
            None    =>         (split_auth(rest                   ), ("/", None, None                 )) ,
        };



        let userinfo = Userinfo          ::new(userinfo.unwrap_or_default());
        let host     = SpecialNotFileHost::new(host)?;
        let mut port = MaybePort         ::new(port.filter(|x| !x.is_empty()))?;

        if port.as_u16() == scheme.default_port() {
            port = MaybePort(None);
        }

        let path     = SpecialNotFilePath::new(path    );
        let query    = MaybeSpecialQuery ::new(query   );
        let fragment = MaybeFragment     ::new(fragment);


        let scheme_end   = scheme.len();
        let username_end = scheme_end + 3 + userinfo.as_str().bytes().position(|b| b == b':').unwrap_or(userinfo.len());
        let host_start   = scheme_end + 3 + userinfo.len() + (!userinfo.is_empty()) as usize;
        let host_end     = host_start + host.len();
        let path_start   = host_end   + port.as_str().map_or(0, |x| x.len() + 1);
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
