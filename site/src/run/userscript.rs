//! GET URL Cleaner Site Userscript with the instance info pre-filled.

use super::*;

/// GET URL Cleaner Site Userscript with the instance info pre-filled.
pub async fn userscript(state: &'static State, request: Request<Body>) -> Result<String, &'static str> {
    let (host, port) = request.headers()
        .get("host")     .ok_or  (    "The request to have a host header.")?
        .to_str()        .map_err(|_| "The host header to be ASCII."      )?
        .rsplit_once(':').ok_or  (    "The host header to contain a port.")?;

    let protocol = match state.tls {
        true  => "wss",
        false => "ws",
    };

    let (a, b) = crate::USERSCRIPT.split_once("localhost").expect("???");
    let (b, c) = b.split_once("ws://localhost:9149").expect("???");

    Ok(format!("{a}{host}{b}{protocol}://{host}:{port}{c}"))
}
