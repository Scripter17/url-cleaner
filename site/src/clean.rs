//! The `/clean` endpoint.

use std::fmt::Write;

use crate::*;

/// The `/clean` endpoint.
#[post("/clean", data="<tasks>")]
pub async fn clean(state: &State<&'static ServerState>, config: JobConfig, tasks: &[u8]) -> Result<String, CleanError> {
    let state = *state.inner();

    match (&state.config.passwords, &config.password) {
        (None           , None          ) => {},
        (None           , Some(_)       ) => Err(CleanError {status: 400, message: "This instance requires no password.".into()})?,
        (Some(_)        , None          ) => Err(CleanError {status: 401, message: "Password required.".into()})?,
        (Some(passwords), Some(password)) => if !passwords.contains(password) {Err(CleanError {status: 401, message: "Invalid password".into()})?}
    }

    let Some(mut cleaner) = state.config.profiled_cleaner.get(config.profile.as_deref()) else {
        Err(CleanError {status: 400, message: format!("Unknown profile: {:?}", config.profile)})?
    };

    let (in_senders , in_recievers ) = (0..state.config.threads.get()).map(|_| std::sync::mpsc::channel::<&[u8]>()).collect::<(Vec<_>, Vec<_>)>();
    let (out_senders, out_recievers) = (0..state.config.threads.get()).map(|_| std::sync::mpsc::channel::<Box<str>>()).collect::<(Vec<_>, Vec<_>)>();

    if let Some(params_diff) = config.params_diff {
        params_diff.apply(&mut cleaner.params);
    }

    let job = &Job {
        context: config.context,
        cleaner,
        unthreader: state.unthreader.filter(config.unthread),
        #[cfg(feature = "cache")]
        cache: Cache {
            inner: &state.inner_cache,
            config: CacheConfig {
                read : config.read_cache,
                write: config.write_cache,
                delay: config.cache_delay
            }
        },
        #[cfg(feature = "http")]
        http_client: &state.http_client
    };

    std::thread::scope(|s| {
        std::thread::Builder::new().name("Task collector".into()).spawn_scoped(s, move || {
            for (in_sender, lazy_task_config) in {in_senders}.iter().cycle().zip(crate::util::ByteLines(tasks).filter(|line| !line.is_empty())) {
                in_sender.send(lazy_task_config).expect("The in reciever to still exist.");
            }
        }).expect("Spawning a thread to work fine.");

        for (i, (ir, os)) in in_recievers.into_iter().zip(out_senders).enumerate() {
            std::thread::Builder::new().name(format!("Worker {i}")).spawn_scoped(s, move || {
                while let Ok(lazy_task_config) = ir.recv() {
                    let ret = match job.r#do(lazy_task_config) {
                        Ok(x) => String::from(x),
                        Err(e) => format!("-{e:?}")
                    };

                    os.send(ret.into_boxed_str()).expect("The out receiver to still exist.");
                }
            }).expect("Spawning a thread to work fine.");
        }

        let mut ret = String::with_capacity(64 * tasks.len().checked_ilog2().unwrap_or(0).pow(2) as usize);
        for or in out_recievers.iter().cycle() {
            match or.recv() {
                Ok(x) => writeln!(ret, "{x}").expect("???"),
                Err(_) => break
            }
        }
        Ok(ret)
    })
}
