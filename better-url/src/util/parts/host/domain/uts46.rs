//! UTS-46.

use crate::prelude::*;

/// A UTS46 normalizer.
pub(crate) static UTS46: icu_normalizer::uts46::Uts46MapperBorrowed = icu_normalizer::uts46::Uts46MapperBorrowed::new();

/// Do UTS46 mapping and normalization, in-place if possible.
///
/// Effectively a wrapper [`icu_normalizer::uts46::Uts46MapperBorrowed`] that should in theory be faster for ASCII input.
///
/// See [`uts46_classify_map_normalize`] if you're using this for domain/IPv4 stuff.
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
#[inline]
pub fn uts46_map_normalize<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let (changed, value, _) = uts46_classify_map_normalize(value);

    (changed, value)
}

/// Like [`uts46_map_normalize`] but returns the "class" of the input.
///
/// The "class" is the result of the bitwise OR of each byte's value in [`DOMAIN_BYTE_CLASS_DATA`].
///
/// ***Please*** note that this is about the ***input***.
///
/// If the unicode bit is set, you can't assume the output will be described by the class.
///
/// This function exists because:
///
/// 1. To fast path ASCII input, this function already has to compute the "contains Unicode" and "contains ASCII uppercase" bits of the class.
///
/// 2. It is free to compute up to 8 bits of information.
///
/// 3. The extra computed bits are useful elsewhere for additional fast paths.
#[inline]
pub fn uts46_classify_map_normalize<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>, u8) {
    let mut value = value.into();
    let class = value.bytes().fold(0, |acc, b| acc | DOMAIN_BYTE_CLASS_DATA[b as usize]);

    match class & 0b101 {
        0b000 => (false, value, class),
        0b001 => {
            value.to_mut().make_ascii_lowercase();
            (true, value, class)
        },
        _ => {
            #[cold]
            fn outline<'a>(value: Cow<'a, str>, class: u8) -> (bool, Cow<'a, str>, u8) {
                // By using the fact that `.` is "inert" under NFC we can do UTS46 normalization purely per-segment.
                // It does involve effectively computing the input's class twice but it's fine.
                // Credit to Servo for this trick.

                let mut ret = String::new();
                let mut changed = false;

                for segment in SplitDots(Some(&value)) {
                    if changed {
                        ret.push('.');
                    }

                    let segment_class = segment.bytes().fold(0, |acc, b| acc | DOMAIN_BYTE_CLASS_DATA[b as usize]);

                    match segment_class & 0b101 {
                        0b000 => if changed {ret.push_str(segment)},
                        0b001 => {
                            if changed {
                                ret.push_str(segment);
                            } else {
                                ret.reserve(value.len());
                                ret.push_str(&value[.. segment.end_addr() - value.addr()]);
                                changed = true;
                            }
                        },
                        _ => 'a: {
                            let mut a = segment;
                            let mut b = UTS46.map_normalize(segment.chars());

                            while let Some(c) = b.next() {
                                match a.strip_prefix(c) {
                                    Some(x) => a = x,
                                    None => {
                                        if changed {
                                            ret.push_str(&segment[..a.addr() - segment.addr()]);
                                        } else {
                                            ret.reserve(value.len());
                                            ret.push_str(&value[.. a.addr() - value.addr()]);
                                            changed = true;
                                        }

                                        ret.push(c);
                                        ret.extend(b);

                                        break 'a;
                                    }
                                }
                            }

                            match a.is_empty() {
                                true  => if changed {ret.push_str(segment);},
                                false => {
                                    if changed {
                                        ret.push_str(&segment[..a.addr() - segment.addr()]);
                                    } else {
                                        ret.reserve(value.len());
                                        ret.push_str(&value[.. a.addr() - value.addr()]);
                                        changed = true;
                                    }
                                }
                            }
                        }
                    }
                }

                if class & 1 == 1 {
                    ret.make_ascii_lowercase();
                }

                match changed {
                    false => (false, value     , class),
                    true  => (true , ret.into(), class),
                }
            }

            outline(value, class)
        }
    }
}
