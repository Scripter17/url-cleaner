//! # Debugging
//!
//! ## Local
//!
//! Most components have a variant called `Debug` that takes one of itself.
//!
//! `Debug` variants print information about the current state, the component's inputs (if any), the resulting value of what it modified (if any), and its return value.
//!
//! `Debug` variants are one of the few states a component will be in that will cause [`Cleaner::assert_suitability`] to panic. This is to ensure `Debug` variants aren't accidentally committed to the default cleaner.
//!
//! For example, the action `{"Debug": "None"}` will print the following to STDERR
//!
//! ```Text
//! === Action::Debug ===
//! Old task_state: TaskStateDebugHelper { url: "https://example.com/", scratchpad: Scratchpad { flags: {}, vars: {} }, common_args: None }
//! Return value: Ok(())
//! New task_state: TaskStateDebugHelper { url: "https://example.com/", scratchpad: Scratchpad { flags: {}, vars: {} }, common_args: None }
//! ```
//!
//! As with most debugging features, the exact format is not stable or intended to be machine parsable.
//!
//! ## Global
//!
//! To check our intuition for the [reverted changes example of repeating](repeat#reverted-changes), it's useful to compile URL Cleaner Engine with the `debug` feature enabled to print debug info to STDERR.
//!
//! For example, here is the output of using the cleaner from the [reverted changes](repeat#reverted-changes) example cleaner with `https://example.com` as the input.
//!
//! ```Text
//!   0-   -   -Action::apply
//!             - self: Repeat { actions: [SetQueryParam { query_param: QueryParamSelector { name: "unused_parameter", index: 0 }, value: String("whatever") }, RemoveQueryParam(String("unused_parameter"))], limit: 10 }
//!             - task_state.debug_helper(): TaskStateDebugHelper { url: "https://example.com/", scratchpad: Scratchpad { flags: {}, vars: {} }, common_args: None }
//!   1-   -   -|---Action::apply
//!             |   - self: SetQueryParam { query_param: QueryParamSelector { name: "unused_parameter", index: 0 }, value: String("whatever") }
//!             |   - task_state.debug_helper(): TaskStateDebugHelper { url: "https://example.com/", scratchpad: Scratchpad { flags: {}, vars: {} }, common_args: None }
//!   2-   -   -|---Action::apply
//!             |   - self: RemoveQueryParam(String("unused_parameter"))
//!             |   - task_state.debug_helper(): TaskStateDebugHelper { url: "https://example.com/?unused_parameter=whatever", scratchpad: Scratchpad { flags: {}, vars: {} }, common_args: None }
//! ```
//!
//! There's 3 calls to [`Action::apply`] here, marked 0, 1, and 2. Let's call them entries.
//! Entry 0 is the [`Action::Repeat`], entry 1 is the [`Action::SetQueryParam`]action, and entry 2 is the [`Action::RemoveQueryParam`].
//!
//! If we were to intentionally give it a URL with an `unused_parameter` query parameter, we see the following:
//!
//! ```Text
//!   0-   -   -Action::apply
//!             - self: Repeat { actions: [SetQueryParam { query_param: QueryParamSelector { name: "unused_parameter", index: 0 }, value: String("whatever") }, RemoveQueryParam(String("unused_parameter"))], limit: 10 }
//!             - task_state.debug_helper(): TaskStateDebugHelper { url: "https://example.com/?unused_parameter=a", scratchpad: Scratchpad { flags: {}, vars: {} }, common_args: None }
//!   1-   -   -|---Action::apply
//!             |   - self: SetQueryParam { query_param: QueryParamSelector { name: "unused_parameter", index: 0 }, value: String("whatever") }
//!             |   - task_state.debug_helper(): TaskStateDebugHelper { url: "https://example.com/?unused_parameter=a", scratchpad: Scratchpad { flags: {}, vars: {} }, common_args: None }
//!   2-   -   -|---Action::apply
//!             |   - self: RemoveQueryParam(String("unused_parameter"))
//!             |   - task_state.debug_helper(): TaskStateDebugHelper { url: "https://example.com/?unused_parameter=whatever", scratchpad: Scratchpad { flags: {}, vars: {} }, common_args: None }
//!   3-   -   -|---Action::apply
//!             |   - self: SetQueryParam { query_param: QueryParamSelector { name: "unused_parameter", index: 0 }, value: String("whatever") }
//!             |   - task_state.debug_helper(): TaskStateDebugHelper { url: "https://example.com/", scratchpad: Scratchpad { flags: {}, vars: {} }, common_args: None }
//!   4-  1-  2-|---Action::apply
//!             |   - self: RemoveQueryParam(String("unused_parameter"))
//!             |   - task_state.debug_helper(): TaskStateDebugHelper { url: "https://example.com/?unused_parameter=whatever", scratchpad: Scratchpad { flags: {}, vars: {} }, common_args: None }
//! ```
//!
//! Entries 0, 1, and 2 are the same as before.
//!
//! Entry 3 is the same action as entry 1, but with a different [task state](task_state) (the URL being cleaned and some other details).
//!
//! Entry 4 is the same as entry 2 and is being applied to an identical task state. The `1` column means "this action and state combo has been repeated once" and the `2` column is the last entry that combo was found in.
//!
//! In general, you should try to write cleaners such that "repeat count" and "repeat of" columns are always empty. If they are, then you aren't ever entering the same state twice for (usually) no reason.
//!
//! The exact format isn't stable, but I'm reasonably happy with how it is now.

pub(crate) use super::*;
