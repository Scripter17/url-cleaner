//! [`IteratorExt`].

/// [`Iterator`] extension trait.
pub(crate) trait IteratorExt: Iterator {
    /// Get the `n`th element.
    /// # Errors
    /// If fewer than `n` elements are available, returns the missing amount as an error.
    fn try_nth(&mut self, n: usize) -> Result<Self::Item, usize>;
}

impl<I: Iterator> IteratorExt for I {
    fn try_nth(&mut self, n: usize) -> Result<Self::Item, usize> {
        for rem in (1..=n).rev() {
            self.next().ok_or(rem)?;
        }
        self.next().ok_or(0)
    }
}
