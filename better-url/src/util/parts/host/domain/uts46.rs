//! UTS-46.

use crate::prelude::*;

/// A UTS46 normalizer.
pub(crate) static UTS46: icu_normalizer::uts46::Uts46MapperBorrowed = icu_normalizer::uts46::Uts46MapperBorrowed::new();

/// Do UTS46 normalization, in-place if possible.
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
    let mut value = value.into();

    if value.is_ascii() {
        if value.bytes().any(|b| b.is_ascii_uppercase()) {
            value.to_mut().make_ascii_lowercase();
            (true, value)
        } else {
            (false, value)
        }
    } else {
        let mut a = value.char_indices();
        let mut b = UTS46.map_normalize(value.chars());

        loop {
            match (a.next(), b.next()) {
                (None, None) => {
                    drop(b);
                    return (false, value);
                }
                (None, Some(y)) => {
                    let mut ret = value.to_string();
                    ret.extend(std::iter::once(y).chain(b));
                    return (true, ret.into());
                },
                (Some((i, _)), None) => {
                    drop(b);
                    value.retain_range(..i);
                    return (true, value);
                },
                (Some((i, x)), Some(y)) => if x != y {
                    let mut ret = value[..i].to_string();
                    ret.extend(std::iter::once(y).chain(b));
                    return (true, ret.into());
                },
            }
        }
    }
}
