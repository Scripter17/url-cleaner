//! [`Query`] and co..

use crate::prelude::*;

impl MyUrl {
    /// The [`Range::start`] of the query.
    fn query_start(&self) -> Option<usize> {
        Some(self.query_mark?.get() as usize + 1)
    }

    /// The [`Range::end`] of the query.
    fn query_after(&self) -> Option<usize> {
        Some(self.fragment_mark.map_or(self.len(), |x| x.get() as usize))
    }

    /// The [`Range`] of the query.
    fn query_range(&self) -> Option<Range<usize>> {
        Some(self.query_start()? .. self.query_after()?)
    }

    /// The query as a [`str`].
    pub fn query(&self) -> Option<&str> {
        Some(&self.serialization[self.query_range()?])
    }

    /// Set the query.
    /// # Errors
    /// If the URL would become too long, returns the error [`TooLong`].
    pub fn set_query<'a, T: Into<MaybeSpecialQuery<'a>> + Into<MaybeNonSpecialQuery<'a>>>(&mut self, value: T) -> Result<(), SetQueryError> {
        let new = match self.is_special() {
            true  => MaybeQuery::new_special    (value),
            false => MaybeQuery::new_non_special(value),
        };

        match (self.query_range(), new.as_str()) {
            (None, None) => {},

            (None, Some(new)) => {
                if self.len() + new.len() + 1 > u32::MAX as usize {
                    Err(TooLong)?
                }

                match self.fragment_mark {
                    Some(x) => {
                        self.query_mark    = Some(x);
                        self.fragment_mark = NonZero::new(x.get() + new.len() as u32 + 1);

                        self.serialization.insert_str(x.get() as usize, new);
                        self.serialization.insert    (x.get() as usize, '?');
                    },
                    None => {
                        self.query_mark = NonZero::new(self.len() as u32);
                        self.serialization.extend(["?", new]);
                    }
                }
            },

            (Some(range), None     ) => {
                self.serialization.replace_range(range.start - 1 .. range.end, "");

                if self.fragment_mark.is_some() {
                    self.fragment_mark = self.query_mark;
                }

                self.query_mark = None;
            },

            (Some(range), Some(new)) => {
                if self.len() - range.len() + new.len() > u32::MAX as usize {
                    Err(TooLong)?;
                }

                self.serialization.replace_range(range.clone(), new);

                if let Some(x) = self.fragment_mark {
                    self.fragment_mark = NonZero::new(x.get() - range.len() as u32 + new.len() as u32)
                }
            },
        }

        Ok(())
    }
}
