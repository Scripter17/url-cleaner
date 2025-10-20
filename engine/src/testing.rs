//! A basic and not very good testing framework.

use serde::{Serialize, Deserialize};

use crate::prelude::*;

/// Tests.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tests {
    /// The individual [`TestSet`]s.
    pub sets: Vec<TestSet>
}

impl Tests {
    /// Do the tests. Panicking if any fail.
    /// # Panics
    /// If any call to [`TestSet::do`] panics, "returns" that panic.
    pub fn r#do(self, cleaner: &Cleaner) {
        for set in self.sets {
            set.r#do(cleaner)
        }
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

impl TestSet {
    /// Do the tests, panicking if any fail.
    /// # Panics
    /// If a call to [`JobIntoIterator::next`] returns an error, panics.
    ///
    /// If a call to [`Task::do`] returns an error, panics.
    ///
    /// If any test fails, panics.
    pub fn r#do(self, cleaner: &Cleaner) {
        let mut cleaner = cleaner.borrowed();

        println!(
            "TestSet\n  params_diff: {}\n  job_context: {}",
            serde_json::to_string(&self.params_diff).expect("Serialization to never fail."),
            serde_json::to_string(&self.job_context).expect("Serialization to never fail.")
        );

        if let Some(params_diff) = self.params_diff {
            params_diff.apply_once(&mut cleaner.params);
        }

        let job_config = JobConfig {
            context: &self.job_context,
            cleaner: &cleaner,
            unthreader: &Default::default(),
            #[cfg(feature = "cache")]
            cache_handle: CacheHandle {
                cache: &Default::default(),
                config: Default::default()
            },
            #[cfg(feature = "http")]
            http_client: &Default::default()
        };

        for test in self.tests {
            println!("    Test: {}", serde_json::to_string(&test).expect("Serialition to never fail."));
            let result1 = job_config.make_task(test.task_config).r#do().expect("The test to execute succesfully.");
            assert_eq!(result1, test.result, "The test to return the expected value.");
            if test.test_idempotence {
                let result2 = job_config.make_task(result1.clone().into()).r#do().expect("The idempotence test to be succeed.");
                assert_eq!(result2, result1, "Idempotence to be upheld.");
            }
        }
    }
}

/// An individual test.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Test {
    /// The [`TaskConfig`].
    pub task_config: TaskConfig,
    /// The expected result.
    pub result: BetterUrl,
    /// If [`true`], test idempotence.
    ///
    /// Defaults to [`true`].
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub test_idempotence: bool
}
