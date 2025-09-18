//! # [`Map`]
//!
//! [`Map`]s are key-value pairs with two extra properties:
//!
//! 1. The [`None`]/`null` key is written outside the map in the `"if_none"` field. This is because JSON doesn't let you use `null` as a key in a map.
//!
//! 2. Maps have a fallback value, called `else`, that is returned when using a key not otherwise in the map.
//!
//! Components that use maps "flatten" them, so while [`Action::PartMap`] contains an [`Action::PartMap::map`] field of type [`Map`], you would use it as follows:
//!
//! ```Json
//! {"PartMap": {
//!   "part": "NormalizedHost",
//!   "map": {
//!     "example.com": "The Action to take for example.com URLs",
//!     "example2.com": "The Action to take for example2.com URLs"
//!   },
//!   "if_none": "Optional Action to take if the NormalizedHost is somehow None. This is usually useful for parts like {\"QueryParam\": \"abc\"} that can be expected to be None",
//!   "else": "Optional Action to take if the Normalized Host is not in the map or if the NormalizedHost is None and the if_none field is absent"
//! }}
//! ```
//!
//! Maps excel at flattening certain long if-then-else chains. For example, take this [`Action`]:
//!
//! ```Json
//! {"If": {
//!   "if": {"PathIs": "/search"},
//!   "then": "Do a thing",
//!   "else": {"If": {
//!     "if": {"PathStartsWith": "/user/"},
//!     "then": "Do a different thing",
//!     "else": {"If": {
//!       "if": {"PathStartsWith": "/post/"},
//!       "then": "Do yet another thing"
//!     }}
//!   }}
//! }}
//! ```
//!
//! While not every situation allows for this, the above example can be simplified dramatically by using [maps] and, in this case specifically, [`Action::PartMap`].
//!
//! ```Json
//! {"PartMap": {
//!   "part": {"PathSegment": 0},
//!   "map": {
//!     "search": "Do a thing",
//!     "user": "Do a different thing",
//!     "post": "Do yet another thing"
//!   }
//! }}
//! ```
//!
//! While this is simpler and usually faster (It's a [`HashMap`]), the behavior has been changed slightly.
//!
//! 1. For the first branch, the scope of matching URLs was expanded from only URLs with a path of `/search` to also including URLs with paths like `/search/`, `/search/abc`, and so on.
//!
//! 2. For both the user and post branches, the change results in accepting not just paths of `/user/` and `/post/`, but paths of `/user` and `/post`.
//!
//! While most websites are unlikely to care about these changes (what website has both `/user/user1234` and `/user` as valid paths?), care should be taken to ensure these sorts of changes are relegated only to "invalid" URLs.

pub(crate) use super::*;
