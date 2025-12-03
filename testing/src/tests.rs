//! [`Tests`].

use crate::*;

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
    pub fn r#do(self, cleaner: Cleaner) -> Result<(), DoTestsError> {
        let mut ret = Ok(());

        for set in self.sets {
            if set.r#do(cleaner.borrowed()).is_err() {
                ret = Err(DoTestsError);
            }
        }

        ret
    }
}

