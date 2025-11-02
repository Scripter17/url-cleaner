//! A basic and not very good testing framework.

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::prelude::*;

/// Tests.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tests {
    /// The individual [`TestSet`]s.
    pub sets: Vec<TestSet>
}

/// The error [`Tests::do`] returns.
#[derive(Debug, Error)]
#[error("Tests failed.")]
pub struct DoTestsError;


impl Tests {
    /// Do the tests.
    /// # Errors
    /// If a test fails, returns an error.
    pub fn r#do(self, cleaner: &Cleaner) -> Result<(), DoTestsError> {
        let mut ret = Ok(());

        for set in self.sets {
            if set.r#do(cleaner).is_err() {
                ret = Err(DoTestsError);
            }
        }

        ret
    }
}

/// Rules for how to construct a [`Job`] from a [`Cleaner`] and the [`Test`]s to run on it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestSet {
    /// The [`ParamsDiff`] to apply to the [`Cleaner`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub params_diff: Option<ParamsDiff>,
    /// The [`JobContext`] to give to the [`Job`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub job_context: JobContext,
    /// The [`Test`]s to run.
    pub tests: Vec<Test>
}

/// The error [`TestSet::do`] returns.
#[derive(Debug, Error)]
#[error("Test set failed.")]
pub struct DoTestSetError;

impl TestSet {
    /// Do the tests.
    /// # Errors
    /// If a test fails, returns an error.
    #[expect(clippy::missing_panics_doc, reason = "Shouldn't ever happen.")]
    pub fn r#do(self, cleaner: &Cleaner) -> Result<(), DoTestSetError> {
        let mut cleaner = cleaner.borrowed();

        println!(
            "TestSet\n  params_diff: {}\n  job_context: {}",
            serde_json::to_string(&self.params_diff).expect("Serialization to never fail."),
            serde_json::to_string(&self.job_context).expect("Serialization to never fail.")
        );

        if let Some(params_diff) = self.params_diff {
            params_diff.apply(&mut cleaner.params);
        }

        let job_config = JobConfig {
            context: &self.job_context,
            cleaner: &cleaner,
            unthreader: &Default::default(),
            #[cfg(feature = "cache")]
            cache: Cache {
                config: CacheConfig {
                    read : false,
                    write: false,
                    delay: false,
                },
                inner: &Default::default()
            },
            #[cfg(feature = "http")]
            http_client: &Default::default()
        };

        let mut ret = Ok(());

        for test in self.tests {
            println!("    Test: {}", serde_json::to_string(&test).expect("Serialition to never fail."));
            match test.r#do(&job_config) {
                Ok(()) => {},
                Err(e) => {
                    println!("     FAILED: {e:?}");
                    ret = Err(DoTestSetError);
                }
            }
        }

        ret
    }
}

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
            let idempotence_result = job_config.make_task(result.clone().into()).r#do().map_err(DoTestError::DoIdempotenceTaskError)?;
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
