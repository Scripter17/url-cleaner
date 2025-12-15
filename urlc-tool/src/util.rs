//! Misc. utility stuff.

use super::prelude::*;

/// The output format.
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum Format {
    /// Output plain task lines.
    #[default]
    Quick,
    /// Output benchmark lines.
    Suite
}
