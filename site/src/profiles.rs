//! The `/profiles` endpoint`.

use crate::*;

/// The `/profiles` endpoint.
#[get("/profiles")]
pub async fn profiles(state: &State<ServerState>) -> impl Responder<'_, '_> {
    (
        ContentType(MediaType::JSON),
        &*state.config.profiles_config_string
    )
}

