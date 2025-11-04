//! Components.

pub mod condition;
pub mod action;
pub mod flag_source;
pub mod var_source;
pub mod set;
pub mod map;
pub mod partitioning;
pub mod url_part;
pub mod host_part;
pub mod query_param_selector;

pub mod string_source;
pub mod string_modification;
pub mod string_location;
pub mod string_matcher;
pub mod char_matcher;

pub mod regex;
pub mod base64;

pub mod parsing;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::condition::*;
    pub use super::action::*;
    pub use super::flag_source::*;
    pub use super::var_source::*;
    pub use super::set::*;
    pub use super::map::*;
    pub use super::partitioning::*;
    pub use super::url_part::*;
    pub use super::host_part::*;
    pub use super::query_param_selector::*;

    pub use super::string_source::*;
    pub use super::string_modification::*;
    pub use super::string_location::*;
    pub use super::string_matcher::*;
    pub use super::char_matcher::*;

    pub use super::regex::prelude::*;
    pub use super::base64::prelude::*;

    pub use super::parsing::prelude::*;
}
