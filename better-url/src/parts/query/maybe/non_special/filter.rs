//! Filters.

use crate::prelude::*;

impl MaybeNonSpecialQuery<'_> {
    /// [`Self::filter`] but chainable.
    pub fn filtered<F: FnMut(NonSpecialQuerySegment<'_>) -> bool>(mut self, f: F) -> (bool, Self) {
        (self.filter(f), self)
    }

    /// [`Self::try_filter`] but chainable.
    /// # Errors
    /// If [`Self::try_filter`] returns an error, that error is returned.
    pub fn try_filtered<F: FnMut(NonSpecialQuerySegment<'_>) -> Result<bool, E>, E>(mut self, f: F) -> Result<(bool, Self), E> {
        Ok((self.try_filter(f)?, self))
    }

    /// Keeps only [`NonSpecialQuerySegment`]s matching the predicate `f`.
    #[expect(clippy::missing_panics_doc, reason = "Can't happen.")]
    pub fn filter<F: FnMut(NonSpecialQuerySegment<'_>) -> bool>(&mut self, mut f: F) -> bool {
        self.try_filter(|x| Ok::<_, std::convert::Infallible>(f(x))).expect("???")
    }

    /// Keeps only [`NonSpecialQuerySegment`]s matching the predicate `f`.
    /// # Errors
    /// If any call to `f` returns an error, that error is returned.
    pub fn try_filter<F: FnMut(NonSpecialQuerySegment<'_>) -> Result<bool, E>, E>(&mut self, mut f: F) -> Result<bool, E> {
        let old_len = self.len();

        if let Some(query) = &mut self.0 {
            let mut ranges = smallvec::SmallVec::<[Range<usize>; 2]>::new();

            for segment in query.iter() {
                let range = query.as_str().my_substr_range(segment.as_str());

                if f(segment)? {
                    if let Some(x) = ranges.last_mut() && x.end == range.start - 1 {
                        x.end = range.end;
                    } else {
                        ranges.push(range);
                    }
                }
            }

            match &*ranges {
                [] => self.0 = None,
                [range] => unsafe {query.0.retain_range_unchecked(range.clone())},
                ranges => {
                    let mut ret = String::with_capacity(ranges.iter().map(Range::len).sum::<usize>() + ranges.len());

                    for (i, range) in ranges.iter().enumerate() {
                        if i != 0 {
                            ret.push('&');
                        }

                        ret.push_str(unsafe {query.as_str().get_unchecked(range.clone())});
                    }

                    *query = unsafe {NonSpecialQuery::new_unchecked(ret)};
                }
            }
        }

        Ok(old_len != self.len())
    }
}
