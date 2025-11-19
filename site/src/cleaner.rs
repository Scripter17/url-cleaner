//! The `/cleaner` endpoint.

use crate::*;

/// The `/cleaner` endpoint.
#[get("/cleaner")]
pub async fn cleaner(state: &State<ServerState>) -> impl Responder<'_, '_> {
    (
        ContentType(MediaType::JSON),
        &*state.config.cleaner_string
    )
}

