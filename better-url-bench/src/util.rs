//! Utilities.

use std::time::Duration;

/// Parse a [`Duration`]'s [`std::fmt::Debug`] output.
pub fn parse_duration(s: &str) -> Option<Duration> {
    let secs = match s {
        s if let Some(x) = s.strip_suffix("ns") => x.parse::<f64>().ok()? / 1_000_000_000f64,
        s if let Some(x) = s.strip_suffix("µs") => x.parse::<f64>().ok()? / 1_000_000f64,
        s if let Some(x) = s.strip_suffix("ms") => x.parse::<f64>().ok()? / 1_000f64,
        s if let Some(x) = s.strip_suffix( "s") => x.parse::<f64>().ok()?,
        _ => None?
    };

    Some(Duration::from_secs_f64(secs))
}
