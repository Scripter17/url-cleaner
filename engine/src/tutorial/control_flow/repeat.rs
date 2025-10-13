//! # Repeat
//!
//! Sometimes, an action needs to be applied multiple times.
//!
//! This mainly happens when cleaning redirect URLs. If you handle `t.co` after `bit.ly`, then a `bit.ly` redirect going to a `t.co` redirect won't be fully expanded.
//!
//! Using [`Action::Repeat`] (and, for convenience, [maps]) allows for handling redirect chains of any length and in any order.
//!
//! ```Json
//! {"Repeat": {
//!   "actions": [
//!     {"PartMap": {
//!       "part": "Host",
//!       "map": {
//!         "t.co": "ExpandRedirect",
//!         "bit.ly": "ExpandRedirect",
//!         "youtube.com": {"RemoveQueryParam": "si"}
//!       }
//!     }}
//!   ]
//! }}
//! ```
//!
//! Let's say you give the above example a `t.co` URL that redirects to a `bit.ly` URL that redirects to a `youtube.com` URL with an `si` query parameter.
//!
//! 1. `Repeat` makes a backup of the current [`TaskState`] to later compare with the [`TaskState`] at the end of the loop.
//!
//! 2. `Repeat` applies its contained actions once. Here it will expand the `t.co` URL into the intermediate `bit.ly` URL[^redirect_policy].
//!
//! [^redirect_policy]: Technically this is only true if the [`HttpClientConfig::redirect_policy`] is set to [`RedirectPolicy::None`], which for the default cleaner is usually true as it's set in the [`Params::http_client_config`] used by all HTTP requests.
//!
//! 3. Once all the contained actions are applied, `Repeat` will compare the modified [`TaskState`] with the backup made in step 1.
//!
//! 4. Because the values are different, `Repeat` does steps 1 through 3 again.
//!
//! 5. At the start of the second iteration, `Repeat` once again makes a backup of the current task's state.
//!
//! 7. The second iteration expands the `bit.ly` URL into the `youtube.com` URL with an `si` query parameter.
//!
//! 8. As with step 4, the task's state has changed, and thus `Repeat` will run a third time.
//!
//! 9. The third iteration removes the `si` query parameter from the `youtube.com` URL and then runs a fourth time.
//!
//! 10. On the fourth iteration, while the [`Action::RemoveQueryParam`] *is* applied, the end result is identical to the backup made at the start of the loop, and therefore `Repeat` exits and does not run a fifth time.
//!
//! By default, [`Action::Repeat`] applies its contained actions at most 10 times, but this limit can be increased up to [`u64::MAX`]. The limit is that high because I wanted to make a Turing Machine.
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
//! To validate this intuition, see the [global debugging](debugging#global) tutorial.

pub(crate) use super::*;

