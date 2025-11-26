//! Glue to make the provided types work with Rocket.

use rocket::{
    request::{Outcome, Request, FromRequest},
    response::{Responder, content::RawJson},
    http::Status,
    serde::json::Json
};

use crate::prelude::*;

impl From<Status> for CleanError {
    fn from(value: Status) -> Self {
        Self {
            status: value.code,
            message: value.to_string()
        }
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for CleanError {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'o> {
        (
            Status {code: self.status},
            RawJson(serde_json::to_string(&self).expect("Serializing CleanError to never fail."))
        ).respond_to(request)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CleanConfig {
    type Error = CleanError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match (req.query_value::<Json<Self>>("config"), req.headers().get_one("X-Config").map(serde_json::from_str)) {
            (None        , None        ) => Outcome::Success(Default::default()),

            (Some(Ok (x)), None        ) => Outcome::Success(x.0),
            (Some(Err(e)), None        ) => Outcome::Error((Status::BadRequest, CleanError {status: 400, message: format!("{e:?}")})),

            (None        , Some(Ok(x)) ) => Outcome::Success(x),
            (None        , Some(Err(e))) => Outcome::Error((Status::BadRequest, CleanError {status: 400, message: format!("{e:?}")})),

            (Some(_)     , Some(_)     ) => Outcome::Error((Status::BadRequest, CleanError {status: 400, message: "Can't specify a config in both query and headers.".into()}))
        }
    }
}
