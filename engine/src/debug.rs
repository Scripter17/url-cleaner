//! Debugging tools.

#[cfg(feature = "debug")]
use std::sync::Mutex;

/// The call stack depth.
#[cfg(feature = "debug")]
static DEPTH: Mutex<usize> = Mutex::new(0);

/// Increments and decrements [`DEPTH`].
#[cfg(feature = "debug")]
pub(crate) struct Deindenter(());

#[cfg(feature = "debug")]
impl std::ops::Drop for Deindenter {
    /// Decrement [`DEPTH`].
    fn drop(&mut self) {
        *DEPTH.lock().expect("No panics.") -= 1;
    }
}

#[cfg(feature = "debug")]
impl Deindenter {
    /// Increment [`DEPTH`] and return both a [`Self`] and the new depth.
    pub(crate) fn indent() -> (Self, usize) {
        let mut lock = DEPTH.lock().expect("No panics.");
        *lock += 1;
        (Self(()), *lock)
    }
}

/// The "real" debug macro.
#[cfg(feature = "debug")]
macro_rules! debug {
    ($name:path$(, $arg:expr)*; $x:expr) => {
        {
            let (_deindenter, indent) = $crate::debug::Deindenter::indent();
            let prefix = "\u{2502}   ".repeat(indent - 1);
            eprintln!("{prefix}");
            eprintln!("{prefix}\u{250c}\u{2574}{}", stringify!($name));
            $(eprintln!("{prefix}\u{2502} \u{2514}\u{2574}{} = {:?}", stringify!($arg), $arg);)*
            let ret = (move || $x)();
            eprintln!("{prefix}\u{2514}\u{2574}{ret:?}");
            eprintln!("{prefix}");
            ret
        }
    };
}

/// The "fake" debug macro.
#[cfg(not(feature = "debug"))]
macro_rules! debug {
    ($name:path$(, $arg:expr)*; $x:expr) => {$x}
}

pub(crate) use debug;
