//! UTS-46.

use crate::prelude::*;

/// A UTS46 normalizer.
pub(crate) static UTS46: icu_normalizer::uts46::Uts46MapperBorrowed = icu_normalizer::uts46::Uts46MapperBorrowed::new();

/// Get [`DATA`].
const fn get_data() -> [u8; 256] {
    let mut ret = [0; 256];

    let mut i = b'A';
    while i <= b'Z' {
        ret[i as usize] |= 0b0001;
        i += 1;
    }

    let mut i = b'0';
    while i <= b'9' {
        ret[i as usize] |= 0b0010;
        i += 1;
    }

    let mut i = 128;
    while i < 256 {
        ret[i as usize] |= 0b0100;
        i += 1;
    }

    let mut i = 0;
    while i < 256 {
        if FORBIDDEN_DOMAIN_SEGMENT_INPUT .contains(i as u8) { ret[i as usize] |= 0b0001_0000; }
        if FORBIDDEN_DOMAIN_SEGMENTS_INPUT.contains(i as u8) { ret[i as usize] |= 0b0010_0000; }
        if FORBIDDEN_DOMAIN_HOST_INPUT    .contains(i as u8) { ret[i as usize] |= 0b0100_0000; }
        i += 1;
    }

    ret
}

/// For each byte, `0` if inert, `1` if ASCII uppercase, `2` if non-ASCII.
const DATA: [u8; 256] = get_data();

/// Do UTS46 normalization, in-place if possible.
///
/// See [`uts46_classify_map_normalize`] if you're using this for domain segments/hosts.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(uts46_map_normalize("a"                  ), (false, "a"                  .into()));
/// assert_eq!(uts46_map_normalize("A"                  ), (true , "a"                  .into()));
/// assert_eq!(uts46_map_normalize("3"                  ), (false, "3"                  .into()));
/// assert_eq!(uts46_map_normalize("-"                  ), (false, "-"                  .into()));
/// assert_eq!(uts46_map_normalize("--"                 ), (false, "--"                 .into()));
/// assert_eq!(uts46_map_normalize("London"             ), (true , "london"             .into()));
/// assert_eq!(uts46_map_normalize("Lloyd-Atkinson"     ), (true , "lloyd-atkinson"     .into()));
/// assert_eq!(uts46_map_normalize("This has spaces"    ), (true , "this has spaces"    .into()));
/// assert_eq!(uts46_map_normalize("-> $1.00 <-"        ), (false, "-> $1.00 <-"        .into()));
/// assert_eq!(uts46_map_normalize("Б"                  ), (true , "б"                  .into()));
/// assert_eq!(uts46_map_normalize("ü"                  ), (false, "ü"                  .into()));
/// assert_eq!(uts46_map_normalize("α"                  ), (false, "α"                  .into()));
/// assert_eq!(uts46_map_normalize("例"                 ), (false, "例"                 .into()));
/// assert_eq!(uts46_map_normalize("😉"                 ), (false, "😉"                 .into()));
/// assert_eq!(uts46_map_normalize("αβγ"                ), (false, "αβγ"                .into()));
/// assert_eq!(uts46_map_normalize("München"            ), (true , "münchen"            .into()));
/// assert_eq!(uts46_map_normalize("Mnchen-3ya"         ), (true , "mnchen-3ya"         .into()));
/// assert_eq!(uts46_map_normalize("München-Ost"        ), (true , "münchen-ost"        .into()));
/// assert_eq!(uts46_map_normalize("Bahnhof München-Ost"), (true , "bahnhof münchen-ost".into()));
/// assert_eq!(uts46_map_normalize("abæcdöef"           ), (false, "abæcdöef"           .into()));
/// assert_eq!(uts46_map_normalize("Αθήνα"              ), (true , "αθήνα"              .into()));
/// assert_eq!(uts46_map_normalize("правда"             ), (false, "правда"             .into()));
/// assert_eq!(uts46_map_normalize("ยจฆฟคฏข"            ), (false, "ยจฆฟคฏข"            .into()));
/// assert_eq!(uts46_map_normalize("도메인"             ), (false, "도메인"             .into()));
/// assert_eq!(uts46_map_normalize("ドメイン名例"       ), (false, "ドメイン名例"       .into()));
/// assert_eq!(uts46_map_normalize("MajiでKoiする5秒前" ), (true , "majiでkoiする5秒前" .into()));
/// assert_eq!(uts46_map_normalize("「bücher」"         ), (false, "「bücher」"         .into()));
/// ```
pub fn uts46_map_normalize<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let (changed, value, _) = uts46_classify_map_normalize(value);

    (changed, value)
}

/// Like [`uts46_map_normalize`] but returns additional info about the *input* value in a [`u8`].
///
/// The info is a bitfield where the bits are, from lowest to highest:
///
/// - If an ASCII uppercase letter was found.
///
/// - If an ASCII number was found.
///
/// - If a non-ASCII byte was found.
///
/// - Unused.
///
/// - If a [`FORBIDDEN_DOMAIN_SEGMENT_INPUT`] byte was found.
///
/// - If a [`FORBIDDEN_DOMAIN_SEGMENTS_INPUT`] byte was found.
///
/// - If a [`FORBIDDEN_DOMAIN_HOST_INPUT`] byte was found.
///
/// - Unused.
pub fn uts46_classify_map_normalize<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>, u8) {
    let mut value = value.into();

    let class = value.bytes().fold(0, |acc, b| acc | DATA[b as usize]);

    match class & 0b0000_0101 {
        0b0000 => (false, value, class),
        0b0001 => {
            value.to_mut().make_ascii_lowercase();
            (true, value, class)
        },
        _ => {
            let mut a = value.char_indices();
            let mut b = UTS46.map_normalize(value.chars());

            loop {
                match (a.next(), b.next()) {
                    (None, None) => {
                        drop(b);
                        return (false, value, class);
                    },
                    (None, Some(y)) => {
                        let mut ret = value.to_string();
                        ret.extend(std::iter::once(y).chain(b));
                        return (true, ret.into(), class);
                    },
                    (Some((i, _)), None) => {
                        drop(b);
                        unsafe {
                            value.truncate_unchecked(i);
                        }
                        return (true, value, class);
                    },
                    (Some((i, x)), Some(y)) => if x != y {
                        let mut ret = value[..i].to_string();
                        ret.extend(std::iter::once(y).chain(b));
                        return (true, ret.into(), class);
                    },
                }
            }
        }
    }
}
