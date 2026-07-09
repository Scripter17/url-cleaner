//! Utilities.

/// Split an authority.
pub(crate) fn split_auth(value: &str) -> (Option<&str>, &str, Option<&str>) {
    let (userinfo, rest) = match value.bytes().rposition(|b| b == b'@') {
        Some(i) => (Some(&value[..i]), &value[i + 1..]),
        None    => (None             ,  value         ),
    };

    let (host, port) = match rest.bytes().rposition(|b| b == b':') {
        Some(i) if !rest[i+1..].contains(']') => (&rest[..i], Some(&rest[i+1..])),
        _                                     => ( rest     , None              ),
    };

    (userinfo, host, port)
}

/// Split a path, query, and fragment.
pub(crate) fn split_pqf(value: &str) -> (&str, Option<&str>, Option<&str>) {
    let mut qf = false;
    let mut ff = false;
    let mut qs = 0;
    let mut fs = 0;

    let mut a = value.bytes().enumerate();

    while let Some((i, b)) = a.next() {
        match b {
            b'?' => {
                qs = i;
                qf = true;
                if let Some(i) = a.find_map(|(i, b)| (b == b'#').then_some(i)) {
                    fs = i;
                    ff = true;
                }
            },
            b'#' => {
                fs = i;
                ff = true;
            },
            _ => continue
        }
        break;
    }

    unsafe {
        let (rest, fragment) = match ff {
            true  => (value.get_unchecked(..fs), Some(value.get_unchecked(fs+1..))),
            false => (value, None)
        };

        let (path, query) = match qf {
            true  => (rest.get_unchecked(..qs), Some(rest.get_unchecked(qs+1..))),
            false => (rest, None)
        };

        (path, query, fragment)
    }
}
