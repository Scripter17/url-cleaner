//! Framework for bulk processing.

mod jobs_source;
pub use jobs_source::*;
mod jobs_context;
pub use jobs_context::*;
mod job_config;
pub use job_config::*;
mod job;
pub use job::*;
mod job_context;
pub use job_context::*;
mod job_state;
pub use job_state::*;
mod job_scratchpad;
pub use job_scratchpad::*;
