//! Extension traits.

pub(crate) mod str_ext;
pub(crate) mod string_ext;
pub(crate) mod cow_str_ext;
pub(crate) mod iterator_ext;
pub(crate) mod double_ended_iterator_ext;

/// Prelude module for importing everything here better.
pub(crate) mod prelude {
    pub(crate) use super::str_ext::*;
    pub(crate) use super::string_ext::*;
    pub(crate) use super::cow_str_ext::*;
    pub(crate) use super::iterator_ext::*;
    pub(crate) use super::double_ended_iterator_ext::*;
}
