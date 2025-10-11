use std::hint::black_box;
use criterion::Criterion;

use crate::*;

use url_cleaner_engine::types::*;
use url_cleaner_engine::glue::*;

const TASK_URLS: [&'static str; 3] = [
    "https://x.com?a=2",
    "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id",
    "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8"
];

group!(tasks, make, r#do);

fn make(c: &mut Criterion) {
    let job_config = &JobConfig {
        context            : &Default::default(),
        cleaner            : &Default::default(),
        cache              : &Default::default(),
        cache_handle_config:  Default::default(),
        unthreader         : &Default::default()
    };

    for url in TASK_URLS {
        c.bench_function(
            &format!("Make 10k Tasks, &str: {url}"),
            |b| b.iter_batched(
                || Job {
                    config: job_config,
                    lazy_task_configs: Box::new(std::iter::repeat_n(black_box(url), black_box(10_000)).map(|url| black_box(Ok(black_box(url).into()))))
                },
                |job| black_box(job).for_each(|x| {black_box(x.expect("Ok").make());}),
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
                    lazy_task_configs: Box::new(std::iter::repeat_n(black_box(url), black_box(10_000)).map(|url| black_box(Ok(black_box(url).as_bytes().into()))))
                },
                |job| black_box(job).for_each(|x| {black_box(x.expect("Ok").make());}),
                criterion::BatchSize::SmallInput
            )
        );
    }
}

fn r#do(c: &mut Criterion) {
    for url in TASK_URLS {
        let task = Task {
            config: black_box(url).parse().unwrap(),
            job_context: &Default::default(),
            cleaner: Cleaner::get_default().unwrap(),
            #[cfg(feature = "cache")]
            cache: CacheHandle {
                cache: &Default::default(),
                config: Default::default()
            },
            unthreader: &Default::default()
        };

        c.bench_function(
            &format!("Do 10k Tasks: {url}"),
            |b| b.iter_batched(
                || std::iter::repeat_n(task.clone(), 10_000),
                |tasks| black_box(tasks).for_each(|x| {black_box(black_box(x).r#do());}),
                criterion::BatchSize::SmallInput
            )
        );
    }
}
