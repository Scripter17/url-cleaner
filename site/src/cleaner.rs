//! The `/cleaner` endpoint.

use crate::*;

/// The `/cleaner` endpoint.
#[get("/cleaner")]
pub async fn cleaner<'a>(state: &'a State<&'static ServerState>) -> impl Responder<'a, 'a> {
    (
        ContentType(MediaType::JSON),
        &*state.config.cleaner_string
    )
}

