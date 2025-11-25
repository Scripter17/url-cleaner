//! Implementations for URL Cleaner Site specifically.

use rocket::{
    request::{self, Outcome, Request, FromRequest},
    response::{Responder, content::RawJson},
    http::Status,
    serde::json::Json
};
use base64::prelude::*;

use crate::*;

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
    type Error = rocket::form::Errors<'r>;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.query_value::<Json<Self>>("config") {
            None         => Outcome::Success(Default::default()),
            Some(Ok(x))  => Outcome::Success(x.0),
            Some(Err(e)) => Outcome::Error((Status::BadRequest, e))
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match req.headers().get_one("authorization") {
            None => Outcome::Success(Self::Guest),
            Some(x) => if let Some(b64) = x.strip_prefix("Basic ")
                && let Ok(bytes) = BASE64_STANDARD.decode(b64)
                && let Ok(string) = String::try_from(bytes)
                && let Some((username, password)) = string.split_once(':') {
                Outcome::Success(Self::User {
                    username: username.into(),
                    password: password.into()
                })
            } else {
                Outcome::Error((Status::BadRequest, ()))
            }
        }
    }
}
