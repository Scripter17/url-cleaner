//! Various types to make URL Cleaner far more powerful.

use std::collections::HashMap;

use thiserror::Error;

pub mod url_part;
pub use url_part::*;
pub mod config;
pub use config::*;
pub mod tests;
pub use tests::*;
pub mod rules;
pub use rules::*;
pub mod string_location;
pub use string_location::*;
pub mod string_modification;
pub use string_modification::*;
pub mod string_source;
pub use string_source::*;
pub mod string_matcher;
pub use string_matcher::*;
pub mod char_matcher;
pub use char_matcher::*;
pub mod jobs;
pub use jobs::*;
pub mod stop_loop_condition;
pub use stop_loop_condition::*;
