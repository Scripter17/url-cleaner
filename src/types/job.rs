//! Framework for bulk processing.

pub mod job;
pub use job::*;
pub mod job_context;
pub use job_context::*;
pub mod lazy_task_config;
pub use lazy_task_config::*;
pub mod task_config;
pub use task_config::*;
pub mod lazy_task;
pub use lazy_task::*;
pub mod task;
pub use task::*;
pub mod task_context;
pub use task_context::*;
pub mod task_state;
pub use task_state::*;
pub mod scratchpad;
pub use scratchpad::*;
