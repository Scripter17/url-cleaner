//! Segment utilities.

use crate::util::*;

/// [`set_segment`] but returns a [`String`].
///
/// If the call to [`set_segment`] returns an empty list, returns [`None`].
/// # Errors
/// If the call to [`set_segment`] returns an error, that error is returned.
pub(crate) fn set_segment_str<E>(part: &str, index: isize, value: Option<&str>, segment_not_found: E, split: char, join: &str) -> Result<Option<String>, E> {
    let segments = set_segment(part, index, value, segment_not_found, split)?;
    Ok(if segments.is_empty() {
        None
    } else {
        Some(segments.join(join))
    })
}

/// Sets the specified segment of `part` to `value`, or removes it if `value` is [`None`].
/// # Errors
/// If the call to [`neg_index`] returns [`None`], returns the error provided in `segment_not_found`.
fn set_segment<'a, E>(part: &'a str, index: isize, value: Option<&'a str>, segment_not_found: E, split: char) -> Result<Vec<&'a str>, E> {
    let mut segments = part.split(split).collect::<Vec<_>>();
    let index = neg_index(index, segments.len()).ok_or(segment_not_found)?;
    match value {
        #[expect(clippy::indexing_slicing, reason = "Can't happen.")]
        Some(value) => segments[index] = value,
        None => {segments.remove(index);}
    }
    Ok(segments)
}

use std::iter::Peekable;

/// Basic bodge to do [`std::iter::Intersperse`] while we wait another 6 years for its stabilization.
#[derive(Debug, Clone)]
struct Intersperse<T: Clone, I: Iterator<Item = T>> {
    /// The iterator.
    iter: Peekable<I>,
    /// The thing to intersperse with.
    x: T,
    /// If [`true`], return [`Self::x`]. If [`false`], return [`Self::iter`]'s [`Iterator::next`].
    y: bool
}

fn intersperse<T: Clone, I: Iterator<Item = T>>(iter: I, x: T) -> Intersperse<T, I> {
    Intersperse {
        iter: iter.peekable(),
        x,
        y: false,
    }
}

impl<T: Clone, I: Iterator<Item = T>> Iterator for Intersperse<T, I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.peek()?;
        self.y = !self.y;
        if self.y {
            self.iter.next()
        } else {
           Some(self.x.clone())
        }
    }
}

/// Helper method.
///
/// Assumes `split` is one byte but this is only called with `.` and `/` so who cares.
pub(crate) fn insert_segment<E>(part: &str, index: isize, value: &str, segment_not_found: E, split: char, join: &str) -> Result<String, E> {
    println!("{part} {index}");
    let x = part.split(split);
    let len = x.clone().count();
    let index = match index {
        0.. if index as usize <= len => index as usize,
        #[expect(clippy::arithmetic_side_effects, reason = "A negative number plus one never overflows.")]
        ..0 => len.checked_add_signed(index + 1).ok_or(segment_not_found)?,
        _ => Err(segment_not_found)?
    };
    Ok(intersperse(x.clone().take(index).chain(std::iter::once(value)).chain(x.skip(index)), join).collect::<String>())
}

/// Remove the first `n` segments of `s` split by `split`.
pub(crate) fn char_remove_first_n_segments(s: &str, split: char, n: usize) -> Option<&str> {
    #[allow(clippy::arithmetic_side_effects, reason = "Can't happen.")]
    s.get((s.split(split).nth(n)? as *const str).addr() - (s as *const str).addr() ..)
}

/// Keep the first `n` segments of `s` split by `split`.
pub(crate) fn char_keep_first_n_segments(s: &str, split: char, n: usize) -> Option<&str> {
    if n == 0 {
        None
    } else {
        #[allow(clippy::arithmetic_side_effects, reason = "Can't happen.")]
        let seg = s.split(split).nth(n-1)?;
        #[allow(clippy::arithmetic_side_effects, reason = "Can't happen.")]
        s.get(.. (seg as *const str).addr() + seg.len() - (s as *const str).addr())
    }
}

