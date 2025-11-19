//! The `/clean` endpoint.

use crate::*;

/// The `/clean` endpoint.
#[post("/clean", data="<clean_payload>")]
pub async fn clean(auth: Auth, state: &State<ServerState>, clean_payload: &str) -> SmallCleanResult {
    if !state.config.accounts.check(&auth) {
        Err(Status::Unauthorized)?
    }

    let clean_payload = match serde_json::from_str::<CleanPayload>(clean_payload) {
        Ok(clean_payload) => clean_payload,
        Err(e) => Err(CleanError {status: 400, message: e.to_string()})?
    };

    let Some(mut cleaner) = state.config.profiled_cleaner.get(clean_payload.config.profile.as_deref()) else {
        Err(CleanError {status: 400, message: format!("Unknown profile: {:?}", clean_payload.config.profile)})?
    };

    if let Some(params_diff) = clean_payload.config.params_diff {
        params_diff.apply(&mut cleaner.params);
    }

    let (in_senders , in_recievers ) = (0..state.config.threads.get()).map(|_| std::sync::mpsc::channel::<SmallLazyTask<'_, '_>>()).collect::<(Vec<_>, Vec<_>)>();
    let (out_senders, out_recievers) = (0..state.config.threads.get()).map(|_| std::sync::mpsc::channel::<Result<String, DoTaskError>>()).collect::<(Vec<_>, Vec<_>)>();

    let mut ret = SmallCleanSuccess {
        urls: Vec::with_capacity(clean_payload.tasks.len())
    };

    let unthreader = match clean_payload.config.unthread {
        false => &NO_UNTHREADER,
        true  => &state.unthreader
    };

    std::thread::scope(|s| {
        std::thread::Builder::new().name("Task collector".into()).spawn_scoped(s, || {
            let job_config = JobConfig {
                context: &clean_payload.config.context,
                cleaner: &cleaner,
                unthreader,
                #[cfg(feature = "cache")]
                cache: Cache {
                    inner: &state.inner_cache,
                    config: CacheConfig {
                        read : clean_payload.config.read_cache,
                        write: clean_payload.config.write_cache,
                        delay: clean_payload.config.cache_delay
                    }
                },
                #[cfg(feature = "http")]
                http_client: &state.http_client
            };
            for (in_sender, small_lazy_task_config) in {in_senders}.iter().cycle().zip(clean_payload.tasks) {
                in_sender.send(job_config.make_small_lazy_task(small_lazy_task_config)).expect("To successfully send the LazyTask.");
            }
        }).expect("Spawning a thread to work fine.");

        in_recievers.into_iter().zip(out_senders).enumerate().map(|(i, (ir, os))| {
            std::thread::Builder::new().name(format!("Worker {i}")).spawn_scoped(s, move || {
                while let Ok(task_source) = ir.recv() {
                    let ret = match task_source.make() {
                        Ok(task) => task.r#do().map(Into::into),
                        Err(e) => Err(e.into())
                    };

                    os.send(ret).expect("The out receiver to still exist.");
                }
            }).expect("Spawning a thread to work fine.");
        }).for_each(drop);

        std::thread::Builder::new().name("Task returner".into()).spawn_scoped(s, || {
            for or in {out_recievers}.iter().cycle() {
                match or.recv() {
                    Ok(Ok (x)) => ret.urls.push(Ok(x)),
                    Ok(Err(e))     => ret.urls.push(Err(e.to_string())),
                    Err(_) => break
                }
            }
        }).expect("Spawning a thread to work fine.");
    });

    Ok(ret)
}

