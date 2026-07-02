//! [`MyUrl`].

use crate::prelude::*;

mod join;

mod file;
mod special_not_file;
mod non_special;

mod path;
mod query;
mod fragment;

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

/// The enum of errors that can occur when trying to parse an invalid URL.
#[derive(Debug, Error)]
pub enum InvalidUrl {
    /// [`InvalidScheme`].
    #[error(transparent)]
    InvalidScheme(#[from] InvalidScheme),
    /// [`InvalidHost`].
    #[error(transparent)]
    InvalidHost(#[from] InvalidHost),
    /// [`InvalidPort`].
    #[error(transparent)]
    InvalidPort(#[from] InvalidPort),
    /// [`TooLong`].
    #[error(transparent)]
    TooLong(#[from] TooLong),
    /// Other.
    #[error("Other")]
    Other
}

/// A URL type decoupled from [`url`].
#[derive(Debug, Clone)]
pub struct MyUrl {
    /// The serialization.
    serialization : String,
    /// The end of the scheme.
    scheme_end    : u32,
    /// The end of the username.
    username_end  : Option<NonZero<u32>>,
    /// The start of the host.
    host_start    : Option<NonZero<u32>>,
    /// The end of the host.
    host_end      : Option<NonZero<u32>>,
    /// If [`Self::host_end`] is [`Some`] and not [`Self::path_start`], the port.
    port          : u16,
    /// The start of the path.
    path_start    : u32,
    /// The start of the query.
    query_start   : Option<NonZero<u32>>,
    /// The start of the fragment.
    fragment_start: Option<NonZero<u32>>,
    /// The [`UrlDetails`].
    details       : UrlDetails,
}

impl MyUrl {
    /// Make a new [`Self`].
    pub fn new(value: &str) -> Result<Self, InvalidUrl> {
        let start = value.bytes(). position(|b| b > 0x20 && b != 0x7F).unwrap_or(0);
        let end   = value.bytes().rposition(|b| b > 0x20 && b != 0x7F).map_or(0, |x| x + 1);

        let mut value = Cow::Borrowed(&value[start..end]);

        if value.bytes().any(|b| b == b'\t' || b == b'\n' || b == b'\r') {
            value.to_mut().retain(|c| c != '\t' && c != '\n' && c != '\r');
        }

        let i = value.bytes().position(|b| b == b':').ok_or(InvalidUrl::Other)?;

        let (scheme, rest) = unsafe {(value.get_unchecked(..i), value.get_unchecked(i+1..))};

        Self::after_scheme(Scheme::new(scheme)?, rest)
    }

    pub(crate) fn after_scheme(scheme: Scheme<'_>, rest: &str) -> Result<Self, InvalidUrl> {
        match scheme.r#type() {
            SchemeType::File           => Self::new_file            (scheme, rest),
            SchemeType::SpecialNotFile => Self::new_special_not_file(scheme, rest),
            SchemeType::NonSpecial     => Self::new_non_special     (scheme, rest),
        }
    }

    /// The URL as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.serialization
    }


    /// The scheme as a [`str`].
    pub fn scheme(&self) -> &str {
        &self.serialization[.. self.scheme_end as usize]
    }

    /// The username as a [`str`], or [`None`] if absent.
    pub fn maybe_username(&self) -> Option<&str> {
        let s = self.scheme_end          as usize + 3;
        let a = self.username_end?.get() as usize    ;
        self.serialization.get(s..a)
    }

    /// The username as a [`str`].
    pub fn username(&self) -> &str {
        self.maybe_username().unwrap_or_default()
    }

    /// The password as a [`str`], or [`None`] if absent.
    pub fn maybe_password(&self) -> Option<&str> {
        let s = self.username_end?.get() as usize + 1;
        let a = self.host_start  ?.get() as usize - 1;
        self.serialization.get(s..a)
    }

    /// The password as a [`str`].
    pub fn password(&self) -> &str {
        self.maybe_password().unwrap_or_default()
    }

    /// The host as a [`str`].
    pub fn host(&self) -> Option<&str> {
        Some(&self.serialization[self.host_start?.get() as usize .. self.host_end?.get() as usize])
    }

    /// The port as a [`u16`].
    pub fn port(&self) -> Option<u16> {
        if self.host_end?.get() == self.path_start {
            None
        } else {
            Some(self.port)
        }
    }

    /// The port as a [`str`].
    pub fn port_str(&self) -> Option<&str> {
        let s = self.host_end?.get() as usize + 1;
        let a = self.path_start      as usize    ;
        match self.serialization.get(s .. a) {
            Some("") => None,
            x => x
        }
    }

    pub fn cannot_be_a_base(&self) -> bool {
        !self.serialization[self.scheme_end as usize..].starts_with(":/")
    }

    pub fn len(&self) -> usize {
        self.serialization.len()
    }
}
