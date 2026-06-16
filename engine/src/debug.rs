//! Debugging tools.

#[cfg(feature = "debug")]
use std::sync::atomic::AtomicUsize;

/// The call stack depth.
#[cfg(feature = "debug")]
pub(crate) static DEPTH: AtomicUsize = AtomicUsize::new(0);

/// The "fake" debug macro.
#[cfg(not(feature = "debug"))]
macro_rules! debug {
    ($name:path$(, $arg:expr)*; $x:expr) => {$x}
}

/// The "real" debug macro.
#[cfg(feature = "debug")]
macro_rules! debug {
    ($name:path$(, $arg:expr)*; $x:expr) => {
        {
            let indent = $crate::debug::DEPTH.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let prefix = "\u{2502}   ".repeat(indent);
            eprintln!("{prefix}");
            eprintln!("{prefix}\u{250c}\u{2574}{}", stringify!($name));
            $(eprintln!("{prefix}\u{2502} \u{2514}\u{2574}{} = {:?}", stringify!($arg), $arg);)*
            let ret = $x;
            eprintln!("{prefix}\u{2514}\u{2574}{ret:?}");
            eprintln!("{prefix}");
            $crate::debug::DEPTH.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
            ret
        }
    };
}

pub(crate) use debug;
