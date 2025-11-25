use crate::*;

const TASK_URLS: [&str; 3] = [
    "https://x.com?a=2",
    "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id",
    "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8"
];

group!(tasks, make, r#do);

fn make(c: &mut Criterion) {
    let job_config = JobConfig {
        context            : &Default::default(),
        cleaner            : &Default::default(),
        unthreader         : &Default::default(),
        #[cfg(feature = "cache")]
        cache: Cache {
            inner: &Default::default(),
            config: Default::default()
        },
        #[cfg(feature = "http")]
        http_client: &Default::default()
    };

    for url in TASK_URLS {
        c.bench_function(
            &format!("Make 10k Tasks, &str: {url}"),
            |b| b.iter_batched(
                || Job {
                    config: job_config,
                    tasks: std::iter::repeat_n(bb(url), bb(10_000)).map(|url| bb(Ok(bb(url).into())))
                },
                |job| bb(job).into_iter().for_each(|x| {let _ = bb(x.expect("Ok").make());}),
                criterion::BatchSize::SmallInput
            )
        );
    }

    for url in TASK_URLS {
        c.bench_function(
            &format!("Make 10k Tasks, &[u8]: {url}"),
            |b| b.iter_batched(
                || Job {
                    config: job_config,
                    tasks: std::iter::repeat_n(bb(url), bb(10_000)).map(|url| bb(Ok(bb(url).as_bytes().into())))
                },
                |job| bb(job).into_iter().for_each(|x| {let _ = bb(x.expect("Ok").make());}),
                criterion::BatchSize::SmallInput
            )
        );
    }
}

fn r#do(c: &mut Criterion) {
    let cleaner = &Cleaner::load_from_file("src/cleaner/bundled_cleaner.json").unwrap();

    for url in TASK_URLS {
        let task = Task {
            config     : bb(bb(bb(url).parse()).unwrap()),
            job_context: &Default::default(),
            cleaner,
            unthreader : &Default::default(),
            #[cfg(feature = "cache")]
            cache: Cache {
                inner: &Default::default(),
                config: Default::default()
            },
            #[cfg(feature = "http")]
            http_client: &Default::default()
        };

        c.bench_function(
            &format!("Do 10k Tasks: {url}"),
            |b| b.iter_batched(
                || std::iter::repeat_n(bb(task.clone()), 10_000),
                |tasks| bb(tasks).for_each(|x| {let _ = bb(bb(x).r#do());}),
                criterion::BatchSize::SmallInput
            )
        );
    }
}
