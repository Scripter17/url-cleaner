//! # [`StringModification`]
//!
//! Often, you want to modify a string before using it.
//!
//! For example you may want to get a URL's query then
//!
//! 1. Split it on `/` and keep only the third segment.
//!
//! 2. Remove the first character.
//!
//! 3. Base64 decode it using the default URL safe alphabet.
//!
//! 4. Split it on `,` and keep only the last segment.
//!
//! 5. Replace the URL being cleaned with that segment.
//!
//! While this seems like and is a bizarre example, this exact process is used to clean the links cnn sends you in emails.
//!
//! To express the above operations, we would write a [`StringSource::Modified`] as follows:
//!
//! ```Json
//! {"SetWhole": {"Modified": {
//!   "value": {"Part": "Query"},
//!   "modification": {"All": [
//!     {"KeepNthSegment": {"split": "/", "index": 2}},
//!     {"RemoveChar": 0},
//!     "Base64Decode",
//!     {"KeepNthSegment": {"split": ",", "index": -1}}
//!   ]}
//! }}}
//! ```

pub(crate) use super::*;
