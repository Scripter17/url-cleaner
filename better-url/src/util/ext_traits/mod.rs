//! Extension traits.

pub(crate) mod str_ext;
pub(crate) mod string_ext;
pub(crate) mod cow_str_ext;
pub(crate) mod iterator_ext;
pub(crate) mod double_ended_iterator_ext;

pub(crate) use str_ext::*;
pub(crate) use string_ext::*;
pub(crate) use cow_str_ext::*;
pub(crate) use iterator_ext::*;
pub(crate) use double_ended_iterator_ext::*;
