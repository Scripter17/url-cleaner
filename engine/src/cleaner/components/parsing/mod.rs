//! Parsing.

pub mod percent_encoding;

pub mod get_html_attribute;
pub mod unescape_html;
pub mod get_html_char_ref;

pub mod get_js_string_literal_prefix;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::percent_encoding::*;

    pub use super::get_html_attribute::*;
    pub use super::unescape_html::*;
    pub use super::get_html_char_ref::*;

    pub use super::get_js_string_literal_prefix::*;
}
