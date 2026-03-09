//! Do tests.

use serde::Deserialize;

use super::prelude::*;

/// Do tests.
#[derive(Debug, Parser)]
pub struct Args {
    /// The Cleaner to test.
    #[arg(long)]
    pub cleaner: Option<PathBuf>,
    /// Assert the cleaner's suitability.
    #[arg(long)]
    pub assert_suitability: bool,
    /// The ProfilesConfig to assert suitability with.
    #[arg(long, requires = "assert_suitability")]
    pub profiles: Option<PathBuf>,
    /// The tests file.
    #[arg(long)]
    pub test_suite: Option<PathBuf>,
    /// The filter to decide which jobs to do.
    #[arg(long)]
    pub filter: Option<String>,
}

/// The bundled test suite.
pub const BUNDLED_TEST_SUITE: &str = include_str!("bundled-test-suite.json");

/// A tests file.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct TestSuite {
    /// The [`TestJob`]s.
    pub jobs: Vec<TestJob>
}

/// A job.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct TestJob {
    /// The name.
    pub name: String,
    /// The [`JobContext`].
    #[serde(default)]
    pub job_context: JobContext,
    /// The [`ParamsDiff`].
    #[serde(default)]
    pub params_diff: ParamsDiff,
    /// The [`Test`]s.
    pub tests: Vec<Test>
}

/// A test.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Test {
    /// The task.
    pub task: serde_json::Value,
    /// The expected result.
    pub expect: String
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let cleaner = Box::leak(Box::new(Cleaner::load_or_get_bundled(self.cleaner).unwrap())).borrowed();

        let profiled_cleaner = ProfiledCleanerConfig {
            cleaner,
            profiles_config: self.profiles.map(ProfilesConfig::load_from_file).transpose().unwrap().unwrap_or_default()
        }.make();

        if self.assert_suitability {
            println!("Asserting suitability:");

            profiled_cleaner.assert_suitability();

            println!();
        }

        let cleaner = profiled_cleaner.into_base();

        let test_suite: TestSuite = match self.test_suite {
            Some(path) => serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap(),
            None       => serde_json::from_str(BUNDLED_TEST_SUITE).unwrap()
        };

        let filter = Regex::new(&self.filter.unwrap_or_default()).unwrap();

        let cache = Cache {
            inner: &CacheLocation::Memory.into(),
            config: CacheConfig {
                read : false,
                write: false,
                delay: false
            }
        };
        let http_client = &HttpClient::default();

        for test_job in test_suite.jobs {
            if filter.find(&test_job.name).is_none() {
                continue;
            }

            println!("Job:");
            println!("  Name: {}", test_job.name);
            println!("  Params diff: {}", serde_json::to_string(&test_job.params_diff).unwrap());
            println!("  Job context: {}", serde_json::to_string(&test_job.job_context).unwrap());
            println!("  Tests:");

            let mut cleaner = cleaner.borrowed();

            test_job.params_diff.apply(&mut cleaner.params);

            let job = Job {
                context: test_job.job_context,
                cleaner,
                unthreader: &Unthreader::Off,
                cache,
                http_client
            };

            for test in test_job.tests {
                println!("    {}", test.task);

                let cleaned = job.r#do(test.task).unwrap();
                assert_eq!(cleaned, test.expect, "Expectation failed.");

                let recleaned = job.r#do(cleaned.clone()).unwrap();
                assert_eq!(recleaned, cleaned, "Idempotence failed.");
            }

            println!();
        }
    }
}
