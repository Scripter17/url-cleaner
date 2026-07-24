//! [`BetterUrl::join`].

use crate::prelude::*;

mod scheme_start;

mod authority;
mod abs_path;
mod rel_path;

impl BetterUrl {
    /// Join in-place.
    /// # Errors
    /// If `value` isn't a valid join, returns the error [`InvalidJoin`].
    pub fn join<'a, T: Into<Cow<'a, str>>>(&mut self, value: T) -> Result<(), InvalidJoin> {
        let (_, value) = canonize_parser_input(value);

        self.join_scheme_start(&value)
    }

    /// Replace the path.
    fn join_path_thing<'a, T: Into<SegmentedPath<'a>>>(&mut self, value: T) {
        self.serialization.truncate(self.path_start());
        self.details.query_mark    = None;
        self.details.fragment_mark = None;

        let path = value.into();

        if !self.has_host() {
            match (path.as_str().starts_with("//"), self.details.path_start == self.details.scheme_mark + 1) {
                (true, true) => {
                    self.serialization.push_str("/.");
                    self.details.path_start = self.len() as u32;
                },
                (false, false) => {
                    self.serialization.truncate(self.len() - 2);
                    self.details.path_start = self.len() as u32;
                },
                _ => {}
            }
        }

        self.serialization.push_str(path.as_str());
    }

    /// Push the query.
    fn join_push_query<'a, T: Into<MaybeQuery<'a>>>(&mut self, value: T) {
        if let Some(q) = value.into().as_str() {
            self.details.query_mark = NonZero::new(self.len() as u32);
            self.serialization.extend(["?", q]);
        }
    }

    /// Push the fragment.
    fn join_push_fragment<'a, T: Into<MaybeFragment<'a>>>(&mut self, value: T) {
        if let Some(f) = value.into().as_str() {
            self.details.fragment_mark = NonZero::new(self.len() as u32);
            self.serialization.extend(["#", f]);
        }
    }
}
