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
        match n - self.take(n).count() {
            0 => self.next().ok_or(0),
            x => Err(x)
        }
    }
}
