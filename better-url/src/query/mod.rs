//! Query stuff.

use std::borrow::Cow;

mod cow;
mod maybe_cow;
mod r#ref;
mod maybe_ref;

mod raw_segment;
mod encode;
mod decode;

pub use cow::*;
pub use maybe_cow::*;
pub use r#ref::*;
pub use maybe_ref::*;

pub use raw_segment::*;
pub use encode::*;
pub use decode::*;

/// A type that can be borrowed as an `Option<&str>` for setting a query.
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
