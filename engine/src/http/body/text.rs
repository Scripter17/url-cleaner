//! [`HttpTextBodySource`].

use crate::prelude::*;

/// [`StringSource`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub struct HttpTextBodySource(pub StringSource);

impl HttpTextBodySource {
    /// [`StringSource::get`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`StringNotFound`].
    pub fn get<'j: 't, 't>(&'j self, task_state: &'t TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Cow<'t, str>, HttpTextBodySourceError> {
        Ok(self.0.get(task_state, args)?.ok_or(StringNotFound)?)
    }
}
