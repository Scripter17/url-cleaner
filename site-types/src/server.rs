//! Implementations for URL Cleaner Site specifically.

use rocket::{
    request::{self, Outcome, Request, FromRequest},
    response::{Responder, content::RawJson},
    http::Status,
    serde::json::Json
};
use base64::prelude::*;

use url_cleaner_engine::prelude::*;

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
        (Status {code: self.status}, RawJson(serde_json::to_string(&Err::<SmallCleanSuccess, _>(self)).expect("Serializing CleanError to never fail."))).respond_to(request)
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for CleanSuccess {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'o> {
        RawJson(serde_json::to_string(&Ok::<_, CleanError>(self)).expect("Serializing CleanSuccess to never fail.")).respond_to(request)
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for SmallCleanSuccess {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'o> {
        RawJson(serde_json::to_string(&Ok::<_, CleanError>(self)).expect("Serializing SmallCleanSuccess to never fail.")).respond_to(request)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CleanPayloadConfig {
    type Error = rocket::form::Errors<'r>;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        fn inner<'r>(req: &'r Request<'_>) -> Result<CleanPayloadConfig, rocket::form::Errors<'r>> {
            Ok(CleanPayloadConfig {
                context    : req.query_value::<Json<JobContext>>        ("context"    ).transpose()?.map(|x| x.0).unwrap_or_else(Default::default),
                profile    : req.query_value::<Json<Option<String>>>    ("profile"    ).transpose()?.map(|x| x.0).unwrap_or     (None),
                params_diff: req.query_value::<Json<Option<ParamsDiff>>>("params_diff").transpose()?.map(|x| x.0).unwrap_or     (None),
                read_cache : req.query_value::<Json<bool>>              ("read_cache" ).transpose()?.map(|x| x.0).unwrap_or     (true),
                write_cache: req.query_value::<Json<bool>>              ("write_cache").transpose()?.map(|x| x.0).unwrap_or     (true),
                cache_delay: req.query_value::<Json<bool>>              ("cache_delay").transpose()?.map(|x| x.0).unwrap_or     (false),
                unthread   : req.query_value::<Json<bool>>              ("unthread"   ).transpose()?.map(|x| x.0).unwrap_or     (false)
            })
        }
        match inner(req) {
            Ok(x) => Outcome::Success(x),
            Err(e) => Outcome::Error((Status::BadRequest, e))
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
