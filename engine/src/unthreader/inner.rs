//! [`UnthreaderInner`].

use std::time::Instant;
use std::cell::Cell;

use parking_lot::ReentrantMutex;

#[expect(unused_imports, reason = "Used in a doc comment.")]
use crate::prelude::*;

/// The actual unthreading handler.
#[derive(Debug, Default)]
pub struct UnthreaderInner(pub ReentrantMutex<Cell<Option<Instant>>>);

