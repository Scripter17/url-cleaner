//! Setters.

use crate::prelude::*;

impl BetterUrl {
    /// Set the query.
    /// # Errors
    /// If the URL would become too long, returns the error [`TooLong`].
    pub fn set_query<'a, T: Into<MaybeSpecialQuery<'a>> + Into<MaybeNonSpecialQuery<'a>>>(&mut self, value: T) -> Result<(), SetQueryError> {
        let new = MaybeQuery::new(value, self.query_type());

        match (self.query_range(), new.as_str()) {
            (None, None) => {},

            (None, Some(new)) => {
                if self.len() + new.len() + 1 > u32::MAX as usize {
                    Err(TooLong)?
                }

                match self.details.fragment_mark {
                    Some(x) => {
                        self.details.query_mark    = Some(x);
                        self.details.fragment_mark = NonZero::new(x.get() + new.len() as u32 + 1);

                        self.serialization.insert_str(x.get() as usize, new);
                        self.serialization.insert    (x.get() as usize, '?');
                    },
                    None => {
                        self.details.query_mark = NonZero::new(self.len() as u32);
                        self.serialization.extend(["?", new]);
                    }
                }
            },

            (Some(range), None     ) => {
                self.serialization.replace_range(range.start - 1 .. range.end, "");

                if self.details.fragment_mark.is_some() {
                    self.details.fragment_mark = self.details.query_mark;
                }

                self.details.query_mark = None;
            },

            (Some(range), Some(new)) => {
                if self.len() - range.len() + new.len() > u32::MAX as usize {
                    Err(TooLong)?;
                }

                self.serialization.replace_range(range.clone(), new);

                if let Some(x) = self.details.fragment_mark {
                    self.details.fragment_mark = NonZero::new(x.get() - range.len() as u32 + new.len() as u32)
                }
            },
        }

        Ok(())
    }



    /// Remove the query.
    #[expect(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn remove_query(&mut self) -> bool {
        if self.query_str().is_some() {
            self.set_query(None::<&str>).expect("???");
            true
        } else {
            false
        }
    }

    /// Remove the query if it's empty.
    #[expect(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn remove_empty_query(&mut self) -> bool {
        if self.query_str() == Some("") {
            self.set_query(None::<&str>).expect("???");
            true
        } else {
            false
        }
    }

    /// [`MaybeQuery::set`].
    /// # Errors
    /// If the call to [`MaybeQuery::set`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_query`] returns an error, that error is returned.
    pub fn set_query_param(&mut self, name: &str, index: isize, value: Option<Option<&str>>) -> Result<bool, SetQueryError> {
        let mut query = self.query();

        if query.set(name, index, value)? {
            self.set_query(query.into_owned())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// [`MaybeQuery::filtered`].
    #[allow(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn filter_query<F: FnMut(QuerySegment<'_>) -> bool>(&mut self, f: F) -> bool {
        if let (true, query) = self.query().filtered(f) {
            self.set_query(query.into_owned()).expect("???");
            true
        } else {
            false
        }
    }

    /// [`MaybeQuery::try_filtered`].
    /// # Errors
    /// If the call to [`MaybeQuery::try_filtered`] returns an error, that error is returned in an `Ok(Err(_))`.
    #[allow(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn try_filter_query<F: FnMut(QuerySegment<'_>) -> Result<bool, E>, E>(&mut self, f: F) -> Result<bool, E> {
        if let (true, query) = self.query().try_filtered(f)? {
            self.set_query(query.into_owned()).expect("???");
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
