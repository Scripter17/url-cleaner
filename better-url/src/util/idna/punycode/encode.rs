//! Encoding.

use crate::prelude::*;

/// Punycode encode, in-place if possible.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(punycode("a"                  .into()), "a-"                        );
/// assert_eq!(punycode("A"                  .into()), "A-"                        );
/// assert_eq!(punycode("3"                  .into()), "3-"                        );
/// assert_eq!(punycode("-"                  .into()), "--"                        );
/// assert_eq!(punycode("--"                 .into()), "---"                       );
/// assert_eq!(punycode("London"             .into()), "London-"                   );
/// assert_eq!(punycode("Lloyd-Atkinson"     .into()), "Lloyd-Atkinson-"           );
/// assert_eq!(punycode("This has spaces"    .into()), "This has spaces-"          );
/// assert_eq!(punycode("-> $1.00 <-"        .into()), "-> $1.00 <--"              );
/// assert_eq!(punycode("Б"                  .into()), "d0a"                       );
/// assert_eq!(punycode("ü"                  .into()), "tda"                       );
/// assert_eq!(punycode("α"                  .into()), "mxa"                       );
/// assert_eq!(punycode("例"                 .into()), "fsq"                       );
/// assert_eq!(punycode("😉"                 .into()), "n28h"                      );
/// assert_eq!(punycode("αβγ"                .into()), "mxacd"                     );
/// assert_eq!(punycode("München"            .into()), "Mnchen-3ya"                );
/// assert_eq!(punycode("Mnchen-3ya"         .into()), "Mnchen-3ya-"               );
/// assert_eq!(punycode("München-Ost"        .into()), "Mnchen-Ost-9db"            );
/// assert_eq!(punycode("Bahnhof München-Ost".into()), "Bahnhof Mnchen-Ost-u6b"    );
/// assert_eq!(punycode("abæcdöef"           .into()), "abcdef-qua4k"              );
/// assert_eq!(punycode("Αθήνα"              .into()), "pwa2dj0a0a"                );
/// assert_eq!(punycode("правда"             .into()), "80aafi6cg"                 );
/// assert_eq!(punycode("ยจฆฟคฏข"            .into()), "22cdfh1b8fsa"              );
/// assert_eq!(punycode("도메인"             .into()), "hq1bm8jm9l"                );
/// assert_eq!(punycode("ドメイン名例"       .into()), "eckwd4c7cu47r2wf"          );
/// assert_eq!(punycode("MajiでKoiする5秒前" .into()), "MajiKoi5-783gue6qz075azm5e");
/// assert_eq!(punycode("「bücher」"         .into()), "bcher-kva8445foa"          );
///
/// assert_eq!(punycode("ليهمابتكلموشعربي؟"                                                       .into()), "egbpdaj6bu4bxfgehfvwxn"                                               );
/// assert_eq!(punycode("他们为什么不说中文"                                                      .into()), "ihqwcrb4cv8a8dqg056pqjye"                                             );
/// assert_eq!(punycode("他們爲什麽不說中文"                                                      .into()), "ihqwctvzc91f659drss3x8bo0yb"                                          );
/// assert_eq!(punycode("Pročprostěnemluvíčesky"                                                  .into()), "Proprostnemluvesky-uyb24dma41a"                                       );
/// assert_eq!(punycode("למההםפשוטלאמדבריםעברית"                                                  .into()), "4dbcagdahymbxekheh6e0a7fei0b"                                         );
/// assert_eq!(punycode("यहलोगहिन\u{94d}दीक\u{94d}यो\u{902}नही\u{902}बोलसकत\u{947}ह\u{948}\u{902}".into()), "i1baa7eci9glrd9b2ae1bj0hfcgg6iyaf8o0a1dig0cd"                         );
/// assert_eq!(punycode("なぜみんな日本語を話してくれないのか"                                    .into()), "n8jok5ay5dzabd5bym9f0cm5685rrjetr6pdxa"                               );
/// assert_eq!(punycode("세계의모든사람들이한국어를이해한다면얼마나좋을까"                        .into()), "989aomsvi5e83db1d2a355cv1e0vak1dwrv93d5xbh15a0dt30a5jpsd879ccm6fea98c");
/// assert_eq!(punycode("почемужеонинеговорятпорусски"                                            .into()), "b1abfaaepdrnnbgefbadotcwatmq2g4l"                                     );
/// assert_eq!(punycode("PorquénopuedensimplementehablarenEspañol"                                .into()), "PorqunopuedensimplementehablarenEspaol-fmd56a"                        );
/// assert_eq!(punycode("TạisaohọkhôngthểchỉnóitiếngViệt"                                         .into()), "TisaohkhngthchnitingVit-kjcr8268qyxafd2f1b9g"                         );
/// ```
pub fn punycode(mut s: Cow<'_, str>) -> Cow<'_, str> {
    if s.is_ascii() {
        if !s.is_empty() {
            s.to_mut().push('-');
        }

        s
    } else {
        let mut out = String::new();
        punycode_into(s.chars(), &mut out);
        out.into()
    }
}

/// Punycode encode into a [`String`].
///
/// Returnx the number of ASCII codepoints and non-ASCII codepoints.
pub fn punycode_into<I: IntoIterator<Item = char>>(iter: I, out: &mut String) -> (usize, usize) {
    punycode_into_bytes(iter, unsafe {out.as_mut_vec()})
}

/// Punycode encode into a [`Vec`] of bytes.
///
/// Returnx the number of ASCII codepoints and non-ASCII codepoints.
pub fn punycode_into_bytes<I: IntoIterator<Item = char>>(iter: I, out: &mut Vec<u8>) -> (usize, usize) {
    let mut delta    = 0;
    let mut bias     = 72;
    let mut look     = 127;
    let mut first    = true;
    let mut numpoint = 1;
    let mut ascii    = 0;
    let mut unicode  = 0;

    let mut input  = Vec::new();
    let mut looks  = std::collections::BTreeSet::new();

    for c in iter {
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
        delta += (new_look - look - 1) as usize * numpoint;
        look = new_look;

        for &c in input.iter() {
            if c < look {
                delta += 1;
            }

            if c == look {
                let mut q = delta;

                for i in 1usize.. {
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

    (ascii, unicode)
}
