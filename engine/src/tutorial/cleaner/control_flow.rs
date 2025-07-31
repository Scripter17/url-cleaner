//! # Control flow
//!
//! To create advanced cleaners, several components supper various forms of control flow.

pub(crate) use super::*;

pub mod if_then_else;
pub(crate) use if_then_else::*;
pub mod maps;
pub(crate) use maps::*;
pub mod named_partitionings;
pub(crate) use named_partitionings::*;
pub mod repeat;
pub(crate) use repeat::*;
pub mod error_handling;
pub(crate) use error_handling::*;
