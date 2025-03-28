//! Framework for bulk processing.

mod job;
pub use job::*;
mod job_context;
pub use job_context::*;
mod task_config;
pub use task_config::*;
mod task;
pub use task::*;
mod task_context;
pub use task_context::*;
mod task_state;
pub use task_state::*;
mod task_scratchpad;
pub use task_scratchpad::*;
