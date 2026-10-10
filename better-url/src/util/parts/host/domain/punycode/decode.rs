//! [`decode_punycode`].

use crate::prelude::*;

/// Decode Punycode.
///
/// Please note that, per [Appendic C of RFC 3492](https://datatracker.ietf.org/doc/html/rfc3492#appendix-C), this uses [`usize`]s instead of [`u32`]s.
///
/// In practice this means nothing but someone's going to complain so I figured I'd explain myself early.
/// # Errors
/// If [`decode_punycode_into`] returns an error, that error is returned.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(decode_punycode("jxafb0a0a" ).unwrap(), "αθήνα");
/// assert_eq!(decode_punycode("pwa2dj0a0a").unwrap(), "Αθήνα");
///
/// assert_eq!(decode_punycode("egbpdaj6bu4bxfgehfvwxn"                                               ).unwrap(), "ليهمابتكلموشعربي؟"                                                       );
/// assert_eq!(decode_punycode("ihqwcrb4cv8a8dqg056pqjye"                                             ).unwrap(), "他们为什么不说中文"                                                      );
/// assert_eq!(decode_punycode("ihqwctvzc91f659drss3x8bo0yb"                                          ).unwrap(), "他們爲什麽不說中文"                                                      );
/// assert_eq!(decode_punycode("Proprostnemluvesky-uyb24dma41a"                                       ).unwrap(), "Pročprostěnemluvíčesky"                                                  );
/// assert_eq!(decode_punycode("4dbcagdahymbxekheh6e0a7fei0b"                                         ).unwrap(), "למההםפשוטלאמדבריםעברית"                                                  );
/// assert_eq!(decode_punycode("i1baa7eci9glrd9b2ae1bj0hfcgg6iyaf8o0a1dig0cd"                         ).unwrap(), "यहलोगहिन\u{94d}दीक\u{94d}यो\u{902}नही\u{902}बोलसकत\u{947}ह\u{948}\u{902}");
/// assert_eq!(decode_punycode("n8jok5ay5dzabd5bym9f0cm5685rrjetr6pdxa"                               ).unwrap(), "なぜみんな日本語を話してくれないのか"                                    );
/// assert_eq!(decode_punycode("989aomsvi5e83db1d2a355cv1e0vak1dwrv93d5xbh15a0dt30a5jpsd879ccm6fea98c").unwrap(), "세계의모든사람들이한국어를이해한다면얼마나좋을까"                        );
/// assert_eq!(decode_punycode("b1abfaaepdrnnbgefbadotcwatmq2g4l"                                     ).unwrap(), "почемужеонинеговорятпорусски"                                            );
/// assert_eq!(decode_punycode("PorqunopuedensimplementehablarenEspaol-fmd56a"                        ).unwrap(), "PorquénopuedensimplementehablarenEspañol"                                );
/// assert_eq!(decode_punycode("TisaohkhngthchnitingVit-kjcr8268qyxafd2f1b9g"                         ).unwrap(), "TạisaohọkhôngthểchỉnóitiếngViệt"                                         );
///
/// assert_eq!(decode_punycode("--rmckbbajlc6dj7bxne2c").unwrap(), "وزارة-الأتصالات");
/// ```
pub fn decode_punycode(value: &str) -> Result<String, InvalidPunycode> {
    let mut ret = String::new();

    decode_punycode_into(value, &mut ret)?;

    Ok(ret)
}

/// Decode Punycode into an existing [`String`] and return a [`str`] to it.
///
/// Please note that, per [Appendic C of RFC 3492](https://datatracker.ietf.org/doc/html/rfc3492#appendix-C), this uses [`usize`]s instead of [`u32`]s.
///
/// In practice this means nothing but someone's going to complain so I figured I'd explain myself early.
/// # Errors
/// If `value` is invalid Punycode, returns the error [`InvalidPunycode`].
///
/// If an error is returend, `out` is left unmodified.
pub fn decode_punycode_into<'a>(value: &str, out: &'a mut String) -> Result<&'a str, InvalidPunycode> {
    let (base, input) = match value.memrchr(b'-') {
        Some(i) => (&value[..i], &value[i+1..]),
        None    => (&value[..0],  value       ),
    };

    if !base.is_ascii() || input.bytes().any(|b| !b.is_ascii_alphanumeric()) {
        Err(InvalidPunycode)?;
    }

    let mut insertions = smallvec::SmallVec::<[(usize, char); 10]>::new();

    let mut insert_points = base.len() + 1;
    let mut code_point    = 0x80usize;
    let mut bias          = 72usize;
    let mut iter          = input.bytes();
    let mut reserve       = base.len();
    let mut new_decode    = 0usize;
    let mut first         = true;
    let mut delta         = 0usize;

    while let Some(mut byte) = iter.next() {
        let mut weight = 1;

        for i in 1usize.. {
            let digit = match byte {
                b'0'..=b'9' => byte - b'0' + 26,
                b'a'..=b'z' => byte - b'a',
                b'A'..=b'Z' => byte - b'A',
                _ => unsafe {std::hint::unreachable_unchecked()},
            };

            delta += (digit as usize) * weight;

            let t = (i * 36).saturating_sub(bias).clamp(1, 26) as u8;

            if digit < t {
                break;
            }

            weight *= 36 - t as usize;
            byte = iter.next().ok_or(InvalidPunycode)?;
        }



        new_decode += delta;

        code_point += new_decode / insert_points;

        let insert_i = new_decode % insert_points;

        let insert_c = char::from_u32(code_point as u32).ok_or(InvalidPunycode)?;

        reserve += insert_c.len_utf8();

        let mut insert_n = insertions.len();

        for (i, (idx, _)) in insertions.iter_mut().enumerate().rev() {
            if *idx < insert_i {
                break;
            }

            insert_n = i;
            *idx += 1;
        }

        insertions.insert(insert_n, (insert_i, insert_c));

        new_decode = insert_i + 1;



        match first {
            true  => delta /= 700,
            false => delta /= 2,
        }

        delta += delta / insert_points;

        bias = 0;

        while delta > 455 {
            delta /= 35;
            bias  += 36;
        }

        bias += (delta * 36) / (delta + 38);



        insert_points += 1;
        delta = 0;
        first = false;
    }



    let new_start = out.len();

    out.reserve(reserve);



    if base.is_empty() {
        out.extend(insertions.into_iter().map(|(_, c)| c));
    } else {
        let mut old_read_idx = 0;

        for (i, (idx, c)) in insertions.into_iter().enumerate() {
            let read_idx = idx - i;

            out.push_str(&base[old_read_idx .. read_idx]);

            old_read_idx = read_idx;

            out.push(c);
        }

        out.push_str(&base[old_read_idx..]);
    }



    Ok(&out[new_start..])
}
