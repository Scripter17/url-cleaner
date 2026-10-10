//! Encoding.

use crate::prelude::*;

/// Punycode encode, in-place if possible.
///
/// Please note that, per [Appendic C of RFC 3492](https://datatracker.ietf.org/doc/html/rfc3492#appendix-C), this uses [`usize`]s instead of [`u32`]s.
///
/// In practice this means nothing but someone's going to complain so I figured I'd explain myself early.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(encode_punycode(""  ), ""   );
/// assert_eq!(encode_punycode("a" ), "a-" );
/// assert_eq!(encode_punycode("a-"), "a--");
///
/// assert_eq!(encode_punycode("ليهمابتكلموشعربي؟"                                                       ), "egbpdaj6bu4bxfgehfvwxn"                                               );
/// assert_eq!(encode_punycode("他们为什么不说中文"                                                      ), "ihqwcrb4cv8a8dqg056pqjye"                                             );
/// assert_eq!(encode_punycode("他們爲什麽不說中文"                                                      ), "ihqwctvzc91f659drss3x8bo0yb"                                          );
/// assert_eq!(encode_punycode("Pročprostěnemluvíčesky"                                                  ), "Proprostnemluvesky-uyb24dma41a"                                       );
/// assert_eq!(encode_punycode("למההםפשוטלאמדבריםעברית"                                                  ), "4dbcagdahymbxekheh6e0a7fei0b"                                         );
/// assert_eq!(encode_punycode("यहलोगहिन\u{94d}दीक\u{94d}यो\u{902}नही\u{902}बोलसकत\u{947}ह\u{948}\u{902}"), "i1baa7eci9glrd9b2ae1bj0hfcgg6iyaf8o0a1dig0cd"                         );
/// assert_eq!(encode_punycode("なぜみんな日本語を話してくれないのか"                                    ), "n8jok5ay5dzabd5bym9f0cm5685rrjetr6pdxa"                               );
/// assert_eq!(encode_punycode("세계의모든사람들이한국어를이해한다면얼마나좋을까"                        ), "989aomsvi5e83db1d2a355cv1e0vak1dwrv93d5xbh15a0dt30a5jpsd879ccm6fea98c");
/// assert_eq!(encode_punycode("почемужеонинеговорятпорусски"                                            ), "b1abfaaepdrnnbgefbadotcwatmq2g4l"                                     );
/// assert_eq!(encode_punycode("PorquénopuedensimplementehablarenEspañol"                                ), "PorqunopuedensimplementehablarenEspaol-fmd56a"                        );
/// assert_eq!(encode_punycode("TạisaohọkhôngthểchỉnóitiếngViệt"                                         ), "TisaohkhngthchnitingVit-kjcr8268qyxafd2f1b9g"                         );
/// ```
pub fn encode_punycode<'a, T: Into<Cow<'a, str>>>(value: T) -> Cow<'a, str> {
    let mut value = value.into();

    if value.is_ascii() {
        if !value.is_empty() {
            value.push_str("-");
        }

        value
    } else {
        let mut out = String::new();
        encode_punycode_into(&value, &mut out);
        out.into()
    }
}

/// Punycode encode into a [`String`].
///
/// Please note that, per [Appendic C of RFC 3492](https://datatracker.ietf.org/doc/html/rfc3492#appendix-C), this uses [`usize`]s instead of [`u32`]s.
///
/// In practice this means nothing but someone's going to complain so I figured I'd explain myself early.
pub fn encode_punycode_into<'a>(value: &str, out: &'a mut String) -> &'a str {
    encode_punycode_into_bytes(value, unsafe {out.as_mut_vec()})
}

/// Punycode encode into a [`Vec`] of bytes.
///
/// Please note that, per [Appendic C of RFC 3492](https://datatracker.ietf.org/doc/html/rfc3492#appendix-C), this uses [`usize`]s instead of [`u32`]s.
///
/// In practice this means nothing but someone's going to complain so I figured I'd explain myself early.
pub fn encode_punycode_into_bytes<'a>(value: &str, out: &'a mut Vec<u8>) -> &'a str {
    let mut delta         = 0usize;
    let mut bias          = 72usize;
    let mut last_look     = '\u{7F}';
    let mut first         = true;
    let mut insert_points = 1usize;

    let new_len = out.len();
    out.reserve(value.len());

    for b in value.bytes() {
        if b.is_ascii() {
            out.push(b);
            insert_points += 1;
        }
    }

    if insert_points != 1 {
        out.push(b'-');
    }

    while let Some(look) = value.chars().filter(|&c| c > last_look).min() {
        delta += (look as usize - last_look as usize - 1) * insert_points;
        last_look = look;

        for c in value.chars() {
            if c < look {
                delta += 1;
            }

            if c == look {
                let mut q = delta;

                for i in 1usize.. {
                    let threshold = (i * 36).saturating_sub(bias).clamp(1, 26);

                    if q < threshold {
                        out.push(q as u8 + b'a');
                        break;
                    }

                    let digit = threshold + (q - threshold) % (36 - threshold);
                        q     =             (q - threshold) / (36 - threshold);

                    out.push(match digit as u8 {
                        digit @ ..26 => digit      + b'a',
                        digit @ 26.. => digit - 26 + b'0',
                    });
                }



                match first {
                    true  => delta /= 700,
                    false => delta /= 2,
                }

                delta += delta / insert_points;

                bias = 36;

                while delta > 455 {
                    delta /= 35;
                    bias  += 36;
                }

                bias -= (delta + 1405) / (delta + 38);



                insert_points += 1;
                delta = 0;
                first = false;
            }
        }

        delta += 1;
    }

    unsafe {
        str::from_utf8_unchecked(out.get_unchecked(new_len..))
    }
}
