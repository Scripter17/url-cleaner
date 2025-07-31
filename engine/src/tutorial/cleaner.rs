//! # [`Cleaner`]
//!
//! A [`Cleaner`] is the bulk configuration unit for URL Cleaner Engine. It contains all the logic for how to clean URLs as well as documentation about itself.
//!
//! The following cleaner always removes the `utm_source` query parameter and, if the `https_upgrade` flag is enabled, upgrades HTTP URLs to HTTPS.
//!
//! ```Json
//! {
//!   "docs": {
//!     "name": "Example cleaner",
//!     "description": [
//!       "An example cleaner"
//!     ],
//!     "flags": {
//!       "https_upgrade": "Upgrade HTTP URLs to HTTPS"
//!     }
//!   },
//!   "actions": [
//!     {"RemoveQueryParam": "utm_source"},
//!     {"If": {
//!       "if": {"All": [
//!         {"FlagIsSet": "https_upgrade"},
//!         {"SchemeIs": "http"}
//!       ]},
//!       "then": {"SetScheme": "https"}
//!     }}
//!   ]
//! }
//! ```
//!
//! The "[docs](Cleaner::docs)" field contains the [`CleanerDocs`]. It's optional but polite to add.
//!
//! The "[actions](Cleaner::actions)" field contains a list of [`Action`]s to do. The first action, written as `{"RemoveQueryParam": "utm_source"}` removes any "utm_source" query param found in the URL.
//!
//! The second action, the "[If](Action::If)" action, applies the action in its "[then](Action::If::then)" field if and only if the condition in its "[if](Action::If::if)" field is "satisfied".
//! In this case, the condition is satisfied if the "https_upgrade" flag is set and the URL's scheme is "http".
//!
//! Unlike AdGuard/uBlock Origin filters, URL Cleaner actions are applied in order of declaration.
//! This has the benefit of letting you build far more complex operations out of far simpler building blocks, but has the downside of being very easy to write slow cleaners.

pub(crate) use super::*;

pub mod params;
pub(crate) use params::*;
pub mod default_cleaner;
pub(crate) use default_cleaner::*;
pub mod commons;
pub(crate) use commons::*;
pub mod string_modification;
pub(crate) use string_modification::*;
pub mod string_source;
pub(crate) use string_source::*;
pub mod action;
pub(crate) use action::*;
pub mod control_flow;
pub(crate) use control_flow::*;
pub mod glue;
pub(crate) use glue::*;
pub mod url_part;
pub(crate) use url_part::*;
pub mod set;
pub(crate) use set::*;
