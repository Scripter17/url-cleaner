//! Basic testing framework to ensure configs are working as intended.

use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use url::Url;

use crate::types::*;
#[allow(unused_imports, reason = "Needed for doc links.")]
use crate::glue::*;
#[allow(unused_imports, reason = "Needed for doc links.")]
use crate::util::*;

/// Tests to make sure a [`Config`] is working as intended.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TestSet {
    /// The [`ParamsDiff`] to apply to the [`Config::params`] for this test.
    #[serde(default, skip_serializing_if = "is_default")]
    pub params_diff: Option<ParamsDiff>,
    /// A list of URLs to test and the expected results.
    pub expectations: Vec<Expectation>
}

impl TestSet {
    /// Runs the tests.
    /// # Panics
    /// Panics if a call to [`Expectation::run`] panics.
    pub fn run(&self, config: &Config) {
        println!("Testing the following test set:\n{}", serde_json::to_string(self).expect("The entire config to be serializable"));
        let mut config = Cow::Borrowed(config);
        if let Some(params_diff) = &self.params_diff {
            params_diff.apply(&mut config.to_mut().params);
        }
        for expectation in &self.expectations {
            expectation.run(&config);
        }
    }
}

/// Individual [`TestSet`] test.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Expectation {
    /// The URL to clean.
    pub job_config: JobConfig,
    /// The expected result of cleaning [`Self::job_config`].
    pub result: Url
}

impl Expectation {
    /// Runs the test.
    /// # Panics
    /// If serializing `self` fails, panics.
    /// 
    /// If the call to [`Config::apply`] fails, panics.
    /// 
    /// If the expectation fails, you guessed it, panics.
    pub fn run(&self, config: &Config) {
        println!("Testing the following expectation:\n{}", serde_json::to_string(self).expect("The entire config to be serializable"));
        let mut url = self.job_config.url.clone();
        config.apply(&mut JobState {
            url: &mut url,
            params: &config.params,
            scratchpad: &mut Default::default(),
            context: &self.job_config.context,
            #[cfg(feature = "cache")]
            cache: &Default::default(),
            commons: &config.commons,
            common_args: None
        }).expect("The URL to be modified without errors.");
        assert_eq!(url, self.result);
    }
}
