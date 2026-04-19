//! Encoding.

use crate::prelude::*;

/// Encode a domain, in-place if possible.
///
/// Please note that there are no checks to ensure the input is a valid domain.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// let domains = [
///     "abc.com",
///     "abc.com.",
///     "Αθήνα.abc.Αθήνα",
///     "Αθήνα.abc.Αθήνα.",
/// ];
///
/// for domain in domains {
///     assert_eq!(encode_domain(domain.into()), url::Host::parse(domain).unwrap().to_string());
/// }
/// ```
pub fn encode_domain(domain: Cow<'_, str>) -> Cow<'_, str> {
    if domain.is_ascii() {
        domain
    } else {
        let mut ret = String::new();

        for (i, label) in domain.split('.').enumerate() {
            if i != 0 {
                ret.push('.');
            }

            match label.is_ascii() {
                true  => ret.push_str(label),
                false => encode_label_into(label.chars(), &mut ret)
            }
        }

        ret.into()
    }
}

/// Encode a domain into a [`String`].
///
/// Please note that there are no checks to ensure the input is a valid domain.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// let domains = [
///     "abc.com",
///     "abc.com.",
///     "Αθήνα.abc.Αθήνα",
///     "Αθήνα.abc.Αθήνα.",
/// ];
///
/// for domain in domains {
///     let mut out = String::new();
///     encode_domain_into(domain.chars(), &mut out);
///     assert_eq!(out, url::Host::parse(domain).unwrap().to_string());
/// }
/// ```
pub fn encode_domain_into<I: IntoIterator<Item = char>>(iter: I, out: &mut String) {
    encode_domain_into_bytes(iter, unsafe {out.as_mut_vec()})
}

/// Encode a domain into a [`Vec`] of bytes.
///
/// Please note that there are no checks to ensure the input is a valid domain.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// let domains = [
///     "abc.com",
///     "abc.com.",
///     "Αθήνα.abc.Αθήνα",
///     "Αθήνα.abc.Αθήνα.",
/// ];
///
/// for domain in domains {
///     let mut out = Vec::new();
///     encode_domain_into_bytes(domain.chars(), &mut out);
///     assert_eq!(out, url::Host::parse(domain).unwrap().to_string().as_bytes());
/// }
/// ```
pub fn encode_domain_into_bytes<I: IntoIterator<Item = char>>(iter: I, out: &mut Vec<u8>) {
    // [`std::iter::Peekable`] should have a method like [`Iterator::take_while`] that doesn't consume the stopper.

    let mut iter = iter.into_iter().chain(['.']).peekable();

    loop {
        encode_label_into_bytes((&mut iter).take_while(|&b| b != '.'), out);

        match iter.peek() {
            Some(_) => out.push(b'.'),
            None => break
        }
    }
}
