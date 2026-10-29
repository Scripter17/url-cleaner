//! [`FileHost`].

use std::fmt::Write;

use crate::prelude::*;

/// Make a [`FileHost`].
/// # Errors
/// If `value` is an invalid [`FileHost`], returns the error [`InvalidFileHost`].
pub fn make_file_host<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>, FileHostDetails), InvalidFileHost> {
    let value = value.into();

    Ok(match value.as_bytes() {
        b""        => (false, value, EmptyHostDetails.into()),
        [b'[', ..] => {
            let (changed, value, ip) = make_ipv6_host(value)?;

            (changed, value, ip.into())
        },
        _ => {
            let (a, value       ) = try_percent_decode(value).map_err(|_| InvalidFileHost)?;
            let (b, value, class) = uts46_classify_map_normalize(value);

            match value.as_bytes() {
                b""          => Err(InvalidFileHost)?,
                b"localhost" => (false, "".into(), EmptyHostDetails.into()),
                _ => {
                    if class & 0b0001_0100 == 0b0001_0000 {
                        Err(InvalidFileHost)?
                    } else if class & 0b0000_0110 != 0b0000_0000 && ends_in_a_number(&value) {
                        let addr = parse_ipv4_addr(&value).ok_or(InvalidIpv4Host)?;

                        let mut normalizer = Normalizer::new(value);
                        write!(normalizer, "{addr}").expect("???");

                        let (c, value) = normalizer.done();

                        (a || b || c, value, addr.into())
                    } else if class & 0b0001_0110 == 0b0000_0000 && !value.is_empty() {
                        let details = DomainHostDetails::parse_unchecked(&value);

                        (a || b, value, details.into())
                    } else {
                        let (raw, ascii) = uts46_domain_host_to_ascii(value, false)?;

                        let c = ascii.is_some();

                        let value = ascii.map_or(raw, Into::into);

                        if b && value.bytes().any(|b| FORBIDDEN_DOMAIN_HOST_LITERAL.contains(b)) {
                            Err(InvalidDomainHost)?;
                        }

                        if value.is_empty() {
                            Err(InvalidDomainHost)?
                        }

                        let details = DomainHostDetails::parse_unchecked(&value);

                        (a || b || c, value, details.into())
                    }
                }
            }
        }
    })
}

/// Make a [`FileHost`] from bytes.
/// # Errors
/// If `value` is an invalid [`FileHost`], returns the error [`InvalidFileHost`].
pub fn make_file_host_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> Result<(bool, Cow<'a, str>, FileHostDetails), InvalidFileHost> {
    let value = value.into();

    Ok(match &*value {
        b""        => (false, unsafe {cow_bytes_to_str_unchecked(value)}, EmptyHostDetails.into()),
        [b'[', ..] => {
            let (changed, value, ip) = make_ipv6_host_bytes(value)?;

            (changed, value, ip.into())
        },
        _ => {
            let (a, value       ) = try_percent_decode_bytes(value).map_err(|_| InvalidFileHost)?;
            let (b, value, class) = uts46_classify_map_normalize(value);

            match value.as_bytes() {
                b""          => Err(InvalidFileHost)?,
                b"localhost" => (false, "".into(), EmptyHostDetails.into()),
                _ => {
                    if class & 0b0001_0100 == 0b0001_0000 {
                        Err(InvalidFileHost)?
                    } else if class & 0b0000_0110 != 0b0000_0000 && ends_in_a_number(&value) {
                        let addr = parse_ipv4_addr(&value).ok_or(InvalidIpv4Host)?;

                        let mut normalizer = Normalizer::new(value);
                        write!(normalizer, "{addr}").expect("???");

                        let (c, value) = normalizer.done();

                        (a || b || c, value, addr.into())
                    } else if class & 0b0001_0110 == 0b0000_0000 && !value.is_empty() {
                        let details = DomainHostDetails::parse_unchecked(&value);

                        (a || b, value, details.into())
                    } else {
                        let (raw, ascii) = uts46_domain_host_to_ascii(value, false)?;

                        let c = ascii.is_some();

                        let value = ascii.map_or(raw, Into::into);

                        if value.bytes().any(|b| FORBIDDEN_DOMAIN_HOST_LITERAL.contains(b)) {
                            Err(InvalidDomainHost)?;
                        }

                        if value.is_empty() {
                            Err(InvalidDomainHost)?
                        }

                        let details = DomainHostDetails::parse_unchecked(&value);

                        (a || b || c, value, details.into())
                    }
                }
            }
        }
    })
}
