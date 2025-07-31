//! # Repeat
//!
//! Sometimes, an action needs to be applied multiple times.
//!
//! This mainly happens when cleaning redirect URLs. If you handle `t.co` after `bit.ly`, then a `bit.ly` redirect going to a `t.co` redirect won't be fully expanded.
//!
//! Hereis an example derived from the [maps] tutorial.
//!
//! ```Json
//! {
//!   "actions": [
//!     {"Repeat": {
//!       "actions": [
//!         {"PartMap": {
//!           "part": "Host",
//!           "map": {
//!             "bit.ly": "ExpandRedirect",
//!             "youtube.com": {"RemoveQueryParam": "si"}
//!           }
//!         }}
//!       ]
//!     }}
//!   ]
//! }
//! ```
//!
//! If this cleaner is given a bit.ly redirect it will
//!
//! 1. Expand the redirect.
//!
//! 2. If the new URL is a youtube.com URL, remove the `si` query parameter if it exists.
//!
//! 3. If an `si` query parameter was found and therefore removed, apply the `PartMap` a third time.
//!
//! Since the third iteration leaves the URL unchanged, the `Repeat` action then finishes and the cleaner moves on to the next action.
//!
//! By default, `Repeat` actions will apply their contained actions at most 10 times. The limit can be raised up to at most 18,446,744,073,709,551,615 to accomodate for making a turing machine, which I did.
//!
//! ## Reverted changes
//!
//! It's important to note that `Repeat` won't loop if a URL is changed then the change is reverted. It only cares if the state at the end of a loop is the same as the state at the start of that loop.
//!
//! For example, assuming you don't intentionally give this cleaner a URL with an `unused_parameter` query parameter to make what I'm about to say wrong, this `Repeat` will only apply its actions once.
//!
//! ```Json
//! {
//!   "actions": [
//!     {"Repeat": {
//!       "actions": [
//!         {"SetQueryParam": {
//!           "query_param": "unused_parameter",
//!           "value": "whatever"
//!         }},
//!         {"RemoveQueryParam": "unused_parameter"}
//!       ]
//!     }}
//!   ]
//! }
//! ```
//!
//! To validate this intuition, see the [debugging] tutorial.

pub(crate) use super::*;

