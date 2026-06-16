//! [`StringExt`].

/// Extension trait for [`String`].
pub(crate) trait StringExt {
    /// [`String::insert_str`] + [`String::extend`].
    fn insert_with(&mut self, idx: usize, strings: &[&str]);
}

impl StringExt for String {
    fn insert_with(&mut self, mut idx: usize, strings: &[&str]) {
        assert!(self.is_char_boundary(idx));

        let len = self.len();
        let amt = strings.iter().copied().map(str::len).sum::<usize>();
        self.reserve(amt);

        unsafe {
            std::ptr::copy(self.as_bytes().as_ptr().add(idx), self.as_mut_vec().as_mut_ptr().add(idx + amt), len - idx);

            for string in strings {
                std::ptr::copy_nonoverlapping(string.as_ptr(), self.as_mut_vec().as_mut_ptr().add(idx), string.len());
                idx += string.len();
            }

            self.as_mut_vec().set_len(len + amt);
        }
    }
}
