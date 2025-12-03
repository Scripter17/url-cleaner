//! [`TestSet`].

use crate::*;

/// Rules for how to construct a [`Job`] and the [`Test`]s to run on it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestSet {
    /// The [`ParamsDiff`] to apply to the [`Cleaner`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub params_diff: Option<ParamsDiff>,
    /// The [`JobContext`] to use.
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
    pub fn r#do(self, mut cleaner: Cleaner) -> Result<(), DoTestSetError> {
        println!(
            "TestSet\n  params_diff: {}",
            serde_json::to_string(&self.params_diff).expect("Serialization to never fail.")
        );

        if let Some(params_diff) = self.params_diff {
            params_diff.apply(&mut cleaner.params);
        }

        let job = &Job {
            context: self.job_context,
            cleaner,
            unthreader: &Default::default(),
            cache: Cache {
                config: CacheConfig {
                    read : false,
                    write: false,
                    delay: false,
                },
                inner: &Default::default()
            },
            http_client: &Default::default()
        };

        let mut ret = Ok(());

        for test in self.tests {
            println!("    Test: {}", serde_json::to_string(&test).expect("Serialition to never fail."));
            match test.r#do(job) {
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


