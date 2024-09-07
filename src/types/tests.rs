//! Basic testing framework to ensure configs are working as intended.

use serde::{Serialize, Deserialize};
use url::Url;

use crate::types::*;
#[allow(unused_imports, reason = "Needed for doc links.")]
use crate::glue::*;

/// Tests to make sure a [`Config`] is working as intended.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TestSet {
    /// The [`ParamsDiff`] to apply to the [`Config::params`] for this test.
    #[serde(default)]
    pub params_diff: ParamsDiff,
    /// A list of URLs to test and the expected results.
    pub expectations: Vec<Expectation>
}

impl TestSet {
    /// Runs the tests.
    /// # Panics
    /// Panics if a call to [`Expectation::run`] panics.
    pub fn run(&self, mut config: Config) {
        println!("Testing the following test set:\n{}", serde_json::to_string(self).expect("The entire config to be serializable")); // Only applies when testing a config.
        self.params_diff.apply(&mut config.params);
        for expectation in &self.expectations {
            expectation.run(&config);
        }
    }
}

/// Individual [`TestSet`] test.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Expectation {
    /// The URL to clean.
    before: Url,
    /// The expected result of cleaning [`Self::before`].
    after: Url
}

impl Expectation {
    /// Runs the test.
    /// # Panics
    /// Panics if a making the [`CacheHandler`], a call to [`Rules::apply`], or a test fails.
    pub fn run(&self, config: &Config) {
        println!("Testing the following expectation set:\n{}", serde_json::to_string(self).expect("The entire config to be serializable")); // Only applies when testing a config.
        let mut temp = self.before.clone();
        let context = Default::default();
        #[cfg(feature = "cache")]
        let cache_handler = config.cache_path.as_path().try_into().expect("The cache handler path to be valid UTF-8");
        config.rules.apply(&mut JobState {
            url: &mut temp,
            params: &config.params,
            vars: Default::default(),
            context: &context,
            #[cfg(feature = "cache")]
            cache_handler: &cache_handler,
            commons: &config.commons,
            common_vars: None
        }).expect("The URL to be modified without errors."); // Only applies when testing a config.
        assert_eq!(temp, self.after);
    }
}
