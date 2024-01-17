use serde::{Serialize, Deserialize};

/// The location of a string. Used by [`crate::rules::conditions::Condition::UrlPartContains`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum StringLocation {
    /// [`str::contains`].
    #[default]
    Anywhere,
    /// [`str::starts_with`].
    Start,
    /// [`str::ends_with`].
    End,
    /// `str::get(start..).handle_error().substr.starts_with(...)`.
    StartsAt(usize),
    /// `str::get(..end).handle_error().substr.ends_with(...)`.
    EndsAt(usize),
    /// `str::get(start..end).handle_error()==...`.
    RangeIs {
        /// The start of the range to check.
        start: usize,
        /// The end of the range to check.
        end: usize
    },
    /// `str::get(start.end).handle_error().substr.contains(...)`
    RangeHas {
        /// The start of the range to check.
        start: usize,
        /// The end of the range to check.
        end: usize
    },
    /// `str::get(start..).handle_error().substr.contains(...)`.
    After(usize),
    /// `str::get(..end).handle_error().substr.contains(...)`.
    Before(usize)
}

impl StringLocation {
    /// Ceck if `needle` exists in `haystack` according to `self`'s rules.
    pub fn satisfied_by(&self, haystack: &str, needle: &str) -> Result<bool, super::StringError> {
        Ok(match self {
            Self::Anywhere             => haystack.contains   (needle),
            Self::Start                => haystack.starts_with(needle),
            Self::End                  => haystack.ends_with  (needle),
            Self::StartsAt(start     ) => haystack.get(*start..    ).ok_or(super::StringError::InvalidSlice)?.starts_with(needle),
            Self::EndsAt  (       end) => haystack.get(      ..*end).ok_or(super::StringError::InvalidSlice)?.ends_with  (needle),
            Self::RangeIs {start, end} => haystack.get(*start..*end).ok_or(super::StringError::InvalidSlice)?==needle,
            Self::RangeHas{start, end} => haystack.get(*start..*end).ok_or(super::StringError::InvalidSlice)?.contains(needle),
            Self::After   (start     ) => haystack.get(*start..    ).ok_or(super::StringError::InvalidSlice)?.contains(needle),
            Self::Before  (       end) => haystack.get(      ..*end).ok_or(super::StringError::InvalidSlice)?.contains(needle)
        })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    const fn passes(x: bool) -> bool {x}
    const fn fails(x: bool) -> bool {!x}

    #[test]
    fn string_location_anywhere() {
        assert!(StringLocation::Anywhere.satisfied_by("abcdef", "cde").is_ok_and(passes));
        assert!(StringLocation::Anywhere.satisfied_by("abcdef", "efg").is_ok_and(fails));
    }

    #[test]
    fn string_location_start() {
        assert!(StringLocation::Start.satisfied_by("abcdef", "abc").is_ok_and(passes));
        assert!(StringLocation::Start.satisfied_by("abcdef", "bcd").is_ok_and(fails));
    }

    #[test]
    fn string_location_end() {
        assert!(StringLocation::End.satisfied_by("abcdef", "def").is_ok_and(passes));
        assert!(StringLocation::End.satisfied_by("abcdef", "cde").is_ok_and(fails));
    }

    #[test]
    fn string_location_starts_at() {
        assert!(StringLocation::StartsAt(0).satisfied_by("abcdef", "abc").is_ok_and(passes));
        assert!(StringLocation::StartsAt(1).satisfied_by("abcdef", "bcd").is_ok_and(passes));
        assert!(StringLocation::StartsAt(5).satisfied_by("abcdef", "f"  ).is_ok_and(passes));
        assert!(StringLocation::StartsAt(0).satisfied_by("abcdef", "bcd").is_ok_and(fails));
        assert!(StringLocation::StartsAt(1).satisfied_by("abcdef", "cde").is_ok_and(fails));
        assert!(StringLocation::StartsAt(5).satisfied_by("abcdef", "def").is_ok_and(fails));
    }

    #[test]
    fn string_location_ends_at() {
        assert!(StringLocation::EndsAt(3).satisfied_by("abcdef", "abc").is_ok_and(passes));
        assert!(StringLocation::EndsAt(4).satisfied_by("abcdef", "bcd").is_ok_and(passes));
        assert!(StringLocation::EndsAt(6).satisfied_by("abcdef", "def").is_ok_and(passes));
        assert!(StringLocation::EndsAt(6).satisfied_by("abcdef", "f"  ).is_ok_and(passes));
        assert!(StringLocation::EndsAt(3).satisfied_by("abcdef", "bcd").is_ok_and(fails));
        assert!(StringLocation::EndsAt(4).satisfied_by("abcdef", "cde").is_ok_and(fails));
    }

    #[test]
    fn string_location_range_is() {
        assert!(StringLocation::RangeIs{start: 0, end: 3}.satisfied_by("abcdef", "abc"   ).is_ok_and(passes));
        assert!(StringLocation::RangeIs{start: 1, end: 4}.satisfied_by("abcdef", "bcd"   ).is_ok_and(passes));
        assert!(StringLocation::RangeIs{start: 0, end: 6}.satisfied_by("abcdef", "abcdef").is_ok_and(passes));
        assert!(StringLocation::RangeIs{start: 5, end: 6}.satisfied_by("abcdef", "f"     ).is_ok_and(passes));
        assert!(StringLocation::RangeIs{start: 6, end: 7}.satisfied_by("abcdef", "f"     ).is_err());
        assert!(StringLocation::RangeIs{start: 7, end: 8}.satisfied_by("abcdef", "f"     ).is_err());
    }

    #[test]
    fn string_location_range_has() {
        assert!(StringLocation::RangeHas{start: 0, end: 1}.satisfied_by("abcdef", "a"   ).is_ok_and(passes));
        assert!(StringLocation::RangeHas{start: 0, end: 2}.satisfied_by("abcdef", "a"   ).is_ok_and(passes));
        assert!(StringLocation::RangeHas{start: 0, end: 6}.satisfied_by("abcdef", "bcde").is_ok_and(passes));
        assert!(StringLocation::RangeHas{start: 1, end: 6}.satisfied_by("abcdef", "a"   ).is_ok_and(fails));
        assert!(StringLocation::RangeHas{start: 0, end: 7}.satisfied_by("abcdef", ""    ).is_err());
    }

    #[test]
    fn string_location_after() {
        assert!(StringLocation::After(0).satisfied_by("abcdef", "abcdef").is_ok_and(passes));
        assert!(StringLocation::After(1).satisfied_by("abcdef", "bcdef" ).is_ok_and(passes));
        assert!(StringLocation::After(1).satisfied_by("abcdef", "1"     ).is_ok_and(fails));
        assert!(StringLocation::After(6).satisfied_by("abcdef", "f"     ).is_ok_and(fails));
        assert!(StringLocation::After(7).satisfied_by("abcdef", ""      ).is_err());
    }

    #[test]
    fn string_location_before() {
        assert!(StringLocation::Before(0).satisfied_by("abcdef", ""   ).is_ok_and(passes));
        assert!(StringLocation::Before(1).satisfied_by("abcdef", "a"  ).is_ok_and(passes));
        assert!(StringLocation::Before(6).satisfied_by("abcdef", "a"  ).is_ok_and(passes));
        assert!(StringLocation::Before(4).satisfied_by("abcdef", "def").is_ok_and(fails ));
        assert!(StringLocation::Before(7).satisfied_by("abcdef", "a"  ).is_err());
    }    
}
