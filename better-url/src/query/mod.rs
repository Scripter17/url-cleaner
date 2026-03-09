//! Query stuff.

use std::borrow::Cow;

pub mod cow;
pub mod maybe_cow;
pub mod r#ref;
pub mod maybe_ref;

pub mod raw_segment;
pub mod encode;
pub mod decode;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::cow::*;
    pub use super::maybe_cow::*;
    pub use super::r#ref::*;
    pub use super::maybe_ref::*;

    pub use super::raw_segment::*;
    pub use super::encode::*;
    pub use super::decode::*;

    pub use super::AsQueryStr;
}

use prelude::*;

/// A type that can be borrowed as an `Option<&str>` for setting a query.`
pub trait AsQueryStr {
    /// Borrow as an `Option<&str>`.
    fn as_query_str(&self) -> Option<&str>;
}

impl<T: AsQueryStr> AsQueryStr for &T {fn as_query_str(&self) -> Option<&str> {(*self).as_query_str()}}

impl AsQueryStr for str          {fn as_query_str(&self) -> Option<&str> {Some(self)}}
impl AsQueryStr for Cow<'_, str> {fn as_query_str(&self) -> Option<&str> {Some(self)}}
impl AsQueryStr for String       {fn as_query_str(&self) -> Option<&str> {Some(self)}}

impl AsQueryStr for Option<&str        > {fn as_query_str(&self) -> Option<&str> {self.as_deref()}}
impl AsQueryStr for Option<Cow<'_, str>> {fn as_query_str(&self) -> Option<&str> {self.as_deref()}}
impl AsQueryStr for Option<String      > {fn as_query_str(&self) -> Option<&str> {self.as_deref()}}

impl AsQueryStr for BetterRefQuery     <'_> {fn as_query_str(&self) -> Option<&str> {Some(self.as_str())}}
impl AsQueryStr for BetterQuery        <'_> {fn as_query_str(&self) -> Option<&str> {Some(self.as_str())}}
impl AsQueryStr for BetterMaybeRefQuery<'_> {fn as_query_str(&self) -> Option<&str> {self.as_option_str()}}
impl AsQueryStr for BetterMaybeQuery   <'_> {fn as_query_str(&self) -> Option<&str> {self.as_option_str()}}
