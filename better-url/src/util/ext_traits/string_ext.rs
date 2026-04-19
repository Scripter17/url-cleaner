//! [`StringExt`].

/// Extension trait for [`String`].
pub(crate) trait StringExt {
    /// [`String::insert_str`] + [`String::extend`].
    fn insert_with<T: AsRef<str>, I: IntoIterator<Item = T>>(&mut self, index: usize, iter: I);
}

impl StringExt for String {
    fn insert_with<T: AsRef<str>, I: IntoIterator<Item = T>>(&mut self, mut index: usize, iter: I) {
        for x in iter {
            let x = x.as_ref();
            self.insert_str(index, x);
            index += x.len();
        }
    }
}
