//! Getters.

use std::borrow::Cow;

use crate::prelude::*;

impl<'a> BetterMaybeRefQuery<'a> {
    /// Gets the `index`th [`RawQuerySegment`].
    pub fn get(self, index: isize) -> Option<RawQuerySegment<'a>> {
        self.iter().neg_nth(index)
    }

    /// [`Self::get`] and [`RawQuerySegment::raw_name`].
    pub fn get_raw_name(self, index: isize) -> Option<&'a str> {
        self.get(index).map(RawQuerySegment::raw_name)
    }

    /// [`Self::get`] and [`RawQuerySegment::raw_value`].
    pub fn get_raw_value(self, index: isize) -> Option<Option<&'a str>> {
        self.get(index).map(RawQuerySegment::raw_value)
    }

    /// [`Self::get`] and [`RawQuerySegment::lazy_name`].
    pub fn get_lazy_name(self, index: isize) -> Option<QueryPartDecoder<'a>> {
        self.get(index).map(RawQuerySegment::lazy_name)
    }

    /// [`Self::get`] and [`RawQuerySegment::lazy_value`].
    pub fn get_lazy_value(self, index: isize) -> Option<Option<QueryPartDecoder<'a>>> {
        self.get(index).map(RawQuerySegment::lazy_value)
    }

    /// [`Self::get`] and [`RawQuerySegment::name`].
    pub fn get_name(self, index: isize) -> Option<Cow<'a, str>> {
        self.get(index).map(RawQuerySegment::name)
    }

    /// [`Self::get`] and [`RawQuerySegment::value`].
    pub fn get_value(self, index: isize) -> Option<Option<Cow<'a, str>>> {
        self.get(index).map(RawQuerySegment::value)
    }

    /// Gets the `index`th [`RawQuerySegment`] with name `name`.
    pub fn find(self, name: &str, index: isize) -> Option<RawQuerySegment<'a>> {
        self.iter().filter(|segment| segment.lazy_name() == name).neg_nth(index)
    }

    /// [`Self::find`] and [`RawQuerySegment::raw_name`].
    pub fn find_raw_name(self, name: &str, index: isize) -> Option<&'a str> {
        self.find(name, index).map(RawQuerySegment::raw_name)
    }

    /// [`Self::find`] and [`RawQuerySegment::raw_value`].
    pub fn find_raw_value(self, name: &str,  index: isize) -> Option<Option<&'a str>> {
        self.find(name, index).map(RawQuerySegment::raw_value)
    }

    /// [`Self::find`] and [`RawQuerySegment::lazy_name`].
    pub fn find_lazy_name(self, name: &str,  index: isize) -> Option<QueryPartDecoder<'a>> {
        self.find(name, index).map(RawQuerySegment::lazy_name)
    }

    /// [`Self::find`] and [`RawQuerySegment::lazy_value`].
    pub fn find_lazy_value(self, name: &str,  index: isize) -> Option<Option<QueryPartDecoder<'a>>> {
        self.find(name, index).map(RawQuerySegment::lazy_value)
    }

    /// [`Self::find`] and [`RawQuerySegment::name`].
    pub fn find_name(self, name: &str, index: isize) -> Option<Cow<'a, str>> {
        self.find(name, index).map(RawQuerySegment::name)
    }

    /// [`Self::find`] and [`RawQuerySegment::value`].
    pub fn find_value(self, name: &str,  index: isize) -> Option<Option<Cow<'a, str>>> {
        self.find(name, index).map(RawQuerySegment::value)
    }
}
