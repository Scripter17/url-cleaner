//! [`Ipv6Host`] stuff.

use std::fmt::Write;
use std::net::Ipv6Addr;

use crate::prelude::*;

/// Make an [`Ipv6Host`].
///
/// Please note that you can also make an [`Ipv6Host`] directly from an existing [`Ipv6Addr`].
/// # Errors
/// If `value` is not a valid [`Ipv6Host`], returns the error [`InvalidIpv6Host`].
/// # Examples
/// ```
/// use std::assert_matches;
/// use std::borrow::Cow;
///
/// use better_url::util::*;
///
/// assert_matches!(make_ipv6_host("[1:2::3]"  ).unwrap(), (false, Cow::Borrowed(x), _) if x == "[1:2::3]");
/// assert_matches!(make_ipv6_host("[1:2:0::3]").unwrap(), (true , Cow::Owned   (x), _) if x == "[1:2::3]");
///
/// // The URL spec and Rust's standard library differ here.
///
/// assert_matches!(make_ipv6_host("[::ffff:127.0.0.1]").unwrap(), (true, Cow::Owned(x), _) if x == "[::ffff:7f00:1]");
///
/// assert_eq!(format!("[{}]", "::ffff:127.0.0.1".parse::<std::net::Ipv6Addr>().unwrap()), "[::ffff:127.0.0.1]");
/// ```
#[expect(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
pub fn make_ipv6_host<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>, Ipv6HostDetails), InvalidIpv6Host> {
    let value = value.into();

    let addr = match value.as_bytes() {
        [b'[', addr @ .., b']'] => unsafe {str::from_utf8_unchecked(addr)}.parse::<Ipv6Addr>().map_err(|_| InvalidIpv6Host)?,
        _ => Err(InvalidIpv6Host)?,
    };

    let mut normalizer = Normalizer::new(value);
    write_ipv6_host(&addr, &mut normalizer).expect("???");

    let (changed, host) = normalizer.done();

    Ok((changed, host, addr.into()))
}



/// Make an [`Ipv6Host`] from bytes.
///
/// Please note that you can also make an [`Ipv6Host`] directly from an existing [`Ipv6Addr`].
/// # Errors
/// If `value` is not a valid [`Ipv6Host`], returns the error [`InvalidIpv6Host`].
/// # Examples
/// ```
/// use std::assert_matches;
/// use std::borrow::Cow;
///
/// use better_url::util::*;
///
/// assert_matches!(make_ipv6_host_bytes(b"[1:2::3]"  ).unwrap(), (false, Cow::Borrowed(x), _) if x == "[1:2::3]");
/// assert_matches!(make_ipv6_host_bytes(b"[1:2:0::3]").unwrap(), (true , Cow::Owned   (x), _) if x == "[1:2::3]");
///
/// // The URL spec and Rust's standard library differ here.
///
/// assert_matches!(make_ipv6_host_bytes(b"[::ffff:127.0.0.1]").unwrap(), (true, Cow::Owned(x), _) if x == "[::ffff:7f00:1]");
///
/// assert_eq!(format!("[{}]", "::ffff:127.0.0.1".parse::<std::net::Ipv6Addr>().unwrap()), "[::ffff:127.0.0.1]");
/// ```
#[expect(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
pub fn make_ipv6_host_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> Result<(bool, Cow<'a, str>, Ipv6HostDetails), InvalidIpv6Host> {
    let value = value.into();

    if !value.is_ascii() {
        Err(InvalidIpv6Host)?;
    }

    let value = unsafe {cow_bytes_to_str_unchecked(value)};

    let addr = match value.as_bytes() {
        [b'[', addr @ .., b']'] => unsafe {str::from_utf8_unchecked(addr)}.parse::<Ipv6Addr>().map_err(|_| InvalidIpv6Host)?,
        _ => Err(InvalidIpv6Host)?,
    };

    let mut normalizer = Normalizer::new(value);
    write_ipv6_host(&addr, &mut normalizer).expect("???");

    let (changed, host) = normalizer.done();

    Ok((changed, host, addr.into()))
}



