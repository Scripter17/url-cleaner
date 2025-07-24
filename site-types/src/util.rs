//! Utility stuff.

/// Returns [`true`] if `x` is `T`'s [`Default::default`].
pub(crate) fn is_default<T: Default + PartialEq>(x: &T) -> bool {x == &T::default()}
/// Returns [`true`] if `x` is [`true`].
pub(crate) fn is_true(x: &bool) -> bool {*x}
/// Returns [`true`].
pub(crate) fn get_true() -> bool {true}
