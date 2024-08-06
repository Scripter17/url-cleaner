//! A simple condition system to determing if and when a [`Rule::Repeat`] should stop looping before the limit.

use serde::{Serialize, Deserialize};
use url::Url;

use crate::types::*;
use crate::util::*;

/// A simple condition system to determing if and when a [`Rule::Repeat`] should stop looping before the limit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StopLoopCondition {
    /// Passes if all contained [`Self`]s pass.
    All(Vec<Self>),
    /// Passes if any contained [`Self`]s pass.
    Any(Vec<Self>),
    /// Passes if the contained [`Self`] fails.
    Not(Box<Self>),
    /// Passes if none of the [`Rule`] pass.
    /// 
    /// See [`Rule::Repeat`] for details.
    NonePass,
    /// Passes if the [`Url`]'s end value is the same as its start value.
    NoUrlChange,
    /// Passes if the [`JobState`]'s [`JobState::vars`]'s end value is tha same as its start value.
    NoJobVarChange
}

impl Default for StopLoopCondition {
    /// Defaults to `Self::Any(vec![Self::NonePass, Self::All(vec![Self::NoUrlChange, Self::NoJobVarChange])]).
    fn default() -> Self {
        Self::Any(vec![Self::NonePass, Self::All(vec![Self::NoUrlChange, Self::NoJobVarChange])])
    }
}

impl StopLoopCondition {
    /// See each vaeiant of [`Self`] for details.
    pub fn satisfied_by(&self, job_state: &JobState, none_passed: bool, previous_url: &Url, previous_job_vars: &HashMap<String, String>) -> bool {
        debug!(StopLoopCondition::satisfied_by, self, job_state, none_passed, previous_url, previous_job_vars);
        match self {
            Self::All(conditions) => conditions.iter().all(|condition| condition.satisfied_by(job_state, none_passed, previous_url, previous_job_vars)),
            Self::Any(conditions) => conditions.iter().any(|condition| condition.satisfied_by(job_state, none_passed, previous_url, previous_job_vars)),
            Self::Not(condition ) => !condition.satisfied_by(job_state, none_passed, previous_url, previous_job_vars),
            Self::NonePass => none_passed,
            Self::NoUrlChange => job_state.url == previous_url,
            Self::NoJobVarChange => job_state.vars == *previous_job_vars
        }
    }
}
