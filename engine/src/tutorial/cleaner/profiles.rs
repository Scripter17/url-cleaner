//! # [`Profiles`]
//!
//! Often you'll have only a handful of [`ParamsDiff`]s you want to switch between and share between multiple URL Cleaner Engine frontends (such as URL Cleaner Site and URL Cleaner Discord App).
//!
//! [`Profiles`] allow for this to be done with minimal performance hits.
//!
//! [`Profiles`] are created via a [`ProfilesConfig`], which in JSON looks like this:
//!
//! ```Json
//! {
//!   "base": {
//!     "params_diff": {
//!       "flags": ["flags", "to", "add", "for", "all", "profiles"]
//!     }
//!   },
//!   "profiles": {
//!     "additional_changes": {
//!       "params_diff": {
//!         "flags": ["flags to add", "for only this profile"]
//!       }
//!     }
//!   }
//! }
//! ```
//!
//! As stated in the above [`ProfilesConfig`], the [`ProfilesConfig::base`] [`ProfileConfig`] is applied to all profiles in the resulting [`ProfiledCleaner`].
//!
//! Each [`ProfileConfig`] in [`ProfilesConfig::profiles`] is then applied on top of the base profile.
//!
//! In the above example the `additional_changes` profile has the flags `["flags", "to", "add", "for", "all", "profiles", "flags to add", "for only this profile"]`.
