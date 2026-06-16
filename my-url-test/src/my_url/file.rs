use better_url::util::*;

use super::*;

impl MyUrl {
    pub(super) fn new_file(scheme: Scheme<'_>, rest: &str) -> Result<Self, InvalidUrl> {
        println!("{rest}");

        let (host, rest) = match rest.strip_prefix(['/', '\\']) {
            Some(rest) => match rest.strip_prefix(['/', '\\']) {
                Some(rest) => match rest.bytes().position(|b| b == b'/' || b == b'\\' || b == b'?' || b == b'#') {
                    Some(i) if &rest[..i] == "localhost" => ("", &rest[i..]),
                    Some(i) if path_segment_is_windows_drive_letter(&rest[..i]) => ("", rest),
                    Some(i) => (&rest[..i], &rest[i..]),
                    None    => (rest, ""),
                },
                None => ("", rest),
            }
            None => ("", rest)
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



        let host = match host {
            ""   => EmptyHost::default().into(),
            host => Host::new(host)?,
        };

        let path = match scheme.r#type() {
            SchemeType::File           => Path::new_file            (path),
            SchemeType::SpecialNotFile => Path::new_special_not_file(path),
            SchemeType::NonSpecial     => Path::new_non_special     (path),
        };

        let query = match scheme.r#type() {
            SchemeType::File | SchemeType::SpecialNotFile => MaybeQuery::new_special    (query),
            SchemeType::NonSpecial                        => MaybeQuery::new_non_special(query),
        };

        let fragment = MaybeFragment::new(fragment);


        let scheme_end   = scheme.len();
        let host_start   = scheme_end + 3;
        let host_end     = host_start + host.len();
        let path_start   = host_end;
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

        serialization.extend([scheme.as_str(), "://", host.as_str(), path.as_str()]);

        if let Some(query   ) = query   .as_str() {serialization.extend(["?", query   ])}
        if let Some(fragment) = fragment.as_str() {serialization.extend(["#", fragment])}

        Ok(Self {
            serialization,
            scheme_end    : scheme_end as u32,
            username_end  : None,
            host_start    : NonZero::new(host_start   as u32),
            host_end      : NonZero::new(host_end     as u32),
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

