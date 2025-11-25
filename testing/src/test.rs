use crate::*;

/// An individual test.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Test {
    /// The [`TaskConfig`].
    pub task_config: TaskConfig,
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
    pub fn r#do(self, job_config: &JobConfig) -> Result<(), DoTestError> {
        let result = job_config.make_task(self.task_config).r#do().map_err(DoTestError::DoTaskError)?;
        if result != self.expect {
            Err(DoTestError::ExpectationError {
                expected: self.expect,
                got: result
            })?
        } else {
            let idempotence_result = job_config.make_task(result.clone()).r#do().map_err(DoTestError::DoIdempotenceTaskError)?;
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

