//! [`decode_punycode`].

use std::iter::{Peekable, Enumerate, Copied};

use crate::prelude::*;

/// Decode Punycode.
/// # Errors
/// If the input is invalid, returns the error [`InvalidPunycode`].
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
/// ```
pub fn decode_punycode(value: &str) -> Result<String, InvalidPunycode> {
    decode_punycode_bytes(value.as_bytes())
}

/// Decode punycode bytes.
/// # Errors
/// If the input is invalid, returns the error [`InvalidPunycode`].
pub fn decode_punycode_bytes(value: &[u8]) -> Result<String, InvalidPunycode> {
    Ok(decode_punycode_base_bytes(value)?.collect())
}

/// An [`Iterator`] over the [`char`]s of a Punycode decoding.
#[derive(Debug)]
struct PunycodeDecoder<'a> {
    /// The base chars.
    bcs: Peekable<Enumerate<Copied<std::slice::Iter<'a, u8>>>>,
    /// The insert chars.
    ics: Peekable<Enumerate<std::vec::IntoIter<(u32, char)>>>,
}

impl Iterator for PunycodeDecoder<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.bcs.peek(), self.ics.peek()) {
            (Some(&(bi, bc)), Some(&(a, (ii, _)))) if bi + a < ii as usize => {
                let _ = self.bcs.next();
                Some(bc as char)
            },

            (Some(&(_ , bc)), None                ) => {let _ = self.bcs.next(); Some(bc as char)},
            (_              , Some(&(_, (_ , ic)))) => {let _ = self.ics.next(); Some(ic)},

            (None, None) => None,
        }
    }
}

/// Decode Punycode encoded bytes into a [`PunycodeDecoder`].
fn decode_punycode_base_bytes(value: &[u8]) -> Result<PunycodeDecoder<'_>, InvalidPunycode> {
    if value.len() > u32::MAX as usize {
        Err(InvalidPunycode)?;
    }

    if !value.is_ascii() {
        Err(InvalidPunycode)?;
    }

    let mut insertions = Vec::<(u32, char)>::new();

    let (base, input) = match value.iter().position(|b| *b == b'-') {
        Some(i) => (&value[..i], &value[i+1..]),
        None    => (&value[..0],  value       ),
    };

    let mut length     = base.len() as u32;
    let mut code_point = 0x80u32;
    let mut bias       = 72u32;
    let mut i          = 0u32;
    let mut iter       = input.iter().copied();

    while let Some(mut byte) = iter.next() {
        let previous_i = i;
        let mut weight = 1;
        let mut k = 36u32;

        loop {
            let digit = match byte {
                b'0'..=b'9' => byte - b'0' + 26,
                b'a'..=b'z' => byte - b'a',
                b'A'..=b'Z' => byte - b'A',
                _ => Err(InvalidPunycode)?
            };
            let product = (digit as u32).checked_mul(weight).ok_or(InvalidPunycode)?;

            i = i.checked_add(product).ok_or(InvalidPunycode)?;

            let t = k.saturating_sub(bias).clamp(1, 26) as u8;

            if digit < t {
                break;
            }

            weight = weight.checked_mul(36 - t as u32).ok_or(InvalidPunycode)?;
            k += 36;
            byte = iter.next().ok_or(InvalidPunycode)?;
        }

        let mut delta = i - previous_i;

        match previous_i {
            0 => delta /= 700,
            _ => delta /= 2
        }

        delta += delta / (length + 1);

        bias = 0;

        while delta > 455 {
            delta /= 35;
            bias  += 36;
        }

        bias += (delta * 36) / (delta + 38);

        code_point = code_point.checked_add(i / (length + 1)).ok_or(InvalidPunycode)?;

        i %= length + 1;
        let c = char::from_u32(code_point).ok_or(InvalidPunycode)?;

        let mut a = None;

        for (b, (idx, _)) in insertions.iter_mut().enumerate() {
            if *idx >= i {
                if a.is_none() {
                    a = Some(b);
                }
                *idx += 1;
            }
        }

        insertions.insert(a.unwrap_or(insertions.len()), (i, c));

        length += 1;
        i += 1;
    }

    Ok(PunycodeDecoder {
        bcs: base.iter().copied().enumerate().peekable(),
        ics: insertions.into_iter().enumerate().peekable(),
    })
}
