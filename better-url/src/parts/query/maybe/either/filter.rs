//! Filters.

use crate::prelude::*;

impl MaybeQuery<'_> {
    /// [`Self::filter`] but chainable.
    pub fn filtered<F: FnMut(&QuerySegment<'_>) -> bool>(mut self, f: F) -> (bool, Self) {
        (self.filter(f), self)
    }

    /// [`Self::try_filter`] but chainable.
    /// # Errors
    /// If the call to [`Self::try_filter`] returns an error, that error is returned.
    pub fn try_filtered<F: FnMut(&QuerySegment<'_>) -> Result<bool, E>, E>(mut self, f: F) -> Result<(bool, Self), E> {
        Ok((self.try_filter(f)?, self))
    }

    /// Keeps only [`QuerySegment`]s matching the predicate `f`.
    #[allow(clippy::missing_panics_doc, reason = "Can't happen.")]
    pub fn filter<F: FnMut(&QuerySegment<'_>) -> bool>(&mut self, mut f: F) -> bool {
        self.try_filter(|x| Ok::<_, std::convert::Infallible>(f(x))).expect("???")
    }

    /// Keeps only [`QuerySegment`]s matching the predicate `f`.
    /// # Errors
    /// If any call to `f` returns an error, that error is returned.
    pub fn try_filter<F: FnMut(&QuerySegment<'_>) -> Result<bool, E>, E>(&mut self, mut f: F) -> Result<bool, E> {
        let old_len = self.len();

        if let Some(query) = &mut self.0 {
            let mut ranges = Vec::<Range<usize>>::new();

            for segment in query.iter() {
                if f(&segment)? {
                    let range = query.as_str().my_substr_range(segment.as_str());

                    if let Some(x) = ranges.last_mut() && x.end == range.start - 1 {
                        x.end = range.end;
                    } else {
                        ranges.push(range);
                    }
                }
            }

            match &*ranges {
                [] => self.0 = None,
                [range] => match query {
                    Query::Special   (x) => x.0.retain_range(range.clone()),
                    Query::NonSpecial(x) => x.0.retain_range(range.clone()),
                    Query::Fragment  (x) => x.0.retain_range(range.clone()),
                },
                [first, ranges @ ..] => {
                    let mut ret = query.as_str()[first.clone()].to_string();
                    for range in ranges {
                        ret.push('&');
                        ret.push_str(&query.as_str()[range.clone()]);
                    }
                    match query {
                        Query::Special   (x) => x.0 = ret.into(),
                        Query::NonSpecial(x) => x.0 = ret.into(),
                        Query::Fragment  (x) => x.0 = ret.into(),
                    }
                }
            }
        }

        Ok(old_len != self.len())
    }
}