/// Remove the last `n` segments of `s` split by `split`.
pub(crate) fn char_remove_last_n_segments(s: &str, split: char, n: usize) -> Option<&str> {
    let seg = s.split(split).nth_back(n)?;
    #[allow(clippy::arithmetic_side_effects, reason = "Can't happen.")]
    s.get(.. (seg as *const str).addr() + seg.len() - (s as *const str).addr())
}

/// Keep the last `n` segments of `s` split by `split`.
pub(crate) fn char_keep_last_n_segments(s: &str, split: char, n: usize) -> Option<&str> {
    if n == 0 {
        None
    } else {
        #[allow(clippy::arithmetic_side_effects, reason = "Can't happen.")]
        s.get((s.split(split).nth_back(n - 1)? as *const str).addr() - (s as *const str).addr()..)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_segment() {
        let test = "aa-bb-cc-dd-ee";

        assert_eq!(insert_segment(test, -7, "..", (), '-', "-"), Err(()));
        assert_eq!(insert_segment(test, -6, "..", (), '-', "-"), Ok("..-aa-bb-cc-dd-ee".into()));
        assert_eq!(insert_segment(test, -5, "..", (), '-', "-"), Ok("aa-..-bb-cc-dd-ee".into()));
        assert_eq!(insert_segment(test, -4, "..", (), '-', "-"), Ok("aa-bb-..-cc-dd-ee".into()));
        assert_eq!(insert_segment(test, -3, "..", (), '-', "-"), Ok("aa-bb-cc-..-dd-ee".into()));
        assert_eq!(insert_segment(test, -2, "..", (), '-', "-"), Ok("aa-bb-cc-dd-..-ee".into()));
        assert_eq!(insert_segment(test, -1, "..", (), '-', "-"), Ok("aa-bb-cc-dd-ee-..".into()));
        assert_eq!(insert_segment(test,  0, "..", (), '-', "-"), Ok("..-aa-bb-cc-dd-ee".into()));
        assert_eq!(insert_segment(test,  1, "..", (), '-', "-"), Ok("aa-..-bb-cc-dd-ee".into()));
        assert_eq!(insert_segment(test,  2, "..", (), '-', "-"), Ok("aa-bb-..-cc-dd-ee".into()));
        assert_eq!(insert_segment(test,  3, "..", (), '-', "-"), Ok("aa-bb-cc-..-dd-ee".into()));
        assert_eq!(insert_segment(test,  4, "..", (), '-', "-"), Ok("aa-bb-cc-dd-..-ee".into()));
        assert_eq!(insert_segment(test,  5, "..", (), '-', "-"), Ok("aa-bb-cc-dd-ee-..".into()));
        assert_eq!(insert_segment(test,  6, "..", (), '-', "-"), Err(()));

        let test = "";
        assert_eq!(insert_segment(test, -3, "..", (), '-', "-"), Err(()));
        assert_eq!(insert_segment(test, -2, "..", (), '-', "-"), Ok("..-".into()));
        assert_eq!(insert_segment(test, -1, "..", (), '-', "-"), Ok("-..".into()));
        assert_eq!(insert_segment(test,  0, "..", (), '-', "-"), Ok("..-".into()));
        assert_eq!(insert_segment(test,  1, "..", (), '-', "-"), Ok("-..".into()));
        assert_eq!(insert_segment(test,  2, "..", (), '-', "-"), Err(()));

        let test = "aa";
        assert_eq!(insert_segment(test, -3, "..", (), '-', "-"), Err(()));
        assert_eq!(insert_segment(test, -2, "..", (), '-', "-"), Ok("..-aa".into()));
        assert_eq!(insert_segment(test, -1, "..", (), '-', "-"), Ok("aa-..".into()));
        assert_eq!(insert_segment(test,  0, "..", (), '-', "-"), Ok("..-aa".into()));
        assert_eq!(insert_segment(test,  1, "..", (), '-', "-"), Ok("aa-..".into()));
        assert_eq!(insert_segment(test,  2, "..", (), '-', "-"), Err(()));
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

