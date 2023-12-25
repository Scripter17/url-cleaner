use std::ffi::OsString;
use std::process::{Command, Output as CommandOutput, Stdio};
use serde::ser::{Serializer, SerializeStruct};
use serde::Serialize;
use serde::{Deserialize, Deserializer};
use std::io::{Write, Error as IoError};
use std::path::PathBuf;
use url::{Url, ParseError};
use std::str::{from_utf8, FromStr, Utf8Error};
use thiserror::Error;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandWrapper {
    #[serde(flatten, serialize_with = "serialize_command", deserialize_with = "deserialize_command")]
    pub inner: Command,
    #[serde(default)]
    pub output_handling: OutputHandler
}

/// Before a [`CommandWrapper`] returns its output, it passes it through this to allow for piping and control flow.
/// For the sake of simplicity, [`OutputHandler::handle`] returns a [`Result<String, CommandError>`] instead of [`Result<Vec<u8>, CommandError>`].
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub enum OutputHandler {
    /// Return the STDOUT.
    #[default]
    ReturnStdout,
    /// Return the STDERR.
    ReturnStderr,
    /// Always return the error [`CommandError::ExplicitError`].
    Error,
    /// Pipes the STDOUT into the contained command's STDIN.
    PipeStdoutTo(Box<CommandWrapper>),
    /// Pipes the STDERR into the contained command's STDIN.
    PipeStderrTo(Box<CommandWrapper>),
    /// Extracts the URL from the STDOUT and applies it to the contained command's arguments.
    ApplyStdoutUrlTo(Box<CommandWrapper>),
    /// Extracts the URL from the STDERR and applies it to the contained command's arguments.
    ApplyStderrUrlTo(Box<CommandWrapper>),
    /// If the exit code equals `equals`, `then` is used as the handler. Otherwise `else` (Defaults to [`OutputHandler::Error`]) is used.
    IfExitCode {
        #[serde(default)]
        equals: i32,
        then: Box<OutputHandler>,
        #[serde(default = "error_output_handler")]
        r#else: Box<OutputHandler>
    }
}

fn error_output_handler() -> Box<OutputHandler> {Box::new(OutputHandler::Error)}

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
    /// The output handler wsa [`OutputHandler::Error`].
    #[error("The output handler was OutputHandler::Error.")]
    ExplicitError
}

impl OutputHandler {
    pub fn handle(&self, url: &Url, output: CommandOutput) -> Result<String, CommandError> {
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

#[derive(Debug, Serialize, Deserialize)]
struct CommandParts {
    program: String,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    current_dir: Option<PathBuf>,
    #[serde(default)]
    envs: HashMap<String, String>
}

fn deserialize_command<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Command, D::Error> {
    let command_parts: CommandParts = Deserialize::deserialize(deserializer)?;
    let mut ret=Command::new(command_parts.program);
    ret.args(command_parts.args);
    match command_parts.current_dir {
        Some(dir) => {ret.current_dir(dir);},
        None => {}
    }
    ret.envs(command_parts.envs);
    Ok(ret)
}

fn serialize_command<S: Serializer>(command: &Command, serializer: S) -> Result<S::Ok, S::Error> {
    let mut state = serializer.serialize_struct("Comamnd", 3)?;
    state.serialize_field("program", command.get_program().to_str().unwrap())?;
    state.serialize_field("args", &command.get_args().map(|x| x.to_str().unwrap()).collect::<Vec<_>>())?;
    state.serialize_field("envs", &command.get_envs().filter(|(_, v)| v.is_some()).map(|(k, v)| (k.to_str().unwrap(), v.unwrap().to_str().unwrap())).collect::<HashMap<_, _>>())?;
    state.end()
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
        ret.args(self.inner.get_args().map(|arg| if arg.to_str()==Some("{}") {Cow::Owned(OsString::from_str(url.as_str()).unwrap())} else {Cow::Borrowed(arg)}));
        match self.inner.get_current_dir() {
            Some(dir) => {ret.current_dir(dir);},
            None => {}
        }
        ret.envs(self.inner.get_envs().filter(|(_, v)| v.is_some()).map(|(k, v)| (k.to_owned(), v.unwrap().to_owned())));
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
        ret.envs(self.inner.get_envs().filter(|(_, v)| v.is_some()).map(|(k, v)| (k.to_owned(), v.unwrap().to_owned())));
        Self {inner: ret, output_handling: self.output_handling.clone()}
    }
}

// impl PartialEq for CommandWrapper {
//     fn eq(&self, other: &Self) -> bool {
//         self.inner.get_program()==other.inner.get_program() &&
//             self.inner.get_args().collect::<Vec<_>>()==other.inner.get_args().collect::<Vec<_>>() &&
//             self.inner.get_current_dir()==other.inner.get_current_dir() &&
//             self.inner.get_envs().collect::<Vec<_>>()==other.inner.get_envs().collect::<Vec<_>>()
//     }
// }

impl PartialEq for CommandWrapper {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

// impl Eq for CommandWrapper {}