/// [`std::fmt::Write::write_str`]'s an [`Ipv6Addr`] into an [`Ipv6Host`] literal.
/// # Errors
/// If `f`'s call to [`Write::write_str`] returns an error, that error is returned.
pub fn write_ipv6_host<F: Write>(addr: &Ipv6Addr, f: &mut F) -> std::fmt::Result {
    /// A stack allocated buffer that can just store the longest [`Ipv6Host`] literal.
    struct Thing {
        /// The buffer.
        ///
        /// No Ipv6Host literal can exceed 4*8+7+2=41 bytes.
        buf: [u8; 41],
        /// The length.
        len: usize,
    }

    impl Thing {
        /// Get the buffer as a string
        #[inline]
        fn as_str(&self) -> &str {
            // SAFETY: Only called in [`write_ipv6_host`], which enforces the invariants.
            unsafe {
                str::from_utf8_unchecked(self.buf.get_unchecked(..self.len))
            }
        }

        /// Push a single byte.
        #[inline]
        fn push(&mut self, b: u8) {
            // SAFETY: Only called in [`write_ipv6_host`], which enforces the invariants.
            unsafe {
                *self.buf.as_mut_ptr().add(self.len) = b;
            }

            self.len += 1;
        }

        /// Push a [`str`].
        #[inline]
        fn push_str(&mut self, s: &str) {
            // SAFETY: Only called in [`write_ipv6_host`], which enforces the invariants.
            unsafe {
                std::ptr::copy_nonoverlapping(s.as_ptr(), self.buf.as_mut_ptr().add(self.len), s.len());
            }

            self.len += s.len();
        }
    }

    let segments = addr.segments();
    let (x, y) = find_ipv6_elision(&segments);

    let mut temp = Thing {buf: [0; 41], len: 0};

    // Before the elision.

    temp.push(b'[');

    for i in 0..x {
        if i != 0 {
            temp.push(b':');
        }

        temp.push_str(write_ipv6_num(segments[i as usize], &mut [0; 4]));
    }

    // The elision.

    if x != y {
        temp.push(b':');
        temp.push(b':');
    }

    // After the elision.

    for i in y..8 {
        temp.push_str(write_ipv6_num(segments[i as usize], &mut [0; 4]));

        if i != 7 {
            temp.push(b':');
        }
    }

    temp.push(b']');

    // Write to the output buffer.

    f.write_str(temp.as_str())
}

/// Find the start and end indices of the elision of an IPv6 address.
///
/// Returns `(0,0)` if no elision.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(find_ipv6_elision(&[0,0,0,1,1,0,0,0]), (0, 3));
/// assert_eq!(find_ipv6_elision(&[0,0,1,1,1,0,0,0]), (5, 8));
/// assert_eq!(find_ipv6_elision(&[0,1,1,1,1,1,1,0]), (0, 0));
/// ```
pub fn find_ipv6_elision(pieces: &[u16; 8]) -> (u8, u8) {
    // Make a u8 where each bit is 1 iff its segment is 0.
    // I think SIMD can do this faster but that's hard.

    let mut x = 0u8;

    for &piece in pieces {
        x = (x << 1) | (piece == 0) as u8;
    }

    // Repeatedly sink the right coast of each island of 1-bits until the next iteration would result in no islands.

    // 01110110 -> 01100100 -> 01000000
    // - Start at 1
    // - 2 iterations = length of 3

    // 01110111 -> 01100110 -> 01000100
    // - Start at 1 because it's the first island
    // - 2 iterations = length of 3

    let mut i = 0;

    while x & x << 1 != 0 {
        x &= x << 1;

        i += 1;
    }

    // If i = 0, then there were no elidable islands.
    // If i > 0, then the elision is of length i + 1.

    if i == 0 {
        (0, 0)
    } else {
        let index  = x.leading_zeros() as u8;
        let length = i + 1;

        (index, index + length)
    }
}

/// Write a [`u16`] as a lower hexadecimal number much faster than using [`format`] with `{:x}`.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// let mut buf = [0; 4];
///
/// assert_eq!(write_ipv6_num(0x0000, &mut buf),    "0");
/// assert_eq!(write_ipv6_num(0x000a, &mut buf),    "a");
/// assert_eq!(write_ipv6_num(0x00a0, &mut buf),   "a0");
/// assert_eq!(write_ipv6_num(0x0a00, &mut buf),  "a00");
/// assert_eq!(write_ipv6_num(0xa000, &mut buf), "a000");
/// assert_eq!(write_ipv6_num(0xabcd, &mut buf), "abcd");
/// ```
pub fn write_ipv6_num(x: u16, buf: &mut [u8; 4]) -> &str {
    /// The characters to use for each lowercase hex digit.
    const NIBBLES_LOWER: [u8; 16] = *b"0123456789abcdef";

    buf[3] = NIBBLES_LOWER[(x as usize      ) & 0xF];
    buf[2] = NIBBLES_LOWER[(x as usize >>  4) & 0xF];
    buf[1] = NIBBLES_LOWER[(x as usize >>  8) & 0xF];
    buf[0] = NIBBLES_LOWER[(x as usize >> 12) & 0xF];

    unsafe {
        str::from_utf8_unchecked(buf.get_unchecked((x | 1).leading_zeros() as usize / 4 ..))
    }
}
