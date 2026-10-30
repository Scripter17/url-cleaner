//! GET URL Cleaner Site Userscript with the instance info pre-filled.

use super::*;

/// GET URL Cleaner Site Userscript with the instance info pre-filled.
pub async fn userscript(state: &'static State, request: Request<Body>) -> Result<String, &'static str> {
    let socket = request.headers()
        .get("host").ok_or("The request to have a Host header.")?
        .to_str()   .map_err(|_| "The Host header to be ASCII.")?;

    let host = match socket.rsplit_once(':') {
        Some((host, port)) if !port.contains(']') => host,
        _ => socket
    };

    let protocol = match state.tls {
        true  => "wss",
        false => "ws",
    };

    let (a, b) = crate::USERSCRIPT.split_once("localhost").expect("???");
    let (b, c) = b.split_once("ws://localhost:9149")      .expect("???");

    Ok(format!("{a}{host}{b}{protocol}://{socket}{c}"))
}
