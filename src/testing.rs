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
    pub fn r#do(self, config: &Config) {
        for set in self.sets {
            set.r#do(config)
        }
    }
}

/// Rules for how to construct a [`JobsSource`] from a [`Config`] and the [`Test`]s to run on it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestSet {
    /// The [`ParamsDiff`] to apply to the [`Config`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub params_diff: Option<ParamsDiff>,
    /// The [`JobsContext`] to give to the [`JobsSource`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub jobs_context: JobsContext,
    /// The [`Test`]s to run.
    pub tests: Vec<Test>
}

impl TestSet {
    /// Do the tests, panicking if any fail.
    /// # Panics
    /// If a value from [`JobsSource::iter`] is an error, panics.
    ///
    /// If a call to [`Job::do`] returns an error, panics.
    /// 
    /// If any test fails, panics.
    pub fn r#do(self, config: &Config) {
        let mut config = Cow::Borrowed(config);

        let params_diff_json = serde_json::to_string(&self.params_diff).expect("Serialization to never fail");
        
        if let Some(params_diff) = self.params_diff {
            params_diff.apply(&mut config.to_mut().params);
        }

        let (job_configs, results) = self.tests.clone().into_iter().map(|Test {job_config, result}| (job_config, result)).collect::<(Vec<_>, Vec<_>)>();

        let mut jobs = JobsSource {
            jobs_config: JobsConfig {
                config,
                #[cfg(feature = "cache")]
                cache: Default::default()
            },
            context: Cow::Borrowed(&self.jobs_context),
            job_configs_source: Box::new(job_configs.into_iter().map(Ok))
        };

        for (i, (job, result)) in jobs.iter().zip(results).enumerate() {
            assert_eq!(
                job.expect("The job to be makeable.").r#do().expect("The job to succeed."),
                result,
                "Test failed\nparams_diff: {params_diff_json}\njobs_context: {}\ntest: {}",
                serde_json::to_string(&self.jobs_context).expect("Serialization to never fail"),
                serde_json::to_string(self.tests.get(i).expect("`i` to never be out of bounds.")).expect("Serialization to never fail")
            );
        }
    }
}

/// An individual test.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Test {
    /// The [`JobConfig`].
    pub job_config: JobConfig,
    /// The expected result.
    pub result: BetterUrl
}
