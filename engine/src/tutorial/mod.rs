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
//!
//! - Some components like [`Condition`] and [`StringMatcher`] can be "satisfied". A component is satisfied with the input to its `check` method when it returns `Ok(true)` and unsatisfied when it returns `Ok(false)`.
//!
//! - Some components like [`Action`] and [`StringModification`] can be "applied" to stuff. This means it takes in some value and changes that value.
//!
//! - Some components are said to have a "value". This refers to the return value of their main/geting method, such as [`Condition::check`], [`StringSource::get`], [`StringMatcher::check`], and [`UrlPart::get`].

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
