use serde::{Serialize, Deserialize};

use super::{StringError, neg_index, neg_range};

/// A wrapper around [`str`]'s various substring searching functions.
/// [`isize`] is used to allow Python-style negative indexing.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum StringLocation {
    /// Checks if an instance of the needle exists anywhere in the haystack.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringLocation;
    /// assert!(StringLocation::Anywhere.satisfied_by("abcdef", "cde").is_ok_and(|x| x==true ));
    /// assert!(StringLocation::Anywhere.satisfied_by("abcdef", "efg").is_ok_and(|x| x==false));
    /// ```
    #[default]
    Anywhere,
    /// Checks if an instance of the needle exists at the start of the haystack.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringLocation;
    /// assert!(StringLocation::Start.satisfied_by("abcdef", "abc").is_ok_and(|x| x==true ));
    /// assert!(StringLocation::Start.satisfied_by("abcdef", "bcd").is_ok_and(|x| x==false));
    /// ```
    Start,
    /// Checks if an instance of the needle exists at the end of the haystack.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringLocation;
    /// assert!(StringLocation::End.satisfied_by("abcdef", "def").is_ok_and(|x| x==true ));
    /// assert!(StringLocation::End.satisfied_by("abcdef", "cde").is_ok_and(|x| x==false));
    /// ```
    End,
    /// Checks if an instance of the needle starts and ends at the specified range in the haystack.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringLocation;
    /// assert!(StringLocation::RangeIs{start: Some( 0), end: Some( 3)}.satisfied_by("abcdef", "abc"   ).is_ok_and(|x| x==true));
    /// assert!(StringLocation::RangeIs{start: Some( 1), end: Some( 4)}.satisfied_by("abcdef", "bcd"   ).is_ok_and(|x| x==true));
    /// assert!(StringLocation::RangeIs{start: Some( 0), end: Some( 6)}.satisfied_by("abcdef", "abcdef").is_ok_and(|x| x==true));
    /// assert!(StringLocation::RangeIs{start: Some( 5), end: Some( 6)}.satisfied_by("abcdef", "f"     ).is_ok_and(|x| x==true));
    /// assert!(StringLocation::RangeIs{start: Some( 6), end: Some( 7)}.satisfied_by("abcdef", "f"     ).is_err());
    /// assert!(StringLocation::RangeIs{start: Some( 6), end: None    }.satisfied_by("abcdef", ""      ).is_ok_and(|x| x==true));
    ///
    /// assert!(StringLocation::RangeIs{start: Some(-1), end: None    }.satisfied_by("abcdef", "f"     ).is_ok_and(|x| x==true));
    /// assert!(StringLocation::RangeIs{start: Some(-2), end: Some(-1)}.satisfied_by("abcdef", "e"     ).is_ok_and(|x| x==true));
    /// ```
    RangeIs {
        /// The start of the range to check.
        start: Option<isize>,
        /// The end of the range to check.
        end: Option<isize>
    },
    /// Checks if an instance of the needle starts at the specified point in the haystack.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringLocation;
    /// assert!(StringLocation::StartsAt( 0).satisfied_by("abcdef", "abc").is_ok_and(|x| x==true ));
    /// assert!(StringLocation::StartsAt( 1).satisfied_by("abcdef", "bcd").is_ok_and(|x| x==true ));
    /// assert!(StringLocation::StartsAt( 5).satisfied_by("abcdef", "f"  ).is_ok_and(|x| x==true ));
    /// assert!(StringLocation::StartsAt( 0).satisfied_by("abcdef", "bcd").is_ok_and(|x| x==false));
    /// assert!(StringLocation::StartsAt( 1).satisfied_by("abcdef", "cde").is_ok_and(|x| x==false));
    /// assert!(StringLocation::StartsAt( 5).satisfied_by("abcdef", "def").is_ok_and(|x| x==false));
    ///
    /// assert!(StringLocation::StartsAt(-2).satisfied_by("abcdef", "ef" ).is_ok_and(|x| x==true));
    /// ```
    StartsAt(isize),
    /// Checks if an instance of the needle ends at the specified point in the haystack.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringLocation;
    /// assert!(StringLocation::EndsAt(3).satisfied_by("abcdef", "abc").is_ok_and(|x| x==true ));
    /// assert!(StringLocation::EndsAt(4).satisfied_by("abcdef", "bcd").is_ok_and(|x| x==true ));
    /// assert!(StringLocation::EndsAt(6).satisfied_by("abcdef", "def").is_ok_and(|x| x==true ));
    /// assert!(StringLocation::EndsAt(6).satisfied_by("abcdef", "f"  ).is_ok_and(|x| x==true ));
    /// assert!(StringLocation::EndsAt(3).satisfied_by("abcdef", "bcd").is_ok_and(|x| x==false));
    /// assert!(StringLocation::EndsAt(4).satisfied_by("abcdef", "cde").is_ok_and(|x| x==false));
    /// ```
    EndsAt(isize),
    /// Checks if an instance of the needle exists within the specified range of the haystack.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringLocation;
    /// assert!(StringLocation::RangeHas{start: Some(0), end: Some(1)}.satisfied_by("abcdef", "a"   ).is_ok_and(|x| x==true ));
    /// assert!(StringLocation::RangeHas{start: Some(0), end: Some(2)}.satisfied_by("abcdef", "a"   ).is_ok_and(|x| x==true ));
    /// assert!(StringLocation::RangeHas{start: Some(0), end: Some(6)}.satisfied_by("abcdef", "bcde").is_ok_and(|x| x==true ));
    /// assert!(StringLocation::RangeHas{start: Some(1), end: Some(6)}.satisfied_by("abcdef", "a"   ).is_ok_and(|x| x==false));
    /// assert!(StringLocation::RangeHas{start: Some(1), end: Some(6)}.satisfied_by("abcdef", "f"   ).is_ok_and(|x| x==true ));
    /// assert!(StringLocation::RangeHas{start: Some(0), end: Some(7)}.satisfied_by("abcdef", ""    ).is_err());
    /// ```
    RangeHas {
        /// The start of the range to check.
        start: Option<isize>,
        /// The end of the range to check.
        end: Option<isize>
    },
    /// Checks if an instance of the needle exists after the specified point in the haystack.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringLocation;
    /// assert!(StringLocation::After(0).satisfied_by("abcdef", "abcdef").is_ok_and(|x| x==true ));
    /// assert!(StringLocation::After(1).satisfied_by("abcdef", "bcdef" ).is_ok_and(|x| x==true ));
    /// assert!(StringLocation::After(1).satisfied_by("abcdef", "1"     ).is_ok_and(|x| x==false));
    /// assert!(StringLocation::After(6).satisfied_by("abcdef", "f"     ).is_ok_and(|x| x==false));
    /// assert!(StringLocation::After(7).satisfied_by("abcdef", ""      ).is_err());
    /// ```
    After(isize),
    /// Checks if an instance of the needle exists before the specified point in the haystack.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringLocation;
    /// assert!(StringLocation::Before(0).satisfied_by("abcdef", ""   ).is_ok_and(|x| x==true ));
    /// assert!(StringLocation::Before(1).satisfied_by("abcdef", "a"  ).is_ok_and(|x| x==true ));
    /// assert!(StringLocation::Before(6).satisfied_by("abcdef", "a"  ).is_ok_and(|x| x==true ));
    /// assert!(StringLocation::Before(4).satisfied_by("abcdef", "def").is_ok_and(|x| x==false ));
    /// assert!(StringLocation::Before(7).satisfied_by("abcdef", "a"  ).is_err());
    /// ```
    Before(isize),
    /// Checks equality.
    /// Meant primarily for use with [`Self::AnySegment`] and [`Self::NthSegment`].
    Equals,
    /// Splits the haystack at every instance of `split` and check if any segment satisfies `location`.
    AnySegment {
        /// The string to split by.
        split: String,
        /// The [`StringLocation`] of each segment to look for `needle` in.
        #[serde(default = "box_equals")]
        location: Box<StringLocation>
    },
    /// Splits the haystack at every instance of `split` and check if the `n`th segment satisfies `location`.
    NthSegment {
        /// The string to split by.
        split: String,
        /// The index of the segment to search in.
        n: isize,
        /// The [`StringLocation`] of the `n`th segment to look for `needle` in.
        #[serde(default = "box_equals")]
        location: Box<StringLocation>
    }
}

