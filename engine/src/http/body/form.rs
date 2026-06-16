//! [`HttpFormBodySource`].

use crate::prelude::*;

/// A source for an HTTP form body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub struct HttpFormBodySource(pub HashMap<String, StringSource>);

/// The enum of errors [`HttpFormBodySource::get`] can return.
#[derive(Debug, Error)]
pub enum HttpFormBodySourceError {
    /// [`StringSourceError`].
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
}

impl HttpFormBodySource {
    /// Get a HTTP form body.
    /// # Errors
    /// If any call to [`StringSource::get`] returns an error, that error is returned.
    pub fn get<'j: 't, 't>(&'j self, task_state: &'t TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<HashMap<&'j str, Cow<'t, str>>, HttpFormBodySourceError> {
        let mut ret = HashMap::default();

        for (name, value) in self.0.iter() {
            if let Some(value) = get!(?value) {
                ret.insert(&**name, value);
            }
        }

        Ok(ret)
    }
}
