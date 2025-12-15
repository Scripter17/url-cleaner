//! Site.

use super::prelude::*;

pub mod http;
pub mod websocket;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::super::prelude::*;

    pub use super::wait_for_server;
}

/// Wait for the server to start.
/// # Panics
/// If the server doesn't start after 10 seconds, panics.
pub fn wait_for_server() {
    for _ in 0..100 {
        if std::net::TcpStream::connect("127.0.0.1:9148").is_ok() {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    panic!("Server not found???")
}

/// Site.
#[allow(clippy::missing_docs_in_private_items, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
pub enum Args {
    #[command(subcommand)]
    Http(http::Args),
    #[command(subcommand)]
    Websocket(websocket::Args)
}

impl Args {
    /// Do the command.
    pub fn r#do(self) -> String {
        match self {
            Args::Http     (args) => args.r#do(),
            Args::Websocket(args) => args.r#do()
        }
    }
}
