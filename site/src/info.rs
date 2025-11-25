//! The `/info` endpoint.

use crate::*;

/// The `/info` endpoint.
#[get("/info")]
pub async fn info(state: &State<&'static ServerState>) -> Json<ServerInfo> {
    Json(ServerInfo {
        source_code: env!("CARGO_PKG_REPOSITORY").into(),
        version    : env!("CARGO_PKG_VERSION").into(),
        max_payload: state.config.max_payload.as_u64()
    })
}
