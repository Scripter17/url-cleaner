//! Special-not-file URLs.

use crate::prelude::*;

impl BetterUrl {
    /// Make a new-special-not-file [`Self`].
    pub(super) fn new_special_not_file(scheme: Scheme<'_>, mut rest: &str) -> Result<Self, InvalidUrl> {
        if let Some(trim) = rest.bytes().position(|b| b != b'/' && b != b'\\') {
            rest = unsafe {rest.get_unchecked(trim..)};
        }

        let ((userinfo, host, port), (path, query, fragment)) = match rest.bytes().position(|b| b == b'/' || b == b'\\' || b == b'?' || b == b'#') {
            Some(i) => unsafe {(split_auth(rest.get_unchecked(..i)), split_pqf(rest.get_unchecked(i..)))},
            None    =>         (split_auth(rest                   ), ("/", None, None                 )) ,
        };



        let userinfo = match userinfo {
            None | Some("") | Some(":") => None,
            Some(x) => Some(Userinfo::new(x)),
        };
        let host     = SpecialNotFileHost::new(host)?;
        let mut port = MaybePort         ::new(port.filter(|x| !x.is_empty()))?;

        if port.as_num() == scheme.default_port_num() {
            port = MaybePort(None);
        }

        let path     = SpecialNotFilePath::new(path    );
        let query    = MaybeSpecialQuery ::new(query   );
        let fragment = MaybeFragment     ::new(fragment);


        let scheme_mark   = scheme.len();
        let username_after = userinfo.as_ref().map(|x| scheme_mark + 3 + x.as_str().bytes().position(|b| b == b':').unwrap_or(x.len()));
        let host_start     = userinfo.as_ref().map_or(scheme_mark + 3, |x| scheme_mark + 4 + x.len());
        let port_mark      = port.as_str().map   (                         |_| host_start + host.len());
        let path_start     = port.as_str().map_or(host_start + host.len(), |x| host_start + host.len() + x.len() + 1);
        let path_after     = path_start + path.len();
        let query_mark     = query.as_str().map(|_| path_after);

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
