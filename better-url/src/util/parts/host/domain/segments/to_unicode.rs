//! Decoding.

use crate::prelude::*;

/// [Domain to Unicode](https://url.spec.whatwg.org/#concept-domain-to-unicode).
///
/// If your input is in UTS46, see [`uts46_domain_segments_to_unicode`].
/// # Errors
/// If [`uts46_domain_segments_to_unicode`] returns an error, that error is returned.
///
/// Note that, per the spec, the decoded value is still returned. It's just done in a way where you can't ignore errors.
/// # Examples
/// ```
/// use better_url::prelude::*;
/// use better_url::util::*;
///
/// assert_eq!(domain_segments_to_unicode("abc"                ), Ok(("abc"                .into(), None                    , BidiDetail::Ltr     )));
///
/// assert_eq!(domain_segments_to_unicode("abc.⭐"             ), Ok(("abc.⭐"             .into(), None                    , BidiDetail::ForceLtr)));
/// assert_eq!(domain_segments_to_unicode("abc.xn--f7i"        ), Ok(("abc.xn--f7i"        .into(), Some("abc.⭐"   .into()), BidiDetail::ForceLtr)));
///
/// assert_eq!(domain_segments_to_unicode("abc.⭐.⭐"          ), Ok(("abc.⭐.⭐"          .into(), None                    , BidiDetail::ForceLtr)));
/// assert_eq!(domain_segments_to_unicode("abc.xn--f7i.⭐"     ), Ok(("abc.xn--f7i.⭐"     .into(), Some("abc.⭐.⭐".into()), BidiDetail::ForceLtr)));
/// assert_eq!(domain_segments_to_unicode("abc.⭐.xn--f7i"     ), Ok(("abc.⭐.xn--f7i"     .into(), Some("abc.⭐.⭐".into()), BidiDetail::ForceLtr)));
/// assert_eq!(domain_segments_to_unicode("abc.xn--f7i.xn--f7i"), Ok(("abc.xn--f7i.xn--f7i".into(), Some("abc.⭐.⭐".into()), BidiDetail::ForceLtr)));
///
/// assert_eq!(domain_segments_to_unicode("www.ل🌲.com"    ), Err(("www.ل🌲.com"    .into(), None                     )));
/// assert_eq!(domain_segments_to_unicode("www.ل🌲.xn--f7i"), Err(("www.ل🌲.xn--f7i".into(), Some("www.ل🌲.⭐".into()))));
/// ```
#[expect(clippy::type_complexity, reason = "It's fine.")]
pub fn domain_segments_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(Cow<'a, str>, Option<String>, BidiDetail), (Cow<'a, str>, Option<String>)> {
    let (_, value) = uts46_map_normalize(value);

    uts46_domain_segments_to_unicode(value)
}

/// [`domain_segments_to_unicode`], assuming the input has been through [`uts46_map_normalize`].
/// # Errors
/// If [`uts46_domain_segment_to_unicode`] returns either [`BidiDetail::ForceAscii`] or an error, returns an error.
///
/// Note that, per the spec, the decoded value is still returned. It's just done in a way where you can't ignore errors.
#[expect(clippy::type_complexity, reason = "It's fine.")]
pub fn uts46_domain_segments_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(Cow<'a, str>, Option<String>, BidiDetail), (Cow<'a, str>, Option<String>)> {
    let value = value.into();
    let mut changed = false;
    let mut sum_bidi_detail = BidiDetail::Ltr;

    let mut segments = SplitDots(Some(&value)).peekable();

    let mut ret = String::new();

    while let Some(segment) = segments.next() {
        if sum_bidi_detail == BidiDetail::ForceAscii {
            let (raw_segment, unicode_segment) = unchecked_uts46_domain_segment_to_unicode(segment);

            if !changed && unicode_segment.is_some() {
                ret = value[..raw_segment.as_ptr().addr() - value.as_ptr().addr()].to_string();
                changed = true;
            }

            if changed {
                ret.push_str(&unicode_segment.map_or(raw_segment, Into::into));

                if segments.peek().is_some() {
                    ret.push('.');
                }
            }
        } else {
            match uts46_domain_segment_to_unicode(segment) {
                Ok((raw_segment, unicode_segment, bidi_detail)) => {
                    if !changed && unicode_segment.is_some() {
                        ret = value[..raw_segment.as_ptr().addr() - value.as_ptr().addr()].to_string();
                        changed = true;
                    }

                    if changed {
                        ret.push_str(&unicode_segment.map_or(raw_segment, Into::into));

                        if segments.peek().is_some() {
                            ret.push('.');
                        }
                    }

                    sum_bidi_detail = sum_bidi_detail.join(bidi_detail);
                },
                Err((raw_segment, unicode_segment)) => {
                    if !changed && unicode_segment.is_some() {
                        ret = value[..raw_segment.as_ptr().addr() - value.as_ptr().addr()].to_string();
                        changed = true;
                    }

                    if changed {
                        ret.push_str(&unicode_segment.map_or(raw_segment, Into::into));

                        if segments.peek().is_some() {
                            ret.push('.');
                        }
                    }

                    sum_bidi_detail = BidiDetail::ForceAscii;
                },
            }
        }
    }

    match (changed, sum_bidi_detail) {
        (false, BidiDetail::ForceAscii) => Err((value, None                      )),
        (false, _                     ) => Ok ((value, None     , sum_bidi_detail)),
        (true , BidiDetail::ForceAscii) => Err((value, Some(ret)                 )),
        (true , _                     ) => Ok ((value, Some(ret), sum_bidi_detail)),
    }
}
