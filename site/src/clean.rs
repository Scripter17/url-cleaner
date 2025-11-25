//! The `/clean` endpoint.

use crate::*;

/// The `/clean` endpoint.
#[post("/clean", data="<tasks>")]
pub async fn clean(state: &State<&'static ServerState>, auth: Auth, config: CleanConfig, tasks: Vec<u8>) -> Result<String, CleanError> {
    let state = *state.inner();

    if !state.config.accounts.check(&auth) {
        Err(Status::Unauthorized)?
    }

    let Some(mut cleaner) = state.config.profiled_cleaner.get(config.profile.as_deref()) else {
        Err(CleanError {status: 400, message: format!("Unknown profile: {:?}", config.profile)})?
    };

    let (in_senders , in_recievers ) = (0..state.config.threads.get()).map(|_| std::sync::mpsc::channel::<&[u8]>()).collect::<(Vec<_>, Vec<_>)>();
    let (out_senders, out_recievers) = (0..state.config.threads.get()).map(|_| std::sync::mpsc::channel::<Box<str>>()).collect::<(Vec<_>, Vec<_>)>();

    if let Some(params_diff) = config.params_diff {
        params_diff.apply(&mut cleaner.params);
    }

    let job_config = JobConfig {
        context: &config.context,
        cleaner: &cleaner,
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
        std::thread::Builder::new().name("Task collector".into()).spawn_scoped(s, || {
            for (in_sender, lazy_task_config) in {in_senders}.iter().cycle().zip(crate::util::ByteLines::new(&tasks)) {
                in_sender.send(lazy_task_config).expect("The in reciever to still exist.");
            }
        }).expect("Spawning a thread to work fine.");

        for (i, (ir, os)) in in_recievers.into_iter().zip(out_senders).enumerate() {
            std::thread::Builder::new().name(format!("Worker {i}")).spawn_scoped(s, || {
                let (ir, os) = (ir, os); // Moves just the channel stuff without moving job_config.
                while let Ok(lazy_task_config) = ir.recv() {
                    let ret = match job_config.do_lazy_task_config(lazy_task_config) {
                        Ok(x) => {let mut x = String::from(x); x.push('\n'); x},
                        Err(e) => format!("-{e:?}\n")
                    };

                    os.send(ret.into_boxed_str()).expect("The out receiver to still exist.");
                }
            }).expect("Spawning a thread to work fine.");
        }

        let mut ret = String::new();
        for or in out_recievers.iter().cycle() {
            match or.recv() {
                Ok(x) => ret.push_str(&x),
                Err(_) => break
            }
        }
        Ok(ret)
    })
}
