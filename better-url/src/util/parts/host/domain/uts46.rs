//! UTS-46.

use crate::prelude::*;

/// Do UTS46 normalization, in-place if possible.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(uts46_normalize("a"                  ).1, "a"                  );
/// assert_eq!(uts46_normalize("A"                  ).1, "a"                  );
/// assert_eq!(uts46_normalize("3"                  ).1, "3"                  );
/// assert_eq!(uts46_normalize("-"                  ).1, "-"                  );
/// assert_eq!(uts46_normalize("--"                 ).1, "--"                 );
/// assert_eq!(uts46_normalize("London"             ).1, "london"             );
/// assert_eq!(uts46_normalize("Lloyd-Atkinson"     ).1, "lloyd-atkinson"     );
/// assert_eq!(uts46_normalize("This has spaces"    ).1, "this has spaces"    );
/// assert_eq!(uts46_normalize("-> $1.00 <-"        ).1, "-> $1.00 <-"        );
/// assert_eq!(uts46_normalize("Б"                  ).1, "б"                  );
/// assert_eq!(uts46_normalize("ü"                  ).1, "ü"                  );
/// assert_eq!(uts46_normalize("α"                  ).1, "α"                  );
/// assert_eq!(uts46_normalize("例"                 ).1, "例"                 );
/// assert_eq!(uts46_normalize("😉"                 ).1, "😉"                 );
/// assert_eq!(uts46_normalize("αβγ"                ).1, "αβγ"                );
/// assert_eq!(uts46_normalize("München"            ).1, "münchen"            );
/// assert_eq!(uts46_normalize("Mnchen-3ya"         ).1, "mnchen-3ya"         );
/// assert_eq!(uts46_normalize("München-Ost"        ).1, "münchen-ost"        );
/// assert_eq!(uts46_normalize("Bahnhof München-Ost").1, "bahnhof münchen-ost");
/// assert_eq!(uts46_normalize("abæcdöef"           ).1, "abæcdöef"           );
/// assert_eq!(uts46_normalize("Αθήνα"              ).1, "αθήνα"              );
/// assert_eq!(uts46_normalize("правда"             ).1, "правда"             );
/// assert_eq!(uts46_normalize("ยจฆฟคฏข"            ).1, "ยจฆฟคฏข"            );
/// assert_eq!(uts46_normalize("도메인"             ).1, "도메인"             );
/// assert_eq!(uts46_normalize("ドメイン名例"       ).1, "ドメイン名例"       );
/// assert_eq!(uts46_normalize("MajiでKoiする5秒前" ).1, "majiでkoiする5秒前" );
/// assert_eq!(uts46_normalize("「bücher」"         ).1, "「bücher」"         );
/// ```
pub fn uts46_normalize<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();

    if value.is_ascii() {
        if value.bytes().any(|b| b.is_ascii_uppercase()) {
            value.to_mut().make_ascii_lowercase();
            return (true, value);
        } else {
            return (false, value);
        }
    }

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
