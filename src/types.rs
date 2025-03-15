//! The core tools of URL Cleaner.

pub mod better_url;
pub use better_url::*;
pub mod url_part;
pub use url_part::*;
pub mod config;
pub use config::*;
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
pub mod named_partitioning;
pub use named_partitioning::*;
pub mod map;
pub use map::*;
pub mod error_behavior;
pub use error_behavior::*;

use crate::util::*;
