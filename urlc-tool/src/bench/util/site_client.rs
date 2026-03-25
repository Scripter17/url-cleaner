//! Site CLIent.

use crate::prelude::*;

/// The protocol to use.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Protocol {
    /// HTTP.
    Http,
    /// HTTPS.
    Https,
    /// WebSocket.
    Ws,
    /// Secure WebSocket.
    Wss,
}

impl Protocol {
    /// If this protocol uses TLS.
    pub fn tls(self) -> bool {
        matches!(self, Self::Https | Self::Wss)
    }

    /// The endpoint to use.
    pub fn endpoint(self) -> &'static str {
        match self {
            Self::Http  => "http://127.0.0.1:9148",
            Self::Https => "https://127.0.0.1:9148",
            Self::Ws    => "ws://127.0.0.1:9148",
            Self::Wss   => "wss://127.0.0.1:9148",
        }
    }
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http  => write!(formatter, "http"),
            Self::Https => write!(formatter, "https"),
            Self::Ws    => write!(formatter, "ws"),
            Self::Wss   => write!(formatter, "wss"),
        }
    }
}
