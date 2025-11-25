//! The `/profiles` endpoint`.

use crate::*;

/// The `/profiles` endpoint.
#[get("/profiles")]
pub async fn profiles<'a>(state: &'a State<&'static ServerState>) -> impl Responder<'a, 'a> {
    (
        ContentType(MediaType::JSON),
        &*state.config.profiles_config_string
    )
}

