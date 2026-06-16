//! [`Port`].

use crate::prelude::*;

/// A port.
#[derive(Debug, Clone)]
pub struct Port<'a> {
    /// The port as a string.
    port: Cow<'a, str>,
    /// The port as a [`u16`].
    port_num: u16
}

impl<'a> Port<'a> {
    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.port
    }

    /// Get it as a [`u16`].
    pub fn as_u16(&self) -> u16 {
        self.port_num
    }
}

impl<'a> TryFrom<Cow<'a, str>> for Port<'a> {
    type Error = InvalidPort;

    fn try_from(mut value: Cow<'a, str>) -> Result<Self, Self::Error> {
        while value != "0" && let Some(x) = value.strip_prefix('0') {
            value.retain_substr(x);
        }
        Ok(Self {
            port_num: parse_port(&value)?,
            port: value
        })
    }
}
