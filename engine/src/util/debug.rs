//! Debugging stuff.

use std::sync::{Mutex, LazyLock};
use std::collections::HashMap;

/// The state of the debug printing stuff.
#[derive(Debug, Default)]
pub(crate) struct DebugState {
    /// The indentation to use.
    ///
    /// Basically the call stack depth.
    pub(crate) indent: usize,
    /// The number of the next debug line.
    pub(crate) line: usize,
    /// The details for each item.
    ///
    /// Key is address, type, and [`Debug`] value.
    pub(crate) item_details: HashMap<(usize, &'static str, String), ItemDetails>
}

#[derive(Debug, Default)]
pub(crate) struct ItemDetails {
    /// Details for each call.
    pub(crate) call_details: HashMap<(&'static str, Vec<String>), CallDetails>
}

#[derive(Debug, Default)]
pub(crate) struct CallDetails {
    /// Amount of times this item had this call.
    pub(crate) count: usize,
    /// Last time this item had this call.
    pub(crate) last_line: Option<usize>
}

pub(crate) static DEBUG_STATE: LazyLock<Mutex<DebugState>> = LazyLock::new(Default::default);

/// When dropped, decrements [`DEBUG_STATE`]'s [`DebugState::indent`].
pub(crate) struct Deindenter;

impl std::ops::Drop for Deindenter {
    #[allow(clippy::arithmetic_side_effects, reason = "INDENT gets decremented exactly once per increment and always after.")]
    fn drop(&mut self) {
        crate::util::DEBUG_STATE.lock().expect("").indent -= 1;
    }
}

/// When the debug feature is enabled, print debug info.
macro_rules! debug {
    ($func:pat, $self:expr $(, $arg:expr)*) => {
        #[allow(clippy::arithmetic_side_effects, reason = "God help you if your config gets [`usize::MAX`] layers deep.")]
        let _deindenter = {
            let mut dsl = crate::util::DEBUG_STATE.lock().unwrap();
            let indent = dsl.indent;
            let line = dsl.line;

            let call_details = dsl.item_details.entry(($self as *const _ as usize, std::any::type_name_of_val($self), format!("{:?}", $self))).or_default()
                .call_details.entry((stringify!($func), vec![$(format!("{:?}", $arg)),*])).or_default();

            eprintln!(
                "{:>3}-{:>3}-{:>3}-{}{}",
                line,
                match call_details.count {
                    0 => "".to_string(),
                    x => x.to_string()
                },
                match call_details.last_line {
                    Some(x) => x.to_string(),
                    None => "".to_string()
                },
                "|---".repeat(indent),
                stringify!($func)
            );
            eprintln!("            {}- self: {:?}", "|   ".repeat(indent), $self);
            $(eprintln!("            {}- {}: {:?}", "|   ".repeat(indent), stringify!($arg), $arg);)*

            call_details.count += 1;
            call_details.last_line = Some(line);

            dsl.line += 1;
            dsl.indent += 1;

            crate::util::Deindenter
        };
    }
}

pub(crate) use debug;
