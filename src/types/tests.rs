//! Allows including tests in the [`Config`],

use std::borrow::Cow;
use std::str::FromStr;

use serde::{Serialize, Deserialize};
use url::Url;

use crate::types::*;
use crate::util::*;

/// The main API for running tests.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tests {
    /// The [`TestSet`]s to run.
    pub sets: Vec<TestSet>
}

impl Tests {
    /// Run all the tests.
    /// # Panics
    /// If a test fails, panics.
    pub fn r#do(&self, config: &Config) {
        for set in self.sets.iter() {
            set.r#do(config)
        }
    }
}

/// A group of [`Test`]s that share a [`ParamsDiff`] and [`JobsContext`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestSet {
    /// The [`ParamsDiff`] to use.
    #[serde(default, skip_serializing_if = "is_default")]
    pub params_diff: Option<ParamsDiff>,
    /// The [`JobsContext`] to use.
    #[serde(default, skip_serializing_if = "is_default")]
    pub jobs_context: JobsContext,
    /// The [`Test`]s to run.
    pub tests: Vec<Test>
}

impl TestSet {
    /// Runs all the tests
    /// # Panics
    /// If a test fails, panics.
    pub fn r#do(&self, config: &Config) {
        let mut config = Cow::Borrowed(config);
        if let Some(params_diff) = &self.params_diff {
            params_diff.apply(&mut config.to_mut().params);
        }

        let (configs, results) = self.tests.clone().into_iter().map(|Test {job_config, result}| (job_config, result)).collect::<(Vec<_>, Vec<_>)>();

        let mut jobs = Jobs {
            jobs_config: JobsConfig {
                config,
                #[cfg(feature = "cache")]
                cache: Default::default()
            },
            context: Cow::Borrowed(&self.jobs_context),
            job_configs_source: Box::new(configs.into_iter().map(|x| serde_json::from_value(x).map_err(Into::into)))
        };

        for (i, (job, result)) in jobs.iter().zip(results.into_iter()).enumerate() {
            assert_eq!(
                job.expect("The job to be makable.").r#do().expect("The job to succeed."),
                Url::from_str(&result).expect("The result to be a valid BetterUrl."),
                "Test failed\nparams_diff: {:?}\njobs_context: {:?}\ntest: {:?}",
                self.params_diff,
                self.jobs_context,
                self.tests.get(i).expect("`i` to never be out of bounds.")
            );
        }
    }
}

/// An individual test.
///
/// Needs the config from a [`TestSet`] to be run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Test {
    /// The [`JobConfig`] to use.
    pub job_config: serde_json::Value,
    /// The expected result URL.
    pub result: String
}
