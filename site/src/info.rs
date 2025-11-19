//! The `/info` endpoint.

use crate::*;

/// The source code of this instance.
static SOURCE_CODE: LazyLock<BetterUrl> = LazyLock::new(|| env!("CARGO_PKG_REPOSITORY").parse().expect("The CARGO_PKG_REPOSITORY enviroment vairable to be a valid BetterUrl."));
/// The version of this instance.
const VERSION     : &str = env!("CARGO_PKG_VERSION");

/// The `/info` endpoint.
#[get("/info")]
pub async fn info(state: &State<ServerState>) -> Json<ServerInfo<'_>> {
    Json(ServerInfo {
        source_code         : Cow::Borrowed(&SOURCE_CODE),
        version             : Cow::Borrowed(VERSION),
        max_payload         : state.config.max_payload.as_u64(),
        unthreader_mode     : state.unthreader.mode
    })
}

