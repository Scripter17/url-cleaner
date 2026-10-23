//! Serde utilities.

use std::ops::Bound;

/** Serde helper function. **/ pub(crate) const fn get_true (        ) -> bool {true }
/** Serde helper function. **/ pub(crate) const fn get_false(        ) -> bool {false}
/** Serde helper function. **/ pub(crate) const fn is_true  (x: &bool) -> bool {*x   }
/** Serde helper function. **/ pub(crate) const fn is_false (x: &bool) -> bool {!*x  }

/** Serde helper function. **/ pub(crate)       fn is_default  <T: Default + PartialEq>(x: &T       ) -> bool     {x == &T::default()           }
/** Serde helper function. **/ pub(crate) const fn unbounded   <T                     >(            ) -> Bound<T> {            Bound::Unbounded }
/** Serde helper function. **/ pub(crate) const fn is_unbounded<T                     >(x: &Bound<T>) -> bool     {matches!(x, Bound::Unbounded)}
