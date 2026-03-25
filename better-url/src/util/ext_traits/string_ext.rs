//! [`StringExt`].

use std::ops::Range;

/// Extension trait for [`String`].
pub(crate) trait StringExt {
    /// [`String::insert_str`] + [`String::extend`].
    fn insert_with<'a, I: IntoIterator<Item = &'a str>>(&mut self, index: usize, iter: I);

    /// [`String::replace_range`] + [`String::extend`].
    fn replace_range_with<'a, I: IntoIterator<Item = &'a str>>(&mut self, range: Range<usize>, iter: I);
}

impl StringExt for String {
    fn insert_with<'a, I: IntoIterator<Item = &'a str>>(&mut self, mut index: usize, iter: I) {
        for x in iter {
            self.insert_str(index, x);
            index += x.len();
        }
    }

    fn replace_range_with<'a, I: IntoIterator<Item = &'a str>>(&mut self, range: Range<usize>, iter: I) {
        let mut iter = iter.into_iter();

        let first = iter.next().unwrap_or_default();

        self.replace_range(range.clone(), first);

        self.insert_with(range.start + first.len(), iter);
    }
}
