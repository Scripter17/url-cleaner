//! [`ParamsDiff`] stuff.

use crate::prelude::*;


/// The [`ParamsDiffDeleter`].
static PARAMS_DIFF_DELETER: OnceLock<ParamsDiffDeleter> = OnceLock::new();

/// The location of the ParamsDiff.
pub const PARAMS_DIFF: &str = "urlc-tool/tmp/bench/params_diff.json";

/// Write the ParamsDiff.
pub fn write_params_diff(params_diff: &str) {
    let mut file = new_file(PARAMS_DIFF);

    writeln!(file, "{params_diff}").unwrap();

    let _ = PARAMS_DIFF_DELETER.get_or_init(Default::default);
}

/// Deletes [`PARAMS_DIFF`] on [`std::ops::Drop`].
#[derive(Debug, Default)]
pub struct ParamsDiffDeleter;

impl std::ops::Drop for ParamsDiffDeleter {
    fn drop(&mut self) {
        std::fs::remove_file(PARAMS_DIFF).unwrap();
    }
}