fn box_equals() -> Box<StringLocation> {Box::new(StringLocation::Equals)}

impl StringLocation {
    /// Checks if `needle` exists in `haystack` according to `self`'s rules.
    /// # Errors
    /// If only part of the haystack is searched and that part either is out of bounds or splits a UTF-8 codepoint, returns the error [`super::StringError::InvalidSlice`].
    pub fn satisfied_by(&self, haystack: &str, needle: &str) -> Result<bool, StringError> {
        Ok(match self {
            Self::Anywhere             => haystack.contains   (needle),
            Self::Start                => haystack.starts_with(needle),
            Self::End                  => haystack.ends_with  (needle),

            Self::RangeIs {start, end} => haystack.get(  neg_range(*start, *end, haystack.len()).ok_or(StringError::InvalidSlice)?  ).ok_or(StringError::InvalidSlice)?==needle,
            Self::StartsAt(start     ) => haystack.get(  neg_index(*start,       haystack.len()).ok_or(StringError::InvalidIndex)?..).ok_or(StringError::InvalidSlice)?.starts_with(needle),
            Self::EndsAt  (       end) => haystack.get(..neg_index(        *end, haystack.len()).ok_or(StringError::InvalidIndex)?  ).ok_or(StringError::InvalidSlice)?.ends_with(needle),

            Self::RangeHas{start, end} => haystack.get(  neg_range(*start, *end, haystack.len()).ok_or(StringError::InvalidSlice)?  ).ok_or(StringError::InvalidSlice)?.contains(needle),
            Self::After   (start     ) => haystack.get(  neg_index(*start,       haystack.len()).ok_or(StringError::InvalidIndex)?..).ok_or(StringError::InvalidSlice)?.contains(needle),
            Self::Before  (       end) => haystack.get(..neg_index(        *end, haystack.len()).ok_or(StringError::InvalidIndex)?  ).ok_or(StringError::InvalidSlice)?.contains(needle),

            Self::Equals               => haystack==needle,
            Self::AnySegment {split, location} => {
                for segment in haystack.split(split) {
                    if location.satisfied_by(segment, needle)? {
                        return Ok(true)
                    }
                }
                return Ok(false)
            },
            Self::NthSegment {split, n, location} => location.satisfied_by(super::neg_nth(haystack.split(split), *n).ok_or(StringError::SegmentNotFound)?, needle)?
        })
    }
}
