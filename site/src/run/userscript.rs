//! GET URL Cleaner Site Userscript with the instance info pre-filled.

use super::*;

/// GET URL Cleaner Site Userscript with the instance info pre-filled.
pub async fn userscript(state: &'static State, request: Request<Body>) -> String {
    let (host, port) = request.headers()
        .get("host")     .expect("The request to have a host header.")
        .to_str()        .expect("The host header to be ASCII.")
        .rsplit_once(':').expect("The host header to contain a port.");

    let protocol = match state.tls {
        true  => "wss",
        false => "ws",
    };

    let (a, b) = crate::USERSCRIPT.split_once("localhost").expect("???");
    let (b, c) = b.split_once("ws://localhost:9149").expect("???");

    format!("{a}{host}{b}{protocol}://{host}:{port}{c}")
}
