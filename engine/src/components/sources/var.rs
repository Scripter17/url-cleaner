//! [`VarSource`].

use std::env;

use crate::prelude::*;

/// Gets a var from somewhere.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(remote = "Self")]
pub enum VarSource {
    /// Get it from [`Params::vars`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, VarSourceError))]
    Params(StringSource),
    /// Get it from [`TaskContext::vars`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, VarSourceError))]
    TaskContext(StringSource),
    /// Get it from [`JobContext::vars`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, VarSourceError))]
    JobContext(StringSource),
    /// Get it from [`FunctionArgs::vars`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, VarSourceError))]
    FunctionArg(StringSource),
    /// Get it from [`std::env::var`].
    ///
    /// Even though [`std::env::var`] returns an [`Err`] when the environment variable isn't present, this instead returns [`None`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, VarSourceError))]
    ///
    /// If the environment variable exists but isn't valid UTF-8, returns the error [`VarSourceError::EnvVarIsNotUtf8`].
    Env(StringSource)
}

string_or_struct_magic!(VarSource);

impl VarSource {
    /// Get the var.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn get<'j: 't, 't>(&'j self, task_state: &'t TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Option<Cow<'t, str>>, VarSourceError> {
        debug!(VarSource::get, self; match self {
            Self::Params(StringSource::String(x)) => Ok(task_state.job.cleaner.params.vars.get(x).map(|x| Cow::Borrowed(x.as_str()))),
            x => x._get(task_state, args)
        })
    }

    /// [`Self::get`].
    fn _get<'j: 't, 't>(&'j self, task_state: &'t TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Option<Cow<'t, str>>, VarSourceError> {
        Ok(match self {
            Self::Params     (name) => task_state.job.cleaner.params.vars             .get(get!(&name)).map(|x| Cow::Borrowed(x.as_str())),
            Self::TaskContext(name) => task_state.context.vars                        .get(get!(&name)).map(|x| Cow::Borrowed(x.as_str())),
            Self::JobContext (name) => task_state.job.context.vars                    .get(get!(&name)).map(|x| Cow::Borrowed(x.as_str())),
            Self::FunctionArg    (name) => args.ok_or(VarSourceError::NotInFunction)?.vars.get(get!(&name)).map(|x| Cow::Borrowed(x.as_str())),
            Self::Env   (name) => match env::var(get!(&name)) {
                Ok(value) => Some(Cow::Owned(value)),
                Err(env::VarError::NotPresent) => None,
                Err(env::VarError::NotUnicode(_)) => Err(VarSourceError::EnvVarIsNotUtf8)?
            }
        })
    }
}

impl FromStr for VarSource {
    type Err = std::convert::Infallible;

    fn from_str(name: &str) -> Result<VarSource, Self::Err> {
        Ok(name.into())
    }
}

impl From<StringSource> for VarSource {
    fn from(name: StringSource) -> Self {
        Self::Params(name)
    }
}

impl From<String> for VarSource {
    fn from(name: String) -> Self {
        Self::Params(name.into())
    }
}

impl From<&str> for VarSource {
    fn from(name: &str) -> Self {
        Self::Params(name.into())
    }
}

impl Suitability for VarSource {
    fn assert_suitability(&self, config: &Cleaner) {
        match self {
            Self::Params     (StringSource::String(name)) => assert!(config.docs.params.vars       .contains_key(name), "Undocumented Var: {name}"),
            Self::TaskContext(StringSource::String(name)) => assert!(config.docs.task_context.vars .contains_key(name), "Undocumented TaskContext var: {name}"),
            Self::JobContext (StringSource::String(name)) => assert!(config.docs.job_context.vars  .contains_key(name), "Undocumented JobContext var: {name}"),
            Self::FunctionArg(_) => {},
            Self::Env        (StringSource::String(name)) => assert!(config.docs.environment_vars.0.contains_key(name), "Undocumented Env var: {name}"),
            _ => panic!("Unsuitable VarSource: {self:?}")
        }
    }
}

/// The enum of errors [`VarSource::get`] can return.
#[derive(Debug, Error)]
pub enum VarSourceError {
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] Box<StringSourceError>),
    /// [`StringNotFound`].
    #[error(transparent)]
    StringNotFound(#[from] StringNotFound),
    /// Returned when the value of an environment variable isn't valid UTF-8.
    #[error("The value of the environment variable wasn't valid UTF-8")]
    EnvVarIsNotUtf8,

    /// Returned when attempting to use [`FunctionArgs`] outside a function.
    #[error("Attempted to use FunctionArgs outside a function.")]
    NotInFunction
}

impl From<StringSourceError> for VarSourceError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}
