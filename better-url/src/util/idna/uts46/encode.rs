//! Encoding.

use crate::prelude::*;

/// The normalizer.
pub(crate) const NORMALIZER: icu_normalizer::uts46::Uts46MapperBorrowed = icu_normalizer::uts46::Uts46MapperBorrowed::new();

/// Do UTS46 normalization, in-place if possible.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(encode_uts46("a"                  .into()), "a"                  );
/// assert_eq!(encode_uts46("A"                  .into()), "a"                  );
/// assert_eq!(encode_uts46("3"                  .into()), "3"                  );
/// assert_eq!(encode_uts46("-"                  .into()), "-"                  );
/// assert_eq!(encode_uts46("--"                 .into()), "--"                 );
/// assert_eq!(encode_uts46("London"             .into()), "london"             );
/// assert_eq!(encode_uts46("Lloyd-Atkinson"     .into()), "lloyd-atkinson"     );
/// assert_eq!(encode_uts46("This has spaces"    .into()), "this has spaces"    );
/// assert_eq!(encode_uts46("-> $1.00 <-"        .into()), "-> $1.00 <-"        );
/// assert_eq!(encode_uts46("Б"                  .into()), "б"                  );
/// assert_eq!(encode_uts46("ü"                  .into()), "ü"                  );
/// assert_eq!(encode_uts46("α"                  .into()), "α"                  );
/// assert_eq!(encode_uts46("例"                 .into()), "例"                 );
/// assert_eq!(encode_uts46("😉"                 .into()), "😉"                 );
/// assert_eq!(encode_uts46("αβγ"                .into()), "αβγ"                );
/// assert_eq!(encode_uts46("München"            .into()), "münchen"            );
/// assert_eq!(encode_uts46("Mnchen-3ya"         .into()), "mnchen-3ya"         );
/// assert_eq!(encode_uts46("München-Ost"        .into()), "münchen-ost"        );
/// assert_eq!(encode_uts46("Bahnhof München-Ost".into()), "bahnhof münchen-ost");
/// assert_eq!(encode_uts46("abæcdöef"           .into()), "abæcdöef"           );
/// assert_eq!(encode_uts46("Αθήνα"              .into()), "αθήνα"              );
/// assert_eq!(encode_uts46("правда"             .into()), "правда"             );
/// assert_eq!(encode_uts46("ยจฆฟคฏข"            .into()), "ยจฆฟคฏข"            );
/// assert_eq!(encode_uts46("도메인"             .into()), "도메인"             );
/// assert_eq!(encode_uts46("ドメイン名例"       .into()), "ドメイン名例"       );
/// assert_eq!(encode_uts46("MajiでKoiする5秒前" .into()), "majiでkoiする5秒前" );
/// assert_eq!(encode_uts46("「bücher」"         .into()), "「bücher」"         );
/// ```
pub fn encode_uts46(s: Cow<'_, str>) -> Cow<'_, str> {
    let mut mapped = NORMALIZER.map_normalize(s.chars());

    for ((i, x), y) in s.char_indices().zip(&mut mapped) {
        if x != y {
            let mut ret = s[..i].to_string();
            ret.push(y);
            ret.extend(mapped);
            return ret.into();
        }
    }

    drop(mapped);

    s
}

/// Do UTS46 normalization into a [`String`].
pub fn encode_uts46_into<I: IntoIterator<Item = char>>(iter: I, out: &mut String) {
    out.extend(NORMALIZER.map_normalize(iter.into_iter()))
}
