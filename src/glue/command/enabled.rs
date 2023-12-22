use std::ffi::OsString;
use std::process::{Command, Output as CommandOutput, Stdio};
use serde::{Deserialize, Deserializer};
use std::io::{Write, Error as IoError};
use std::path::PathBuf;
use url::{Url, ParseError};
use std::str::{from_utf8, FromStr, Utf8Error};
use thiserror::Error;
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
pub struct CommandWrapper {
    #[serde(flatten, deserialize_with = "deserialize_command")]
    pub inner: Command,
    #[serde(default)]
    pub output_handling: OutputHandling
}

/// The rules for what exactly to return from a command
#[derive(Debug, Deserialize, Default, Clone)]
pub enum OutputHandling {
    #[default]
    ReturnStdout,
    ReturnStderr,
    Error,
    PipeStdoutTo(Box<CommandWrapper>),
    PipeStderrTo(Box<CommandWrapper>),
    ApplyStdoutUrlTo(Box<CommandWrapper>),
    ApplyStderrUrlTo(Box<CommandWrapper>),
    IfExitCode {
        #[serde(default = "get_0")]
        equals: i32,
        then: Box<OutputHandling>,
        #[serde(default = "error_output_handler")]
        r#else: Box<OutputHandling>
    }
}

fn get_0() -> i32 {0}
fn error_output_handler() -> Box<OutputHandling> {Box::new(OutputHandling::Error)}

#[derive(Debug, Deserialize)]
struct CommandParts {
    #[serde(deserialize_with = "deserialize_os_string")]
    program: OsString,
    #[serde(deserialize_with = "deserialize_os_string_vec", default)]
    args: Vec<OsString>,
    #[serde(default)]
    current_dir: Option<PathBuf>
}

fn deserialize_os_string<'de, D>(deserializer: D) -> Result<OsString, D::Error>
where
    D: Deserializer<'de>
{
    let temp: String = Deserialize::deserialize(deserializer)?;
    Ok(temp.into())
}

fn deserialize_os_string_vec<'de, D>(deserializer: D) -> Result<Vec<OsString>, D::Error>
where
    D: Deserializer<'de>
{
    let temp: Vec<String> = Deserialize::deserialize(deserializer)?;
    Ok(temp.into_iter().map(|x| x.into()).collect::<Vec<_>>())
}

fn deserialize_command<'de, D>(deserializer: D) -> Result<Command, D::Error>
where
    D: Deserializer<'de>
{
    let command_parts: CommandParts = Deserialize::deserialize(deserializer)?;
    let mut ret=Command::new(command_parts.program);
    ret.args(command_parts.args);
    match command_parts.current_dir {
        Some(dir) => {ret.current_dir(dir);},
        None => {}
    }
    Ok(ret)
}

/// A wrapper around all the errors a command condition/mapper can return.
#[derive(Debug, Error)]
pub enum CommandError {
    /// I/O error.
    #[error("I/O error.")]
    IoError(#[from] IoError),
    /// UTF-8 error.
    #[error("UTF-8 error.")]
    Utf8Error(#[from] Utf8Error),
    /// URL parsing error.
    #[error("URL parsing error.")]
    ParseError(#[from] ParseError),
    /// The command was terminated by a signal. See [`std::process::ExitStatus::code`] for details.
    #[error("The command was terminated by a signal. See std::process::ExitStatus::code for details.")]
    SignalTermination,
    /// The output handler wsa [`OutputHandling::Error`].
    #[error("The output handler was OutputHandling::Error.")]
    ExplicitError
}

impl OutputHandling {
    fn handle(&self, url: &Url, output: CommandOutput) -> Result<String, CommandError> {
        match self {
            Self::ReturnStdout                     => Ok(from_utf8(&output.stdout)?.to_string()),
            Self::ReturnStderr                     => Ok(from_utf8(&output.stderr)?.to_string()),
            Self::Error                            => Err(CommandError::ExplicitError),
            Self::PipeStdoutTo(command)            => command.output(url, Some(&output.stdout)),
            Self::PipeStderrTo(command)            => command.output(url, Some(&output.stderr)),
            Self::ApplyStdoutUrlTo(command)        => command.output(&Url::parse(from_utf8(&output.stdout)?)?, None),
            Self::ApplyStderrUrlTo(command)        => command.output(&Url::parse(from_utf8(&output.stderr)?)?, None),
            Self::IfExitCode{equals, then, r#else} => if output.status.code().ok_or(CommandError::SignalTermination)?==*equals {then.handle(url, output)} else {r#else.handle(url, output)}
        }
    }
}

impl CommandWrapper {
    /// Runs the command and gets the exit code. Returns [`Err(CommandError::SignalTerminatio)`] if the command returns no exit code.
    pub fn exit_code(&self, url: &Url) -> Result<i32, CommandError> {
        self.clone().apply_url(url).inner.status()?.code().ok_or(CommandError::SignalTermination)
    }

    fn output(&self, url: &Url, stdin: Option<&[u8]>) -> Result<String, CommandError> {
        match stdin {
            Some(stdin) => {
                // https://stackoverflow.com/a/49597789/10720231
                let mut cloned=self.clone().apply_url(url);
                let mut child=cloned.inner
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()?;
                let child_stdin=child.stdin.as_mut().unwrap();
                child_stdin.write_all(stdin)?;
                self.output_handling.handle(url, child.wait_with_output()?)
            },
            None => self.output_handling.handle(url, self.clone().apply_url(url).inner.output()?) 
        }
    }

    /// Run the command and get the resulting URL from the STDOUT.
    /// First calls [`str::trim_end_matches`] on the STDOUT to get rid of all trailing carriage returns and newlines, then passes what's left to [`Url::parse`].
    pub fn get_url(&self, url: &Url) -> Result<Url, CommandError> {
        Ok(Url::parse(&self.clone().apply_url(url).output(url, None)?.trim_end_matches(&['\r', '\n']))?)
    }

    /// A very messy function that puts the URL in the command arguments.
    fn apply_url(self, url: &Url) -> Self {
        // Why doesn't std::process::Command have a clear_args method?
        let mut ret=Command::new(self.inner.get_program());
        match self.inner.get_current_dir() {
            Some(dir) => {ret.current_dir(dir);},
            None => {}
        }
        ret.args(self.inner.get_args().map(|arg| if arg.to_str()==Some("{}") {Cow::Owned(OsString::from_str(url.as_str()).unwrap())} else {Cow::Borrowed(arg)}));
        Self {inner: ret, output_handling: self.output_handling.clone()}
    }
}

impl Clone for CommandWrapper {
    fn clone(&self) -> Self {
        let mut ret=Command::new(self.inner.get_program());
        ret.args(self.inner.get_args());
        match self.inner.get_current_dir() {
            Some(dir) => {ret.current_dir(dir);},
            None => {}
        }
        Self {inner: ret, output_handling: self.output_handling.clone()}
    }
}
