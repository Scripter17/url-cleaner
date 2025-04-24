//! Debugging stuff.

use std::sync::{Mutex, LazyLock};
use std::collections::HashMap;

/// The state of the debug printing stuffl
#[derive(Debug, Default)]
pub(crate) struct DebugState {
    /// The indentation to use.
    ///
    /// Basically the call stack depth.
    pub(crate) indent: usize,
    /// The time the last printing finished.
    pub(crate) time: Option<std::time::Instant>,
    /// Map of item addresses to the last line they showed up on.
    pub(crate) ills: HashMap<(usize, &'static str), usize>,
    /// Map of item addresses to amount of times they were printed.
    pub(crate) ics: HashMap<(usize, &'static str), usize>,
    /// Current line.
    pub(crate) line: usize
}

pub(crate) static DEBUG_STATE: LazyLock<Mutex<DebugState>> = LazyLock::new(|| Mutex::new(DebugState::default()));

/// When dropped, decrements [`INDENT`].
pub(crate) struct Deindenter;

impl std::ops::Drop for Deindenter {
    #[allow(clippy::arithmetic_side_effects, reason = "INDENT gets decremented exactly once per increment and always after.")]
    fn drop(&mut self) {
        crate::util::DEBUG_STATE.lock().expect("").indent -= 1;
    }
}

/// When the debug feature is enabled, print debug info.
macro_rules! debug {
    ($self:expr, $func:pat, $($comment:literal,)? $($name:ident),*) => {
        #[allow(clippy::arithmetic_side_effects, reason = "God help you if your config gets [`usize::MAX`] layers deep.")]
        let _deindenter = {
            let mut dsl = crate::util::DEBUG_STATE.lock().unwrap();

            dsl.line += 1;

            match dsl.time {
                Some(x) => eprint!("{:>4} {:>8.2?}", dsl.line, x.elapsed()),
                None    => eprint!("{:>4}         ", dsl.line)
            }

            let iid = ($self as *const _ as usize, stringify!($func));

            let ic = dsl.ics.entry(iid).or_default();
            let icc = match *ic {
                0 => "".to_string(),
                x => x.to_string()
            };
            *ic += 1;

            let line = dsl.line;
            let ill = match dsl.ills.entry(iid) {
                std::collections::hash_map::Entry::Occupied(mut e) => {e.insert(line).to_string()    },
                std::collections::hash_map::Entry::Vacant  (    e) => {e.insert(line); "".to_string()}
            };

            eprint!(
                " {icc:>4} {ill:>4} {}{}",
                "|   ".repeat(dsl.indent),
                stringify!($func)
            );
            $(eprint!($comment);)?
            $(eprint!(concat!("; ", stringify!($name), ": {:?}"), $name);)*
            eprintln!();

            dsl.indent += 1;
            dsl.time = Some(std::time::Instant::now());

            crate::util::Deindenter
        };
    }
}

pub(crate) use debug;
