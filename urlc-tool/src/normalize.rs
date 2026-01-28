//! Normalize.

use std::collections::{HashSet, HashMap};

use url::Url;
use serde::{Serialize, Deserialize};

use super::prelude::*;

/// Parse each line of STDIN as a URL and print it.
///
/// Output lines starting with - represent errors.
#[derive(Debug, Parser)]
pub struct Args {}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let mut stdin = std::io::stdin().lock();
        let mut buf = Vec::new();

        while stdin.read_until(b'\n', &mut buf).unwrap() > 0 {
            if buf.ends_with(b"\n") {
                buf.pop();
                if buf.ends_with(b"\r") {
                    buf.pop();
                }
            }

            if buf.is_empty() {
                continue;
            }

            match normalize(&buf) {
                Ok (s) => println!("{s}"),
                Err(e) => println!("-{e:?}")
            }

            buf.clear();
        }
    }
}

/// Normalize a task.
fn normalize(b: &[u8]) -> Result<String, MakeTaskError> {
    Ok(match b[0] {
        b'"' => normalize(serde_json::from_slice::<String>(b)?.as_bytes())?,
        b'{' => {
            let task = serde_json::from_slice::<Task>(b)?;
            if task.context == TaskContext::default() {
                task.url.into()
            } else {
                serde_json::to_string(&task).unwrap()
            }
        },
        b'a'..=b'z' | b'A'..=b'Z' => Url::parse(str::from_utf8(b)?)?.into(),
        _ => Err(MakeTaskError::OtherwiseInvalid)?
    })
}

/// A [`Task`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Task {
    /// The [`Url`].
    pub url: Url,
    /// The [`TaskContext`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub context: TaskContext
}

/// The context of a [`Task`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskContext {
    /// The flags to use.
    ///
    /// Defaults to an empty [`HashSet`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: HashSet<String>,
    /// The vars to use.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>
}

/// The enum of errors [`normalize`] can return.
#[derive(Debug, Error)]
pub enum MakeTaskError {
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when a [`std::str::Utf8Error`] is encountered.
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
    /// Returned when a [`serde_json::Error`] is encountered.
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    /// Returned when a line is otherwise invalid.
    #[error("A line was otherwise invalid.")]
    OtherwiseInvalid
}

/// Return [`true`] if `x` is [`T`]'s [`Default`].
fn is_default<T: PartialEq + Default>(x: &T) -> bool {x == &T::default()}
