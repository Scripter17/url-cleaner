//! [`Test`].

use crate::*;

/// An individual test.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Test {
    /// The task.
    pub task: serde_json::Value,
    /// The expected result.
    pub expect: BetterUrl,
    /// If [`true`], test idempotence.
    ///
    /// Defaults to [`true`].
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub test_idempotence: bool
}

/// The enum of errors [`Test::do`] can return.
#[derive(Debug, Error)]
pub enum DoTestError {
    /// Task failed.
    #[error("Task failed: {0:?}")]
    DoTaskError(DoTaskError),
    /// Expectation failed.
    #[error("Expectation failed: Expected {expected}, got {got}.")]
    ExpectationError {
        /// The expected result.
        expected: BetterUrl,
        /// The actual result.
        got: BetterUrl
    },
    /// Idempotence task failed.
    #[error("Idempotence failed: {0:?}")]
    DoIdempotenceTaskError(DoTaskError),
    /// Idempotence expectation failed.
    #[error("Not idempotent: {initial} became {rerun}")]
    NotIdempotent {
        /// The expected result.
        initial: BetterUrl,
        /// The actual result.
        rerun: BetterUrl
    }
}

impl Test {
    /// Do the test.
    /// # Errors
    /// If the test fails, returns an error.
    #[expect(clippy::result_large_err, reason = "It's a testing thing. Who cares?")]
    pub fn r#do(self, job: &Job) -> Result<(), DoTestError> {
        let result = job.r#do(self.task).map_err(DoTestError::DoTaskError)?;
        if result != self.expect {
            Err(DoTestError::ExpectationError {
                expected: self.expect,
                got: result
            })?
        } else {
            let idempotence_result = job.r#do(result.clone()).map_err(DoTestError::DoIdempotenceTaskError)?;
            if idempotence_result != result {
                Err(DoTestError::NotIdempotent {
                    initial: result,
                    rerun: idempotence_result
                })?
            }
        }

        Ok(())
    }
}

