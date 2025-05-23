//! A basic and not very good testing framework.

use std::borrow::Cow;

use serde::{Serialize, Deserialize};

use crate::types::*;
use crate::util::*;

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
    /// If a call to [`Job::next`] returns an error, panics.
    ///
    /// If a call to [`Task::do`] returns an error, panics.
    ///
    /// If any test fails, panics.
    pub fn r#do(self, cleaner: &Cleaner) {
        let mut cleaner = Cow::Borrowed(cleaner);

        println!(
            "Running test set:\nparams_diff: {}\njob_context: {}",
            serde_json::to_string(&self.params_diff).expect("Serialization to never fail"),
            serde_json::to_string(&self.job_context).expect("Serialization to never fail")
        );

        if let Some(params_diff) = self.params_diff {
            params_diff.apply(&mut cleaner.to_mut().params);
        }

        let (task_configs, expectations) = self.tests.clone().into_iter().map(|Test {task_config, expectation}| (task_config, expectation)).collect::<(Vec<_>, Vec<_>)>();

        let job = Job {
            context: &self.job_context,
            cleaner: &cleaner,
            #[cfg(feature = "cache")]
            cache: &Default::default(),
            lazy_task_configs: Box::new(task_configs.into_iter().map(|task_config| Ok(task_config.into())))
        };

        for (test, task_source, expectation) in self.tests.into_iter().zip(job).zip(expectations).map(|((x, y), z)| (x, y, z)) {
            println!("Running test: {}", serde_json::to_string(&test).expect("Serialization to never fail"));
            crate::task_state_view!(task_state, url = task_source.expect("Making TaskSource failed.").make().expect("Making Task failed.").r#do().expect("Task failed."));
            assert!(expectation.satisfied_by(&task_state).is_ok_and(|x| x), "The expectation failed.\nExpectation: {expectation:?}\ntask_state: {task_state:?}");
        }

        println!();
    }
}

/// An individual test.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Test {
    /// The [`TaskConfig`].
    pub task_config: TaskConfig,
    /// The expected result.
    pub expectation: Condition
}
