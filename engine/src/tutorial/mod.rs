#![allow(rustdoc::broken_intra_doc_links, reason = "It's fine.")]
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
//! If you think swearing shouldn't be in documentation, try to get my hatred across without swearing and see where that gets you.
//!
//! ## Terminology
//!
#![cfg_attr(    feature = "cache" , doc = "- A \"[job]\" is a sequence of \"[tasks](task)\", configuration like the [cleaner], [cache], etc. to use for those tasks, and optionally some [context](job_context), such as the website the tasks came from.")]
#![cfg_attr(not(feature = "cache"), doc = "- A \"[job]\" is a sequence of \"[tasks](task)\", configuration like the [cleaner], cache, etc. to use for those tasks, and optionally some [context](job_context), such as the website the tasks came from.")]
//!
//! - A "[task]" is a URL to clean as well as optional [context](task_context), such as the text of the link it came from.
//!
//! - "Component" is a generic term for URL Cleaner Engine types like [`Action`], [`Condition`], [`StringSource`], [`UrlPart`], and so on.
//!
//! - Some components like [`Condition`] and [`StringMatcher`] can be "satisfied". A component is satisfied with the input to its `check` method when it returns `Ok(true)` and unsatisfied when it returns `Ok(false)`.
//!
//! - Some components like [`Action`] and [`StringModification`] can be "applied" to stuff. This means it takes in some value and changes that value.
//!
//! - Some components are said to have a "value". This refers to the return value of their main/getting method, such as [`Condition::check`], [`StringSource::get`], [`UrlPart::get`], etc..

use std::collections::{HashSet, HashMap};

use url::Url;

pub(crate) use crate::types::*;
pub(crate) use crate::glue::prelude::*;

pub mod cleaner;
pub(crate) use cleaner::*;
pub mod debugging;
pub(crate) use debugging::*;
pub mod job;
pub(crate) use job::*;
pub mod types;
pub(crate) use types::*;
pub mod control_flow;
pub(crate) use control_flow::*;
