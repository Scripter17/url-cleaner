//! Serde utilities.

/// Serde helper function that returns true if `x` is `T`'s [`Default::default`] value.
pub(crate) fn is_default<T: Default + PartialEq>(x: &T) -> bool {x == &T::default()}
/// Serde helper function.
pub(crate) const fn get_true() -> bool {true}
/// Serde helper function.
pub(crate) const fn is_true(x: &bool) -> bool {*x}
