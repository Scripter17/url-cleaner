//! [`Docs`].

use crate::prelude::*;

mod var         ; pub use var         ::*;
mod params      ; pub use params      ::*;
mod job_context ; pub use job_context ::*;
mod task_context; pub use task_context::*;
mod secrets     ; pub use secrets     ::*;

/// Documentation for a [`Cleaner`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct Docs {
    /// The name.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub name: Option<String>,
    /// The description lines.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub description: Vec<String>,
    /// The [`ParamsDocs`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub params: ParamsDocs,
    /// The [`JobContextDocs`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub job_context: JobContextDocs,
    /// The [`TaskContextDocs`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub task_context: TaskContextDocs,
    /// The [`SecretsDocs`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub secrets: SecretsDocs,
}
