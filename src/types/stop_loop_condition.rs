//! A simple condition system to determine if and when a [`Rule::Repeat`] should stop looping before the limit.

use serde::{Serialize, Deserialize};
use url::Url;

use crate::types::*;
use crate::util::*;

/// A simple condition system to determine if and when a [`Rule::Repeat`] should stop looping before the limit.
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
    /// Passes if the [`JobState`]'s [`JobState::scratchpad`]'s end value is the same as its start value.
    NoScratchpadChange
}

impl Default for StopLoopCondition {
    /// Defaults to `Self::Any(vec![Self::NonePass, Self::All(vec![Self::NoUrlChange, Self::NoScratchpadChange])]).
    fn default() -> Self {
        Self::Any(vec![Self::NonePass, Self::All(vec![Self::NoUrlChange, Self::NoScratchpadChange])])
    }
}

impl StopLoopCondition {
    /// See each variant of [`Self`] for details.
    pub fn satisfied_by(&self, job_state: &JobStateView, none_passed: bool, previous_url: &Url, previous_scratchpad: &JobScratchpad) -> bool {
        debug!(StopLoopCondition::satisfied_by, self, job_state, none_passed, previous_url, previous_scratchpad);
        match self {
            Self::All(conditions) => conditions.iter().all(|condition| condition.satisfied_by(job_state, none_passed, previous_url, previous_scratchpad)),
            Self::Any(conditions) => conditions.iter().any(|condition| condition.satisfied_by(job_state, none_passed, previous_url, previous_scratchpad)),
            Self::Not(condition ) => !condition.satisfied_by(job_state, none_passed, previous_url, previous_scratchpad),
            Self::NonePass => none_passed,
            Self::NoUrlChange => job_state.url == previous_url,
            Self::NoScratchpadChange => job_state.scratchpad == previous_scratchpad
        }
    }
}
