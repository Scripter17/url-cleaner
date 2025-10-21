//! Glue for [`std::process::Command`].

#[expect(unused_imports, reason = "Used in doc comments.")]
use std::process::{Command, Stdio, ExitStatus, ChildStdin, Child};
use std::io::Write;
use std::path::PathBuf;
use std::str::{from_utf8, FromStr};
use std::collections::HashMap;
use std::convert::Infallible;
use std::ffi::OsString;

use url::Url;
use thiserror::Error;
use serde::{Serialize, Deserialize};

use crate::prelude::*;

/// Config for making [`Command`]s.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Suitability)]
#[suitable(never)]
#[serde(deny_unknown_fields)]
#[serde(remote="Self")]
pub struct CommandConfig {
    /// The program.
    pub program: String,
    /// The arguments to pass to the program.
    ///
    /// Defaults to an empty [`Vec`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub args: Vec<StringSource>,
    /// The directory to run the program in.
    ///
    /// Defaults to [`None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub current_dir: Option<PathBuf>,
    /// The environment variables to run the program with.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub envs: HashMap<String, StringSource>,
    /// The STDIN to give the program.
    ///
    /// Defaults to [`None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub stdin: Option<StringSource>
}

impl FromStr for CommandConfig {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl From<&str> for CommandConfig {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<String> for CommandConfig {
    fn from(value: String) -> Self {
        Self {
            program    : value,
            args       : Default::default(),
            current_dir: Default::default(),
            envs       : Default::default(),
            stdin      : Default::default()
        }
    }
}

crate::util::string_or_struct_magic!(CommandConfig);

/// The enum of errors the various [`CommandConfig`] methods can return.
#[derive(Debug, Error)]
pub enum MakeCommandError {
    /// Returned when an [`std::io::Error`] is encountered.
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    /// Returned when an [`std::str::Utf8Error`] is encountered.
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
    /// Returned when a [`url::ParseError`] is returned.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when the command is terminated by a signal.
    ///
    /// See [`std::process::ExitStatus::code`] for details.
    #[error("The command was terminated by a signal. See std::process::ExitStatus::code for details.")]
    SignalTermination,
    /// Returned when a call to [`StringSource::get`] returns [`None`] where it has to return [`Some`].
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError)
}

impl CommandConfig {
    /// Builds the [`Command`].
    /// # Errors
    /// If a call to [`StringSource::get`] returns an error, that error is returned.
    pub fn make(&self, task_state: &TaskStateView) -> Result<Command, MakeCommandError> {
        let mut ret = Command::new(&self.program);
        for arg in self.args.iter() {
            ret.arg(OsString::from(get_string!(arg, task_state, MakeCommandError)));
        }
        if let Some(current_dir) = &self.current_dir {
            ret.current_dir(current_dir);
        }
        for (k, v) in self.envs.iter() {
            if let Some(v) = v.get(task_state)? {
                ret.env(k, &*v);
            }
        }
        Ok(ret)
    }

    /// Executes the command and gets its exit code.
    /// # Errors
    /// If the call to [`Self::make`] returns an error, that error is returned.
    ///
    /// If the call to [`Command::status`] returns an error, that error is returned.
    ///
    /// If the call to [`ExitStatus::code`] returns [`None`], returns the error [`MakeCommandError::SignalTermination`].
    pub fn exit_code(&self, task_state: &TaskStateView) -> Result<i32, MakeCommandError> {
        self.make(task_state)?.status()?.code().ok_or(MakeCommandError::SignalTermination)
    }

    /// Executes the command and returns its STDOUT.
    /// # Errors
    /// If the call to [`Self::make`] returns an error, that error is returned.
    ///
    /// If the call to [`Command::spawn`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`MakeCommandError::StringSourceIsNone`].
    ///
    /// If the call to [`ChildStdin::write_all`] returns an error, that error is returned.
    ///
    /// If the call to [`Child::wait_with_output`] returns an error, that error is returned.
    ///
    /// If the call to [`std::str::from_utf8`] returns an error, that error is returned.
    #[allow(clippy::missing_panics_doc, reason = "Shouldn't ever panic.")]
    pub fn output(&self, task_state: &TaskStateView) -> Result<String, MakeCommandError> {
        // https://stackoverflow.com/a/49597789/10720231
        let mut command = self.make(task_state)?;
        command.stdout(Stdio::piped());
        command.stderr(Stdio::null());
        let child = if let Some(stdin) = &self.stdin {
            command.stdin(Stdio::piped());
            let mut child=command.spawn()?;
            let child_stdin=child.stdin.as_mut().expect("The STDIN just set to be available."); // This never panics.
            child_stdin.write_all(get_str!(stdin, task_state, MakeCommandError).as_bytes())?;
            child
        } else {
            command.spawn()?
        };
        Ok(from_utf8(&child.wait_with_output()?.stdout)?.to_string())
    }

    /// Executes the command and gets a [`Url`] from the first and only line of its STDOUT, trimming any trailing `\r` and `\n`.
    /// # Errors
    /// If the call to [`Self::output`] returns an error, that error is returned.
    ///
    /// If the call to [`Url::parse`] returns an error, that error is returned.
    pub fn get_url(&self, task_state: &TaskStateView) -> Result<Url, MakeCommandError> {
        Ok(Url::parse(self.output(task_state)?.trim_end_matches(['\r', '\n']))?)
    }
}
