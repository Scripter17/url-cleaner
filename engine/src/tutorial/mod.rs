#![allow(unused_imports, reason = "They're for docs.")]

//! # Tutorial
//!
//! A high level explanation of URL Cleaner.
//!
//! ## Main goal
//!
//! The main goal of URL Cleaner is to make advertisers, trackers, and wrongthink reporters eat shit by removing tracking garbage from URLs.
//!
//! If you think politics shouldn't be involved in privacy tools, you are not prepared for what's coming.
//!
//! If you think swearing shouldn't be in docmuentation, try to get my hatred accross without swearing and see where that gets you.
//!
//! ## Terminology
//!
//! - A "job" is a sequence of "tasks" and configuration like the cleaner, cache, etc. to use for those tasks.
//!
//! - A "task" is a URL to clean as well as optional context, such as the text of the link it came from.
//!
//! - "Component" is a generic term for URL Cleaner Engine types like [`Action`], [`Condition`], [`StringSource`], [`UrlPart`], and so on.
//!   In general, if it shows up inside a [`Cleaner::actions`], it's a component.

use std::collections::{HashSet, HashMap};

use url::Url;

pub(crate) use crate::types::*;
pub(crate) use crate::glue::*;

pub mod cleaner;
pub(crate) use cleaner::*;
pub mod debugging;
pub(crate) use debugging::*;
pub mod job;
pub(crate) use job::*;
