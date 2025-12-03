//! [`doc_test`](crate::doc_test).

#[expect(unused_imports, reason = "Used in doc comments.")]
use crate::prelude::*;

/// A macro to generate doctests.
///
/// Because [`TaskState`] is invariant, code like `assert!(Condition::Whatever(..).check(&task_state).unwrap())` errors on the [`Condition`] not living long enough.
///
/// This allows a relatively okay way to write doctests by completely disregarding performance and only having the minimum needed expressiveness.
///
/// While the API is unstable and meant for internal use only, the current syntax is as follows:
///
/// - `task_state`, the name to give the TaskState, then `field = value` pairs for the [`TaskConfig`], [`CallArgs`], [`JobContext`], [`Params`], [`Functions`], and a [`bool`] for [`Unthreader::if`].
///
/// - `check`, `get`, or `apply` followed by the pattern of the expected result, with containing `Ok`/`Err`/`Some` omitted, the component to call the method on, then the arguments to the method.
#[macro_export]
macro_rules! doc_test {
    (task_state, $task_state:ident $(, task = $task:expr)? $(, call_args: $call_args:expr)? $(, job_context = $job_context:expr)? $(, params = $params:expr)? $(, functions = $functions:expr)? $(, unthread = $unthread:expr)?) => {
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let task                                     = "https://example.com"; $(let task        = $task       ;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let call_args  : $crate::prelude::CallArgs   = Default::default()   ; $(let call_args   = $call_args  ;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let job_context: $crate::prelude::JobContext = Default::default()   ; $(let context     = $job_context;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let params     : $crate::prelude::Params     = Default::default()   ; $(let params      = $params     ;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let functions  : $crate::prelude::Functions  = Default::default()   ; $(let functions   = $functions  ;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let unthread   : bool                        = Default::default()   ; $(let unthread    = $unthread   ;)?

        let Task {url, context} = task.make_task().expect("The task to be valid.");

        let mut $task_state = {
            $crate::prelude::TaskState {
                url,
                context: &context,
                call_args: std::cell::Cell::new(Some(&call_args)),
                job: &Job {
                    context: job_context,
                    cleaner: $crate::prelude::Cleaner {
                        docs: Default::default(),
                        params,
                        functions: std::borrow::Cow::Owned(functions),
                        actions: Default::default()
                    },
                    unthreader: &$crate::prelude::Unthreader::r#if(unthread),
                    #[cfg(feature = "cache")]
                    cache: $crate::prelude::Cache {
                        inner: &Default::default(),
                        config: Default::default()
                    },
                    #[cfg(feature = "http")]
                    http_client: &Default::default()
                }
            }
        };
    };



    (check, Ok, $checker:expr, $($arg:expr),*) => {
        let temp = $checker.clone();
        temp.check($($arg),*).unwrap();
    };
    (check, true, $checker:expr, $($arg:expr),*) => {
        let temp = $checker.clone();
        assert!(temp.check($($arg),*).unwrap());
    };
    (check, false, $checker:expr, $($arg:expr),*) => {
        let temp = $checker.clone();
        assert!(!temp.check($($arg),*).unwrap());
    };
    (check, Err, $checker:expr, $($arg:expr),*) => {
        let temp = $checker.clone();
        temp.check($($arg),*).unwrap_err();
    };



    (get, Some, $source:expr, $($arg:expr),*) => {
        let temp = $source.clone();
        temp.get($($arg),*).unwrap().unwrap();
    };
    (get, Some($eq:expr), $source:expr, $($arg:expr),*) => {
        let temp = $source.clone();
        assert_eq!(temp.get($($arg),*).unwrap(), Some($eq.into()));
    };
    (get, None, $source:expr, $($arg:expr),*) => {
        let temp = $source.clone();
        assert_eq!(temp.get($($arg),*).unwrap(), None);
    };
    (get, Err, $source:expr, $($arg:expr),*) => {
        let temp = $source.clone();
        temp.get($($arg),*).unwrap_err();
    };



    (apply, Ok, $applicator:expr, $($arg:expr),*) => {
        let temp = $applicator.clone();
        temp.apply($($arg),*).unwrap();
    };
    (apply, Err, $applicator:expr, $($arg:expr),*) => {
        let temp = $applicator.clone();
        temp.apply($($arg),*).unwrap_err();
    };
}

