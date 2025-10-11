//! Segment utilities.

use crate::util::*;

/// Set the `index`th segment to `value`.
/// # Errors
/// If `index` is out of range, returns `segment_not_found` as an error.
pub fn set_segment<E>(segments: &str, split: &str, index: isize, value: Option<&str>, segment_not_found: E) -> Result<Option<String>, E> {
    let mut x = segments.split(split);
    let len = x.clone().count();
    let index = neg_index(index, len).ok_or(segment_not_found)?;
    Ok(if len == 1 && value.is_none() {
        None
    } else {
        let replace = x.nth(index).expect("The index to be in-bounds.");
        match (index, value) {
            (0, None) => segments.get(replace.len() + split.len() ..).map(Into::into),
            (_, None) => Some(format!(
                "{}{}",
                segments.get(.. replace.as_ptr().addr() - segments.as_ptr().addr() - split  .len()   ).expect("The substring to be in-bounds."),
                segments.get(   replace.as_ptr().addr() - segments.as_ptr().addr() + replace.len() ..).expect("The substring to be in-bounds.")
            )),
            (_, Some(value)) => Some(format!(
                "{}{value}{}",
                segments.get(.. replace.as_ptr().addr() - segments.as_ptr().addr()                   ).expect("The substring to be in-bounds."),
                segments.get(   replace.as_ptr().addr() - segments.as_ptr().addr() + replace.len() ..).expect("The substring to be in-bounds.")
            ))
        }
    })
}

/// Insert a new segment such that the segment at `index` is `value`, assuming `value` doesn't contain `split`.
/// # Errors
/// If `index` is out of range, returns `segment_not_found` as an error.
pub fn insert_segment<E>(segments: &str, split: &str, index: isize, value: &str, segment_not_found: E) -> Result<String, E> {
    let mut x = segments.split(split);
    let len = x.clone().count();
    let index = match index {
        0.. if index as usize <= len => index as usize,
        ..0 => len.checked_add_signed(index + 1).ok_or(segment_not_found)?,
        _ => Err(segment_not_found)?
    };
    Ok(if index == 0 {
        format!("{value}{split}{segments}")
    } else if index == len {
        format!("{segments}{split}{value}")
    } else {
        let next = x.nth(index).expect("The index to be in-bounds.");
        format!(
            "{}{split}{value}{}",
            segments.get(.. next.as_ptr().addr() - segments.as_ptr().addr() - split.len()   ).expect("The substring to be in-bounds."),
            segments.get(   next.as_ptr().addr() - segments.as_ptr().addr() - split.len() ..).expect("The substring to be in-bounds.")
        )
    })
}

/// Remove the first `n` segments of `s` split by `split`.
pub fn char_remove_first_n_segments(s: &str, split: char, n: usize) -> Option<&str> {
    s.get((s.split(split).nth(n)? as *const str).addr() - (s as *const str).addr() ..)
}

/// Keep the first `n` segments of `s` split by `split`.
pub fn char_keep_first_n_segments(s: &str, split: char, n: usize) -> Option<&str> {
    if n == 0 {
        None
    } else {
        let seg = s.split(split).nth(n-1)?;
        s.get(.. (seg as *const str).addr() + seg.len() - (s as *const str).addr())
    }
}

/// Remove the last `n` segments of `s` split by `split`.
pub fn char_remove_last_n_segments(s: &str, split: char, n: usize) -> Option<&str> {
    let seg = s.split(split).nth_back(n)?;
    s.get(.. (seg as *const str).addr() + seg.len() - (s as *const str).addr())
}

