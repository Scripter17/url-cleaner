//! Foundational types containing the bulk of a [`Cleaner`]'s logic.

pub use better_url::*;

pub mod url_part;
pub use url_part::*;
pub mod host_part;
pub use host_part::*;
pub mod cleaner;
pub use cleaner::*;
pub mod conditions;
pub use conditions::*;
pub mod actions;
pub use actions::*;
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
pub mod job;
pub use job::*;
pub mod named_partitioning;
pub use named_partitioning::*;
pub mod set;
pub use set::*;
pub mod map;
pub use map::*;
pub mod refs;
pub use refs::*;
pub mod unthreader;
pub use unthreader::*;
