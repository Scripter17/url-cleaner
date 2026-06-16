//! `/userscript-bench`.

use crate::prelude::*;

const PRELUDE: &str = r#"<!DOCTYPE html>
<html>
  <head>
    <style>
      body {
        width: 100%;
        overflow-wrap: break-word;
      }
    </style>
    <script>
      window.onload = function() {
        let start = new Date();
        let remaining = document.body.children.length;

        new MutationObserver(function (mutations) {
          if ((remaining -= mutations.length) == 0) {
            document.body.innerHTML = `${new Date() - start}ms`;
          }
        }).observe(document.body, {
          attributeFilter: ["href"],
          subtree: true
        });
      }
    </script>
  </head>
  <body>
"#;

const EPILOGUE: &str = r#"  </body>
</html>"#;

/// A benchmark.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Benchmark {
    /// The [`BetterUrl`].
    #[serde(default = "default_url")]
    pub url: BetterUrl,
    /// The number of tasks.
    #[serde(default = "default_num")]
    pub num: u32
}

fn default_url() -> BetterUrl {"https://example.com/?utm_source=urlcs".parse().expect("To be valid")}
fn default_num() -> u32 {10_000}

/// `/userscript_bench`.
pub async fn userscript_bench(axum::extract::Query(Benchmark {url, num}): axum::extract::Query<Benchmark>) -> Response {
    Body::from_stream(stream!(
        let one = format!("    <a href=\"{url}\"></a>\n");

        let ten = Bytes::from(one.repeat(10));
        let one = Bytes::from(one);

        yield PRELUDE.into();

        for _ in 0..num / 10 {yield ten.clone();}
        for _ in 0..num % 10 {yield one.clone();}

        yield EPILOGUE.into();
    ).map(Ok::<_, std::convert::Infallible>)).into_response()
}