/// Keep the last `n` segments of `s` split by `split`.
pub fn char_keep_last_n_segments(s: &str, split: char, n: usize) -> Option<&str> {
    if n == 0 {
        None
    } else {
        s.get((s.split(split).nth_back(n - 1)? as *const str).addr() - (s as *const str).addr()..)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_segment() {
        let test = "aa-bb-cc";

        assert_eq!(set_segment(test, "-", -4, Some(".."), ()), Err(()));
        assert_eq!(set_segment(test, "-", -3, Some(".."), ()), Ok(Some("..-bb-cc".into())));
        assert_eq!(set_segment(test, "-", -2, Some(".."), ()), Ok(Some("aa-..-cc".into())));
        assert_eq!(set_segment(test, "-", -1, Some(".."), ()), Ok(Some("aa-bb-..".into())));
        assert_eq!(set_segment(test, "-",  0, Some(".."), ()), Ok(Some("..-bb-cc".into())));
        assert_eq!(set_segment(test, "-",  1, Some(".."), ()), Ok(Some("aa-..-cc".into())));
        assert_eq!(set_segment(test, "-",  2, Some(".."), ()), Ok(Some("aa-bb-..".into())));
        assert_eq!(set_segment(test, "-",  3, Some(".."), ()), Err(()));

        assert_eq!(set_segment(test, "-", -4, None, ()), Err(()));
        assert_eq!(set_segment(test, "-", -3, None, ()), Ok(Some("bb-cc".into())));
        assert_eq!(set_segment(test, "-", -2, None, ()), Ok(Some("aa-cc".into())));
        assert_eq!(set_segment(test, "-", -1, None, ()), Ok(Some("aa-bb".into())));
        assert_eq!(set_segment(test, "-",  0, None, ()), Ok(Some("bb-cc".into())));
        assert_eq!(set_segment(test, "-",  1, None, ()), Ok(Some("aa-cc".into())));
        assert_eq!(set_segment(test, "-",  2, None, ()), Ok(Some("aa-bb".into())));
        assert_eq!(set_segment(test, "-",  3, None, ()), Err(()));

        let test = "aa";

        assert_eq!(set_segment(test, "-", -2, Some(".."), ()), Err(()));
        assert_eq!(set_segment(test, "-", -1, Some(".."), ()), Ok(Some("..".into())));
        assert_eq!(set_segment(test, "-",  0, Some(".."), ()), Ok(Some("..".into())));
        assert_eq!(set_segment(test, "-",  1, Some(".."), ()), Err(()));

        assert_eq!(set_segment(test, "-", -2, None, ()), Err(()));
        assert_eq!(set_segment(test, "-", -1, None, ()), Ok(None));
        assert_eq!(set_segment(test, "-",  0, None, ()), Ok(None));
        assert_eq!(set_segment(test, "-",  1, None, ()), Err(()));
    }

    #[test]
    fn test_insert_segment() {
        let test = "aa-bb-cc-dd-ee";

        assert_eq!(insert_segment(test, "-", -7, "..", ()), Err(()));
        assert_eq!(insert_segment(test, "-", -6, "..", ()), Ok("..-aa-bb-cc-dd-ee".into()));
        assert_eq!(insert_segment(test, "-", -5, "..", ()), Ok("aa-..-bb-cc-dd-ee".into()));
        assert_eq!(insert_segment(test, "-", -4, "..", ()), Ok("aa-bb-..-cc-dd-ee".into()));
        assert_eq!(insert_segment(test, "-", -3, "..", ()), Ok("aa-bb-cc-..-dd-ee".into()));
        assert_eq!(insert_segment(test, "-", -2, "..", ()), Ok("aa-bb-cc-dd-..-ee".into()));
        assert_eq!(insert_segment(test, "-", -1, "..", ()), Ok("aa-bb-cc-dd-ee-..".into()));
        assert_eq!(insert_segment(test, "-",  0, "..", ()), Ok("..-aa-bb-cc-dd-ee".into()));
        assert_eq!(insert_segment(test, "-",  1, "..", ()), Ok("aa-..-bb-cc-dd-ee".into()));
        assert_eq!(insert_segment(test, "-",  2, "..", ()), Ok("aa-bb-..-cc-dd-ee".into()));
        assert_eq!(insert_segment(test, "-",  3, "..", ()), Ok("aa-bb-cc-..-dd-ee".into()));
        assert_eq!(insert_segment(test, "-",  4, "..", ()), Ok("aa-bb-cc-dd-..-ee".into()));
        assert_eq!(insert_segment(test, "-",  5, "..", ()), Ok("aa-bb-cc-dd-ee-..".into()));
        assert_eq!(insert_segment(test, "-",  6, "..", ()), Err(()));

        let test = "";
        assert_eq!(insert_segment(test, "-", -3, "..", ()), Err(()));
        assert_eq!(insert_segment(test, "-", -2, "..", ()), Ok("..-".into()));
        assert_eq!(insert_segment(test, "-", -1, "..", ()), Ok("-..".into()));
        assert_eq!(insert_segment(test, "-",  0, "..", ()), Ok("..-".into()));
        assert_eq!(insert_segment(test, "-",  1, "..", ()), Ok("-..".into()));
        assert_eq!(insert_segment(test, "-",  2, "..", ()), Err(()));

        let test = "aa";
        assert_eq!(insert_segment(test, "-", -3, "..", ()), Err(()));
        assert_eq!(insert_segment(test, "-", -2, "..", ()), Ok("..-aa".into()));
        assert_eq!(insert_segment(test, "-", -1, "..", ()), Ok("aa-..".into()));
        assert_eq!(insert_segment(test, "-",  0, "..", ()), Ok("..-aa".into()));
        assert_eq!(insert_segment(test, "-",  1, "..", ()), Ok("aa-..".into()));
        assert_eq!(insert_segment(test, "-",  2, "..", ()), Err(()));
    }

    #[test]
    fn test_char_remove_first_n_segments() {
        let test = "aa-bb-cc-dd-ee";

        assert_eq!(char_remove_first_n_segments(test, '-', 0), Some("aa-bb-cc-dd-ee"));
        assert_eq!(char_remove_first_n_segments(test, '-', 1), Some("bb-cc-dd-ee"));
        assert_eq!(char_remove_first_n_segments(test, '-', 2), Some("cc-dd-ee"));
        assert_eq!(char_remove_first_n_segments(test, '-', 3), Some("dd-ee"));
        assert_eq!(char_remove_first_n_segments(test, '-', 4), Some("ee"));
        assert_eq!(char_remove_first_n_segments(test, '-', 5), None);
    }

    #[test]
    fn test_char_keep_first_n_segments() {
        let test = "aa-bb-cc-dd-ee";

        assert_eq!(char_keep_first_n_segments(test, '-', 0), None);
        assert_eq!(char_keep_first_n_segments(test, '-', 1), Some("aa"));
        assert_eq!(char_keep_first_n_segments(test, '-', 2), Some("aa-bb"));
        assert_eq!(char_keep_first_n_segments(test, '-', 3), Some("aa-bb-cc"));
        assert_eq!(char_keep_first_n_segments(test, '-', 4), Some("aa-bb-cc-dd"));
        assert_eq!(char_keep_first_n_segments(test, '-', 5), Some("aa-bb-cc-dd-ee"));
        assert_eq!(char_keep_first_n_segments(test, '-', 6), None);
    }

    #[test]
    fn test_char_remove_last_n_segments() {
        let test = "aa-bb-cc-dd-ee";

        assert_eq!(char_remove_last_n_segments(test, '-', 0), Some("aa-bb-cc-dd-ee"));
        assert_eq!(char_remove_last_n_segments(test, '-', 1), Some("aa-bb-cc-dd"));
        assert_eq!(char_remove_last_n_segments(test, '-', 2), Some("aa-bb-cc"));
        assert_eq!(char_remove_last_n_segments(test, '-', 3), Some("aa-bb"));
        assert_eq!(char_remove_last_n_segments(test, '-', 4), Some("aa"));
        assert_eq!(char_remove_last_n_segments(test, '-', 5), None);
    }

    #[test]
    fn test_char_keep_last_n_segments() {
        let test = "aa-bb-cc-dd-ee";

        assert_eq!(char_keep_last_n_segments(test, '-', 0), None);
        assert_eq!(char_keep_last_n_segments(test, '-', 1), Some("ee"));
        assert_eq!(char_keep_last_n_segments(test, '-', 2), Some("dd-ee"));
        assert_eq!(char_keep_last_n_segments(test, '-', 3), Some("cc-dd-ee"));
        assert_eq!(char_keep_last_n_segments(test, '-', 4), Some("bb-cc-dd-ee"));
        assert_eq!(char_keep_last_n_segments(test, '-', 5), Some("aa-bb-cc-dd-ee"));
        assert_eq!(char_keep_last_n_segments(test, '-', 6), None);
    }
}

