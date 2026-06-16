//! Encoding.

use crate::prelude::*;

/// Punycode encode, in-place if possible.
/// # Errors
/// If the call to [`encode_punycode_into`] returns an error, that error is returned.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(encode_punycode(""  ).unwrap(), ""   );
/// assert_eq!(encode_punycode("a" ).unwrap(), "a-" );
/// assert_eq!(encode_punycode("a-").unwrap(), "a--");
///
/// assert_eq!(encode_punycode("ليهمابتكلموشعربي؟"                                                       ).unwrap(), "egbpdaj6bu4bxfgehfvwxn"                                               );
/// assert_eq!(encode_punycode("他们为什么不说中文"                                                      ).unwrap(), "ihqwcrb4cv8a8dqg056pqjye"                                             );
/// assert_eq!(encode_punycode("他們爲什麽不說中文"                                                      ).unwrap(), "ihqwctvzc91f659drss3x8bo0yb"                                          );
/// assert_eq!(encode_punycode("Pročprostěnemluvíčesky"                                                  ).unwrap(), "Proprostnemluvesky-uyb24dma41a"                                       );
/// assert_eq!(encode_punycode("למההםפשוטלאמדבריםעברית"                                                  ).unwrap(), "4dbcagdahymbxekheh6e0a7fei0b"                                         );
/// assert_eq!(encode_punycode("यहलोगहिन\u{94d}दीक\u{94d}यो\u{902}नही\u{902}बोलसकत\u{947}ह\u{948}\u{902}").unwrap(), "i1baa7eci9glrd9b2ae1bj0hfcgg6iyaf8o0a1dig0cd"                         );
/// assert_eq!(encode_punycode("なぜみんな日本語を話してくれないのか"                                    ).unwrap(), "n8jok5ay5dzabd5bym9f0cm5685rrjetr6pdxa"                               );
/// assert_eq!(encode_punycode("세계의모든사람들이한국어를이해한다면얼마나좋을까"                        ).unwrap(), "989aomsvi5e83db1d2a355cv1e0vak1dwrv93d5xbh15a0dt30a5jpsd879ccm6fea98c");
/// assert_eq!(encode_punycode("почемужеонинеговорятпорусски"                                            ).unwrap(), "b1abfaaepdrnnbgefbadotcwatmq2g4l"                                     );
/// assert_eq!(encode_punycode("PorquénopuedensimplementehablarenEspañol"                                ).unwrap(), "PorqunopuedensimplementehablarenEspaol-fmd56a"                        );
/// assert_eq!(encode_punycode("TạisaohọkhôngthểchỉnóitiếngViệt"                                         ).unwrap(), "TisaohkhngthchnitingVit-kjcr8268qyxafd2f1b9g"                         );
/// ```
pub fn encode_punycode<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<Cow<'a, str>, TooLong> {
    let mut value = value.into();

    Ok(if value.is_ascii() {
        if !value.is_empty() {
            value.to_mut().push('-');
        }

        value
    } else {
        let mut out = String::new();
        encode_punycode_into(value.chars(), &mut out)?;
        out.into()
    })
}

/// Punycode encode into a [`String`].
///
/// Returnx the number of ASCII codepoints and non-ASCII codepoints.
/// # Errors
/// If the call to [`encode_punycode_into_bytes`] returns an error, that error is returned.
pub fn encode_punycode_into<I: IntoIterator<Item = char>>(iter: I, out: &mut String) -> Result<(u32, u32), TooLong> {
    encode_punycode_into_bytes(iter, unsafe {out.as_mut_vec()})
}

/// Punycode encode into a [`Vec`] of bytes.
///
/// Returnx the number of ASCII codepoints and non-ASCII codepoints.
/// # Errors
/// If there are at least [`u32::MAX`], returns the error [`TooLong`].
///
/// Please note that `out` still has all the ASCII bytes appended to it.
pub fn encode_punycode_into_bytes<I: IntoIterator<Item = char>>(iter: I, out: &mut Vec<u8>) -> Result<(u32, u32), TooLong> {
    let mut delta    = 0;
    let mut bias     = 72;
    let mut look     = 127;
    let mut first    = true;
    let mut numpoint = 1;
    let mut ascii    = 0;
    let mut unicode  = 0;

    let mut input  = Vec::new();
    let mut looks  = std::collections::BTreeSet::new();

    for (i, c) in iter.into_iter().enumerate() {
        if i == u32::MAX as usize {
            Err(TooLong)?;
        }

        input.push(c as u32);
        match c.is_ascii() {
            true  => {out.push(c as u8); numpoint += 1; ascii += 1;},
            false => {looks.insert(c as u32); unicode += 1;},
        }
    }

    if numpoint != 1 {
        out.push(b'-');
    }

    for new_look in looks {
        delta += (new_look - look - 1) * numpoint;
        look = new_look;

        for &c in &input {
            if c < look {
                delta += 1;
            }

            if c == look {
                let mut q = delta;

                for i in 1u32.. {
                    let t = (i * 36).saturating_sub(bias).clamp(1, 26);

                    if q < t {
                        out.push(q as u8 + 97);
                        break;
                    }

                    let digit = t + (q - t) % (36 - t);

                    let encoded = match digit {
                        ..26 => digit + 97,
                        _    => digit + 22,
                    };

                    out.push(encoded as u8);

                    q = (q - t) / (36 - t);
                }



                match first {
                    true  => delta /= 700,
                    false => delta /= 2,
                }

                delta += delta / numpoint;

                bias = 36;

                while delta > 455 {
                    delta /= 35;
                    bias += 36;
                }

                bias -= (delta + 1405) / (delta + 38);



                numpoint += 1;
                delta = 0;
                first = false;
            }
        }

        delta += 1;
    }

    Ok((ascii, unicode))
}
