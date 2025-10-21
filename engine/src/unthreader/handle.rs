//! [`UnthreaderHandle`].

use std::time::Instant;
use std::cell::Cell;

use parking_lot::ReentrantMutexGuard;

#[expect(unused_imports, reason = "Used in a doc comment.")]
use crate::prelude::*;

/// A handle for an [`UnthreaderInner`]/[`Unthreader`].
///
/// Should be assigned to a variable that goes out of scope when the thing being unthrode is over.
#[allow(dead_code, reason = "Used for its drop glue.")]
#[derive(Debug)]
pub struct UnthreaderHandle<'a>(pub(crate) Option<ReentrantMutexGuard<'a, Cell<Option<Instant>>>>);

// For some reason, changing the above `pub(crate)` to `pub(super)` makes the declaration for `Unthreader` fail.
// Something about `UnthreaderHandle` not being found???
