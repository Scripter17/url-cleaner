//! Utility stuff.

/// Returns [`true`] if `x` is `T`'s [`Default::default`].
pub(crate) fn is_default<T: Default + PartialEq>(x: &T) -> bool {x == &T::default()}
