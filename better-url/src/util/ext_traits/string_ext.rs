//! [`StringExt`].

/// Extension trait for [`String`].
pub(crate) trait StringExt {
    /// [`String::insert_str`] + [`String::extend`].
    fn insert_with<const N: usize>(&mut self, idx: usize, strings: [&str; N]);

    /// Insert multiple [`str`]s starting at `idx` without checking for validity.
    unsafe fn insert_with_unchecked<const N: usize>(&mut self, idx: usize, strings: [&str; N]);
}

impl StringExt for String {
    fn insert_with<const N: usize>(&mut self, idx: usize, strings: [&str; N]) {
        assert!(self.is_char_boundary(idx));

        unsafe {
            self.insert_with_unchecked(idx, strings)
        }
    }

    unsafe fn insert_with_unchecked<const N: usize>(&mut self, mut idx: usize, strings: [&str; N]) {
        let len = self.len();
        let amt = strings.into_iter().map(str::len).sum::<usize>();
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
