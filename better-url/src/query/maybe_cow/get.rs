//! Getters.

use std::borrow::Cow;

use crate::prelude::*;

impl BetterMaybeQuery<'_> {
    /// If [`Self::0`] is [`Some`], [`BetterQuery::get`].
    pub fn get<'a>(&'a self, index: isize) -> Option<RawQuerySegment<'a>> {
        self.0.as_ref()?.get(index)
    }

    /// [`Self::get`] and [`RawQuerySegment::raw_name`].
    pub fn get_raw_name(&self, index: isize) -> Option<&str> {
        self.get(index).map(RawQuerySegment::raw_name)
    }

    /// [`Self::get`] and [`RawQuerySegment::raw_value`].
    pub fn get_raw_value(&self, index: isize) -> Option<Option<&str>> {
        self.get(index).map(RawQuerySegment::raw_value)
    }

    /// [`Self::get`] and [`RawQuerySegment::lazy_name`].
    pub fn get_lazy_name(&self, index: isize) -> Option<QueryPartDecoder<'_>> {
        self.get(index).map(RawQuerySegment::lazy_name)
    }

    /// [`Self::get`] and [`RawQuerySegment::lazy_value`].
    pub fn get_lazy_value(&self, index: isize) -> Option<Option<QueryPartDecoder<'_>>> {
        self.get(index).map(RawQuerySegment::lazy_value)
    }

    /// [`Self::get`] and [`RawQuerySegment::name`].
    pub fn get_name(&self, index: isize) -> Option<Cow<'_, str>> {
        self.get(index).map(RawQuerySegment::name)
    }

    /// [`Self::get`] and [`RawQuerySegment::value`].
    pub fn get_value(&self, index: isize) -> Option<Option<Cow<'_, str>>> {
        self.get(index).map(RawQuerySegment::value)
    }

    /// If [`Self::0`] is [`Some`], [`BetterQuery::find`].
    pub fn find<'a>(&'a self, name: &str, index: isize) -> Option<RawQuerySegment<'a>> {
        self.0.as_ref()?.find(name, index)
    }

    /// [`Self::find`] and [`RawQuerySegment::raw_name`].
    pub fn find_raw_name(&self, name: &str, index: isize) -> Option<&str> {
        self.find(name, index).map(RawQuerySegment::raw_name)
    }

    /// [`Self::find`] and [`RawQuerySegment::raw_value`].
    pub fn find_raw_value(&self,name: &str,  index: isize) -> Option<Option<&str>> {
        self.find(name, index).map(RawQuerySegment::raw_value)
    }

    /// [`Self::find`] and [`RawQuerySegment::lazy_name`].
    pub fn find_lazy_name(&self,name: &str,  index: isize) -> Option<QueryPartDecoder<'_>> {
        self.find(name, index).map(RawQuerySegment::lazy_name)
    }

    /// [`Self::find`] and [`RawQuerySegment::lazy_value`].
    pub fn find_lazy_value(&self,name: &str,  index: isize) -> Option<Option<QueryPartDecoder<'_>>> {
        self.find(name, index).map(RawQuerySegment::lazy_value)
    }

    /// [`Self::find`] and [`RawQuerySegment::name`].
    pub fn find_name(&self, name: &str, index: isize) -> Option<Cow<'_, str>> {
        self.find(name, index).map(RawQuerySegment::name)
    }

    /// [`Self::find`] and [`RawQuerySegment::value`].
    pub fn find_value(&self,name: &str,  index: isize) -> Option<Option<Cow<'_, str>>> {
        self.find(name, index).map(RawQuerySegment::value)
    }
}
