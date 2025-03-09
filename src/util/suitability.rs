//! Trait to unify "suitability".

use crate::types::*;

/// Internal trait to handle whether or not a value is "suitable" for being in the default config.
///
/// Mainly about ensuring documentation and no `Debug` variants.
pub(crate) trait Suitable {
    /// Returns [`true`] if [`self`] is "suitable" for being in the default config.
    ///
    /// May panic with an error message.
    fn is_suitable_for_release(&self, config: &Config) -> bool;
}
