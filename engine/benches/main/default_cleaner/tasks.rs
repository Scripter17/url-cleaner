use std::hint::black_box;
use criterion::Criterion;

use crate::*;

use url_cleaner_engine::types::*;

const TASK_URLS: [&'static str; 3] = [
    "https://x.com?a=2",
    "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id",
    "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8"
];

group!(tasks, r#do);

fn r#do(c: &mut Criterion) {
    for url in TASK_URLS {
        let task = Task {
            config: black_box(url).parse().unwrap(),
            job_context: &Default::default(),
            cleaner: Cleaner::get_default().unwrap(),
            #[cfg(feature = "cache")]
            cache: &Default::default()
        };

        c.bench_function(
            &format!("Task::r#do(): DC, {url}"),
            |b| b.iter_batched(
                || task.clone(),
                |task| task.r#do(),
                criterion::BatchSize::SmallInput
            )
        );
    }
}
